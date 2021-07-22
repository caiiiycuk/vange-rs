use wasm_bindgen::prelude::*;

mod layers;
mod level_png;
mod model_obj;

use std::{
    fs::{read as fs_read, File},
    io::BufWriter,
    path::PathBuf,
};

pub fn save_tiff(path: &PathBuf, layers: layers::LevelLayers) {
    let images = [
        tiff::Image {
            width: layers.size.0 as u32,
            height: layers.size.1 as u32,
            bpp: 8,
            name: "h0",
            data: &layers.het0,
        },
        tiff::Image {
            width: layers.size.0 as u32,
            height: layers.size.1 as u32,
            bpp: 8,
            name: "h1",
            data: &layers.het1,
        },
        tiff::Image {
            width: layers.size.0 as u32,
            height: layers.size.1 as u32,
            bpp: 8,
            name: "del",
            data: &layers.delta,
        },
        tiff::Image {
            width: layers.size.0 as u32,
            height: layers.size.1 as u32,
            bpp: 4,
            name: "m0",
            data: &layers.mat0,
        },
        tiff::Image {
            width: layers.size.0 as u32,
            height: layers.size.1 as u32,
            bpp: 4,
            name: "m1",
            data: &layers.mat1,
        },
    ];

    let file = BufWriter::new(File::create(path).unwrap());
    tiff::save(file, &images).unwrap();
}

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn ron2vmp(ron_multi_png: Vec<u8>, height_png: Vec<u8>, material_hi_png: Vec<u8>, material_lo_png: Vec<u8>) -> Vec<u8> {
    log("\tLoading multiple PNGs...");
    let layers = level_png::load(&ron_multi_png, &height_png, &material_hi_png, &material_lo_png);
    log("\tSaving VMP...");
    return layers.export().save_vmp();
}

static mut s_ron_multi_png: Vec<u8> = Vec::new();
static mut s_height_png: Vec<u8> = Vec::new();
static mut s_material_hi_png: Vec<u8> = Vec::new();
static mut s_material_lo_png: Vec<u8> = Vec::new();

#[wasm_bindgen]
pub fn vmp2ron(ini: &str, vmp: Vec<u8>, vpr: Vec<u8>, palette: Vec<u8>) {
    println!("\tLoading the level...");
    let config = vangers::level::LevelConfig::load_str(ini);
    let level = vangers::level::load_vec(&config, &vmp, &vpr, &palette);
    let palette = layers::extract_palette(&level);
    let layers = layers::LevelLayers::from_level_data(
        &vangers::level::LevelData::from(level),
        config.terrains.len() as u8,
    );
    println!("\tSaving multiple PNGs...");
    let (ron, height, hi, lo) = level_png::save(layers, &palette);
    unsafe {
        s_ron_multi_png = ron.as_bytes().to_vec();
        s_height_png = height;
        s_material_hi_png = hi;
        s_material_lo_png = lo;
    }
}

#[wasm_bindgen]
pub fn get_ron_multi_png() -> Vec<u8> {
    let mut copy: Vec<u8> = Vec::new();
    unsafe {
        copy.append(&mut s_ron_multi_png);
    }
    return copy;
}

#[wasm_bindgen]
pub fn get_height_png() -> Vec<u8> {
    let mut copy: Vec<u8> = Vec::new();
    unsafe {
        copy.append(&mut s_height_png);
    }
    return copy;
}

#[wasm_bindgen]
pub fn get_material_hi_png() -> Vec<u8> {
    let mut copy: Vec<u8> = Vec::new();
    unsafe {
        copy.append(&mut s_material_hi_png);
    }
    return copy;
}

#[wasm_bindgen]
pub fn get_material_lo_png() -> Vec<u8> {
    let mut copy: Vec<u8> = Vec::new();
    unsafe {
        copy.append(&mut s_material_lo_png);
    }
    return copy;
}

fn main() {
    log("vange-rs convert started...");
}

