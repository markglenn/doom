use byteorder::LittleEndian as le;
use byteorder::ReadBytesExt;
use num_traits::FromPrimitive;
use std::io;
use std::io::Cursor;

use crate::wad::read_fixed_length_string;

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum SectorType {
    Normal = 0,
    BlinkRandom = 1,
    Blink0_5Second = 2,
    Blink1_0Second = 3,
    BlinkPlusDamage20 = 4,
    Damage10 = 5,
    Damage5 = 7,
    OscillateLight = 8,
    Secret = 9,
    CloseAfter30 = 10,
    End = 11,
    BlinkSynchronized0_5 = 12,
    BlinkSynchronized1_0 = 13,
    OpenAfter300 = 14,
    Damage20 = 16,
    FlickerRandom = 17,
}

#[derive(Debug)]
pub struct Sector {
    floor_height: i16,
    ceiling_height: i16,
    floor_texture: String,
    ceiling_texture: String,
    light_level: i16,
    sector_type: u16,
    tag: i16,
}

impl Sector {
    pub fn load(buffer: Vec<u8>) -> io::Result<Vec<Sector>> {
        let num_sectors = buffer.len() / 26;
        let mut stream = Cursor::new(buffer);

        let mut sectors = Vec::with_capacity(num_sectors);

        for _ in 0..num_sectors {
            let floor_height = stream.read_i16::<le>()?;
            let ceiling_height = stream.read_i16::<le>()?;

            let floor_texture = read_fixed_length_string(&mut stream, 8)?;
            let ceiling_texture = read_fixed_length_string(&mut stream, 8)?;

            let light_level = stream.read_i16::<le>()?;
            let sector_type = stream.read_u16::<le>()?;
            let tag = stream.read_i16::<le>()?;

            sectors.push(Sector {
                floor_height,
                ceiling_height,
                floor_texture,
                ceiling_texture,
                light_level,
                sector_type: FromPrimitive::from_u16(sector_type).unwrap(),
                tag,
            })
        }

        Ok(sectors)
    }
}
