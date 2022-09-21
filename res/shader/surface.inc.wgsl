// Common routines for fetching the level surface data.

struct SurfaceConstants {
    texture_scale: vec4<f32>;    // XY = size, Z = height scale, w = number of layers
    terrain_bits: vec4<u32>;     // X_low = shift, X_high = mask
};

struct Surface {
    low_alt: f32;
    high_alt: f32;
    delta: f32;
    low_type: u32;
    high_type: u32;
    height_diff: f32;
    is_shadowed: bool;
};

struct StorageIndex {
    index: u32;
    mask: u32;
    shift: u32;
};

struct TerrainData {
    heightAndMeta: array<u32>;
};

struct TerrainConstants {
    meta_offset: u32;
    stride: i32;
};

@group(1) @binding(0) var<uniform> u_Surface: SurfaceConstants;
@group(1) @binding(10) var<storage> t_Data: TerrainData;
@group(1) @binding(11) var<uniform> t_Const: TerrainConstants;

let c_DoubleLevelMask: u32 = 64u;
let c_ShadowMask: u32 = 128u;
let c_DeltaShift: u32 = 0u;
let c_DeltaBits: u32 = 2u;
let c_DeltaScale: f32 = 0.03137254901; //8.0 / 255.0;

fn get_terrain_type(meta: u32) -> u32 {
    let bits = u_Surface.terrain_bits.x;
    return (meta >> (bits & 0xFu)) & (bits >> 4u);
}
fn get_delta(meta: u32) -> u32 {
    return (meta >> c_DeltaShift) & ((1u << c_DeltaBits) - 1u);
}

fn modulo(a: i32, b: i32) -> i32 {
    let c = a % b;
    return select(c, c+b, c < 0);
}

fn get_storage_index(ipos: vec2<i32>) -> vec3<u32> {
    // let x = modulo(ipos.x, i32(u_Surface.texture_scale.x));
    // let y = modulo(ipos.y, i32(u_Surface.texture_scale.y));
    let x = ipos.x;
    let y = ipos.y;
    let index = u32(y * t_Const.stride + x);
    let shift = 8u * (index % 4u);

    return vec3<u32>(
        index / 4u,     // index
        0xFFu << shift, // mask
        shift           // shift
    );
}

fn get_storage_meta(si: vec3<u32>) -> u32 {
    return (t_Data.heightAndMeta[si.x + t_Const.meta_offset] & si.y) >> si.z;
}

fn get_storage_height(si: vec3<u32>) -> f32 {
    return f32((t_Data.heightAndMeta[si.x] & si.y) >> si.z) / 256.;
}

fn get_lod_height(ipos: vec2<i32>, lod: u32) -> f32 {
    let alt = get_storage_height(get_storage_index(ipos));
    return alt * u_Surface.texture_scale.z;
}

fn get_map_coordinates(pos: vec2<f32>) -> vec2<i32> {
    return vec2<i32>(pos - floor(pos / u_Surface.texture_scale.xy) * u_Surface.texture_scale.xy);
}

fn get_surface(pos: vec2<f32>) -> Surface {
    var suf: Surface;
    let tci = get_map_coordinates(pos);
    let stor_index = get_storage_index(tci);

    let meta = get_storage_meta(stor_index);
    suf.is_shadowed = (meta & c_ShadowMask) != 0u;
    suf.low_type = get_terrain_type(meta);

    if ((meta & c_DoubleLevelMask) != 0u) {
        //TODO: we need either low or high for the most part
        // so this can be more efficient with a boolean param
        if (tci.x % 2 == 1) {
            let stor_index__1 = get_storage_index(tci + vec2<i32>(-1, 0));
            let meta_low = get_storage_meta(stor_index__1);
            let height = get_storage_height(stor_index);
            suf.high_type = suf.low_type;
            suf.low_type = get_terrain_type(meta_low);
            let delta = (get_delta(meta_low) << c_DeltaBits) + get_delta(meta);
            suf.low_alt = get_storage_height(stor_index__1) * u_Surface.texture_scale.z;
            suf.high_alt = height * u_Surface.texture_scale.z;
            suf.delta = f32(delta) * c_DeltaScale * u_Surface.texture_scale.z;
            suf.height_diff = height - get_storage_height(get_storage_index(tci + vec2<i32>(-2, 0)));
            return suf;
        } else {
            let stor_index_1 = get_storage_index(tci + vec2<i32>(1, 0));
            let meta_high = get_storage_meta(stor_index_1);
            let height = get_storage_height(stor_index_1);
            suf.high_type = get_terrain_type(meta_high);
            let delta = (get_delta(meta) << c_DeltaBits) + get_delta(meta_high);
            suf.low_alt = get_storage_height(stor_index) * u_Surface.texture_scale.z;
            suf.high_alt = height * u_Surface.texture_scale.z;
            suf.delta = f32(delta) * c_DeltaScale * u_Surface.texture_scale.z;
            suf.height_diff = height - get_storage_height(get_storage_index(tci + vec2<i32>(-1, 0)));
            return suf;
        }
    } else {
        let height = get_storage_height(stor_index);
        suf.high_type = suf.low_type;
        suf.low_alt = height * u_Surface.texture_scale.z;
        suf.high_alt = suf.low_alt;
        suf.delta = 0.0;
        suf.height_diff = height - get_storage_height(get_storage_index(tci + vec2<i32>(-2, 0)));
        return suf;
    }
}