/*
fn native_main() {
    use std::env;
    use std::io::Write;

    let args: Vec<_> = env::args().collect();
    let mut options = getopts::Options::new();
    options
        .parsing_style(getopts::ParsingStyle::StopAtFirstFree)
        .optflag("h", "help", "print this help menu");

    let matches = options.parse(&args[1..]).unwrap();
    if matches.opt_present("h") || matches.free.len() != 2 {
        println!("Vangers resource converter");
        let brief = format!("Usage: {} [options] <input> <output>", args[0]);
        println!("{}", options.usage(&brief));
        return;
    }

    let src_path = PathBuf::from(matches.free[0].as_str());
    let dst_path = PathBuf::from(matches.free[1].as_str());

    match (
        src_path
            .extension()
            .and_then(|ostr| ostr.to_str())
            .unwrap_or(""),
        dst_path
            .extension()
            .and_then(|ostr| ostr.to_str())
            .unwrap_or(""),
    ) {
        ("m3d", "ron") => {
            let file = File::open(&src_path).unwrap();
            println!("\tLoading M3D...");
            let raw = m3d::FullModel::load(file);
            println!("\tExporting OBJ data...");
            model_obj::export_m3d(raw, &dst_path);
        }
        ("ron", "md3") => {
            println!("\tImporting OBJ data...");
            let model = model_obj::import_m3d(&src_path);
            println!("\tSaving M3D...");
            model.save(File::create(&dst_path).unwrap());
        }
        ("a3d", "ron") => {
            let file = File::open(&src_path).unwrap();
            println!("\tLoading A3D...");
            let raw = m3d::AnimatedMesh::load(file);
            println!("\tExporting OBJ data...");
            model_obj::export_a3d(raw, &dst_path);
        }
        ("ron", "a3d") => {
            println!("\tImporting OBJ data...");
            let amesh = model_obj::import_a3d(&src_path);
            println!("\tSaving A3D...");
            amesh.save(File::create(&dst_path).unwrap());
        }
        ("ini", "ron") => {
            println!("\tLoading the level...");
            let config = vangers::level::LevelConfig::load(&src_path);
            let level = vangers::level::load(&config);
            let palette = layers::extract_palette(&level);
            let layers = layers::LevelLayers::from_level_data(
                &vangers::level::LevelData::from(level),
                config.terrains.len() as u8,
            );
            println!("\tSaving multiple PNGs...");
            level_png::save(&dst_path, layers, &palette);
        }
        ("ini", "tiff") => {
            println!("\tLoading the level...");
            let config = vangers::level::LevelConfig::load(&src_path);
            let level = vangers::level::load(&config);
            let layers = layers::LevelLayers::from_level_data(
                &vangers::level::LevelData::from(level),
                config.terrains.len() as u8,
            );
            println!("\tSaving TIFF layers...");
            save_tiff(&dst_path, layers);
        }
        ("ini", "vmp") => {
            println!("\tLoading the VMC...");
            let config = vangers::level::LevelConfig::load(&src_path);
            let level = vangers::level::load(&config);
            println!("\tSaving VMP...");
            vangers::level::LevelData::from(level).save_vmp(&dst_path);
        }
        ("ron", "vmp") => {
            println!("\tLoading multiple PNGs...");
            let layers = level_png::load(&src_path);
            println!("\tSaving VMP...");
            let level_data = layers.export();
            level_data.save_vmp(&dst_path);
        }
        ("pal", "png") => {
            println!("Converting palette to PNG...");
            let data = fs_read(&src_path).unwrap();
            let file = File::create(&dst_path).unwrap();
            let mut encoder = png::Encoder::new(file, 0x100, 1);
            encoder.set_color(png::ColorType::RGB);
            encoder
                .write_header()
                .unwrap()
                .write_image_data(&data)
                .unwrap();
        }
        ("png", "pal") => {
            println!("Converting PNG to palette...");
            let file = File::open(&src_path).unwrap();
            let decoder = png::Decoder::new(file);
            let (info, mut reader) = decoder.read_info().unwrap();
            assert_eq!((info.width, info.height), (0x100, 1));
            let stride = match info.color_type {
                png::ColorType::RGB => 3,
                png::ColorType::RGBA => 4,
                _ => panic!("non-RGB image provided"),
            };
            let mut data = vec![0u8; stride * 0x100];
            assert_eq!(info.bit_depth, png::BitDepth::Eight);
            assert_eq!(info.buffer_size(), data.len());
            reader.next_frame(&mut data).unwrap();
            let mut output = File::create(&dst_path).unwrap();
            for chunk in data.chunks(stride) {
                output.write(&chunk[..3]).unwrap();
            }
        }
        (in_ext, out_ext) => {
            panic!("Don't know how to convert {} to {}", in_ext, out_ext);
        }
    }
}
*/