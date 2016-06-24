use std::io::{Read};
use byteorder::{LittleEndian as E, ReadBytesExt};
use gfx;
use gfx::format::I8Norm;
use render::ObjectVertex;


const SCALE: f32 = 1.0 / 256.0;

pub struct Mesh<R: gfx::Resources> {
    pub slice: gfx::Slice<R>,
    pub buffer: gfx::handle::Buffer<R, ObjectVertex>,
}

pub struct Wheel<R: gfx::Resources> {
    pub mesh: Option<Mesh<R>>,
    pub steering: u32,
    pub width: f32,
    pub radius: f32,
}

pub struct Debrie<R: gfx::Resources> {
    pub mesh: Mesh<R>,
}

pub struct Model<R: gfx::Resources> {
    pub body: Mesh<R>,
    pub color: [u32; 2],
    pub wheels: Vec<Wheel<R>>,
    pub debris: Vec<Debrie<R>>,
}

pub fn load_c3d<I, R, F>(source: &mut I, factory: &mut F) -> Mesh<R> where
    I: ReadBytesExt,
    R: gfx::Resources,
    F: gfx::traits::FactoryExt<R>,
{
    let version = source.read_u32::<E>().unwrap();
    assert_eq!(version, 8);
    let num_positions = source.read_u32::<E>().unwrap();
    let num_normals   = source.read_u32::<E>().unwrap();
    let num_polygons  = source.read_u32::<E>().unwrap();
    let _total_verts  = source.read_u32::<E>().unwrap();

    let coord_max  = [
        source.read_i32::<E>().unwrap() as f32 * SCALE,
        source.read_i32::<E>().unwrap() as f32 * SCALE,
        source.read_i32::<E>().unwrap() as f32 * SCALE,
    ];
    let coord_min  = [
        source.read_i32::<E>().unwrap() as f32 * SCALE,
        source.read_i32::<E>().unwrap() as f32 * SCALE,
        source.read_i32::<E>().unwrap() as f32 * SCALE,
    ];
    let _parent_off = [
        source.read_i32::<E>().unwrap(),
        source.read_i32::<E>().unwrap(),
        source.read_i32::<E>().unwrap(),
    ];
    let _max_radius = source.read_u32::<E>().unwrap();
    let _parent_rot = [source.read_u32::<E>().unwrap(), source.read_u32::<E>().unwrap(), source.read_u32::<E>().unwrap()];
    source.by_ref().bytes().nth(8 * (1 + 3 + 9)-1).unwrap().unwrap();

    info!("\tReading {} positions...", num_positions);
    let mut positions = Vec::with_capacity(num_positions as usize);
    for _ in 0 .. num_positions {
        for _ in 0..3 {
            source.read_u32::<E>().unwrap(); //unknown
        }
        let mut pos = [0u8; 3];
        source.read(&mut pos).unwrap();
        let _sort_info = source.read_u32::<E>().unwrap();
        positions.push(pos);
    }

    info!("\tReading {} normals...", num_normals);
    let mut normals = Vec::with_capacity(num_normals as usize);
    for _ in 0 .. num_normals {
        let mut norm = [0u8; 4];
        source.read(&mut norm).unwrap();
        let _sort_info = source.read_u32::<E>().unwrap();
        normals.push(norm);
    }

    info!("\tReading {} polygons...", num_polygons);
    let mut vertices = Vec::with_capacity(num_polygons as usize * 3);
    for i in 0 .. num_polygons {
        let num_corners = source.read_u32::<E>().unwrap();
        assert_eq!(num_corners, 3);
        let _sort_info = source.read_u32::<E>().unwrap();
        let color = [source.read_u32::<E>().unwrap(), source.read_u32::<E>().unwrap()];
        let mut dummy = [0; 4];
        source.read(&mut dummy[..4]).unwrap(); //skip flat normal
        source.read(&mut dummy[..3]).unwrap(); //skip middle point
        //if i == 631 { source.read_u8().unwrap(); } //wtf?
        for _ in 0..3 {
            let pid = source.read_u32::<E>().unwrap();
            let nid = source.read_u32::<E>().unwrap();
            let v = (positions[pid as usize], normals[nid as usize], color);
            vertices.push(v);
        }
    }

    info!("\tCompacting...");
    vertices.sort();
    vertices.dedup();

    let gpu_verts: Vec<_> = vertices.into_iter().map(|(p, n, c)| ObjectVertex {
        pos: [
            coord_min[0] + p[0] as f32 * (coord_max[0] - coord_min[0]) / 255.0,
            coord_min[1] + p[1] as f32 * (coord_max[1] - coord_min[1]) / 255.0,
            coord_min[2] + p[2] as f32 * (coord_max[2] - coord_min[2]) / 255.0,
            1.0],
        color: c,
        normal: [I8Norm(n[0] as i8), I8Norm(n[1] as i8), I8Norm(n[2] as i8), I8Norm(n[3] as i8)],
    }).collect();
    info!("\tGot {} GPU vertices...", gpu_verts.len());
    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&gpu_verts, ());

    Mesh {
        slice: slice,
        buffer: vbuf,
    }
}

pub fn load_m3d<I, R, F>(source: &mut I, factory: &mut F) -> Model<R> where
    I: ReadBytesExt,
    R: gfx::Resources,
    F: gfx::traits::FactoryExt<R>,
{
    info!("\tReading the body...");
    let mut model = Model {
        body: load_c3d(source, factory),
        color: [0, 0],
        wheels: Vec::new(),
        debris: Vec::new(),
    };
    let _bounds = [
        source.read_u32::<E>().unwrap(),
        source.read_u32::<E>().unwrap(),
        source.read_u32::<E>().unwrap(),
    ];
    let _max_radius = source.read_u32::<E>().unwrap();
    let num_wheels = source.read_u32::<E>().unwrap();
    let num_debris = source.read_u32::<E>().unwrap();
    model.color = [source.read_u32::<E>().unwrap(), source.read_u32::<E>().unwrap()];
    model.wheels.reserve_exact(num_wheels as usize);
    model.debris.reserve_exact(num_debris as usize);

    info!("\tReading wheels...");
    for _ in 0 .. num_wheels {
        let steering = source.read_u32::<E>().unwrap();
        source.by_ref().bytes().nth(3*8).unwrap().unwrap();
        let width = source.read_u32::<E>().unwrap() as f32 * SCALE;
        let radius = source.read_u32::<E>().unwrap() as f32 * SCALE;
        let _bound_index = source.read_u32::<E>().unwrap();
        model.wheels.push(Wheel {
            mesh: if steering != 0 {
                Some(load_c3d(source, factory))
            } else {None},
            steering: steering,
            width: width,
            radius: radius,
        })
    }

    info!("\tReading debris...");
    for _ in 0 .. num_debris {
        let mesh = load_c3d(source, factory);
        let _phys_bound = load_c3d(source, factory);
        model.debris.push(Debrie {
            mesh: mesh,
        })
    }

    //let _phys_bound = load_c3d(source, factory);
    model
}
