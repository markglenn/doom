use std::io;

use self::{sector::Sector, thing::Thing, vertex::Vertex};

use super::WAD;

pub mod sector;
pub mod thing;
pub mod vertex;

pub fn load_things(wad: &mut WAD, map_name: &str) -> io::Result<Vec<Thing>> {
    let buffer = wad.load_level_lump(map_name, "THINGS").unwrap();

    Ok(Thing::load(buffer)?)
}

pub fn load_vertexes(wad: &mut WAD, map_name: &str) -> io::Result<Vec<Vertex>> {
    let buffer = wad.load_level_lump(map_name, "VERTEXES").unwrap();

    Ok(Vertex::load(buffer)?)
}

pub fn load_sectors(wad: &mut WAD, map_name: &str) -> io::Result<Vec<Sector>> {
    let buffer = wad.load_level_lump(map_name, "SECTORS").unwrap();

    Ok(Sector::load(buffer)?)
}
