use ini::Ini;
use std::ops::Range;
use std::path::{Path, PathBuf};

pub struct Power(pub i32);
impl Power {
    pub fn as_value(&self) -> i32 {
        1 << self.0
    }
    pub fn as_power(&self) -> i32 {
        self.0
    }
}

#[derive(Clone)]
pub struct TerrainConfig {
    pub shadow_offset: u8,
    pub height_shift: u8,
    pub colors: Range<u8>, // note: actually, this is inclusive range
}

pub struct LevelConfigPath {
    pub palette: PathBuf,
    pub data: PathBuf
}

pub struct LevelConfig {
    pub path: Option<LevelConfigPath>,
    pub is_compressed: bool,
    pub size: (Power, Power),
    pub geo: Power,
    pub section: Power,
    pub min_square: Power,
    pub terrains: Box<[TerrainConfig]>,
}

impl LevelConfig {
    fn load_ini(ini: &Ini) -> Result<Self, ()> {
        let global = &ini["Global Parameters"];
        let storage = &ini["Storage"];
        let render = &ini["Rendering Parameters"];

        let terra_count = render
            .get("Terrain Max")
            .map_or(8, |value| value.parse::<usize>().unwrap());

        let mut terrains = (0..terra_count)
            .map(|_| TerrainConfig {
                shadow_offset: 0,
                height_shift: 0,
                colors: 0..0,
            })
            .collect::<Box<[_]>>();

        for (t, val) in terrains
            .iter_mut()
            .zip(render["Shadow Offsets"].split_whitespace())
        {
            t.shadow_offset = val.parse().unwrap();
        }
        for (t, val) in terrains
            .iter_mut()
            .zip(render["Height Shifts"].split_whitespace())
        {
            t.height_shift = val.parse().unwrap();
        }
        for (t, val) in terrains
            .iter_mut()
            .zip(render["Begin Colors"].split_whitespace())
        {
            t.colors.start = val.parse().unwrap();
        }
        for (t, val) in terrains
            .iter_mut()
            .zip(render["End Colors"].split_whitespace())
        {
            t.colors.end = val.parse().unwrap();
        }

        Ok(LevelConfig {
            path: None,
            is_compressed: &storage["Compressed Format Using"] != "0",
            size: (
                Power(global["Map Power X"].parse().unwrap()),
                Power(global["Map Power Y"].parse().unwrap()),
            ),
            geo: Power(global["GeoNet Power"].parse().unwrap()),
            section: Power(global["Section Size Power"].parse().unwrap()),
            min_square: Power(global["Minimal Square Power"].parse().unwrap()),
            terrains,
        })
    }

    pub fn load_str(str: &str) -> Self {
        let ini = Ini::load_from_str(str).unwrap_or_else(|_| {
            panic!("Unable to read the level's INI description")
        });

        Self::load_ini(&ini).unwrap()
    }

    pub fn load_path(ini_path: &Path) -> Self {
        let ini = Ini::load_from_file(ini_path).unwrap_or_else(|_| {
            panic!("Unable to read the level's INI description: {:?}", ini_path)
        });

        let mut levelcfg = Self::load_ini(&ini).unwrap();

        levelcfg.path = Some(LevelConfigPath {
            data: ini_path.with_file_name(&ini["Storage"]["File Name"]),
            palette: ini_path.with_file_name(&ini["Storage"]["Palette File"])
        });

        levelcfg
    }
}
