mod wad;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate num_derive;

fn main() {
    let mut wad = wad::WAD::load("DOOM.WAD").unwrap();

    let _things = wad::lump::load_things(&mut wad, "E1M1").unwrap();
    let vertexes = wad::lump::load_vertexes(&mut wad, "E1M1").unwrap();
    let sectors = wad::lump::load_sectors(&mut wad, "E1M1").unwrap();

    println!("{:?}", sectors);
}
