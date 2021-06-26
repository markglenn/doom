pub mod lump;

use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom};

use byteorder::ReadBytesExt;

use byteorder::LittleEndian as le;

#[derive(Debug)]
enum WadType {
    IWAD,
    PWAD,
}

#[derive(Debug)]
pub struct WAD {
    file: File,
    wad_type: WadType,
    directory: Vec<DirectoryLump>,
}

#[derive(Debug)]
struct DirectoryLump {
    position: u64,
    size: usize,
    name: String,
}

impl DirectoryLump {
    pub fn load(file: &mut File) -> Result<DirectoryLump> {
        let position = file.read_u32::<le>()?;
        let size = file.read_u32::<le>()?;
        let name = read_fixed_length_string(file, 8)?;

        Ok(DirectoryLump {
            position: position as u64,
            size: size as usize,
            name: name.to_string(),
        })
    }
}

impl WAD {
    pub fn load(filename: &str) -> Result<WAD> {
        let mut file = File::open(filename)?;

        let wad_type = wad_type(&mut file)?;
        let directory = load_directory(&mut file)?;

        Ok(WAD {
            file,
            wad_type,
            directory,
        })
    }

    pub fn load_level_lump(&mut self, map_name: &str, name: &str) -> Result<Vec<u8>> {
        let mut iter = self.directory.iter();
        iter.find(|&l| l.name == map_name).unwrap();

        if let Some(lump) = iter.find(|&l| l.name == name) {
            self.file.seek(SeekFrom::Start(lump.position))?;

            let mut buf = vec![0u8; lump.size];
            self.file.read_exact(&mut buf)?;

            Ok(buf)
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Level lump not found"))
        }
    }
}

fn wad_type(file: &mut File) -> Result<WadType> {
    match file.read_u32::<le>()? {
        0x44415749 => Ok(WadType::IWAD),
        0x44415750 => Ok(WadType::PWAD),
        _ => panic!("Invalid WAD type"),
    }
}

fn load_directory(file: &mut File) -> Result<Vec<DirectoryLump>> {
    let num_lumps = file.read_i32::<le>()?;
    let offset = file.read_u32::<le>()?;

    file.seek(SeekFrom::Start(offset.into()))?;

    let mut directory = vec![];
    for _ in 0..num_lumps {
        let lump = DirectoryLump::load(file)?;
        directory.push(lump);
    }

    Ok(directory)
}

pub fn read_fixed_length_string<S: Read>(stream: &mut S, size: usize) -> Result<String> {
    let mut buf = vec![0; size];
    stream.read_exact(&mut buf)?;

    let raw_name = match String::from_utf8(buf.to_vec()) {
        Ok(it) => it,
        _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid lump name")),
    };

    Ok(raw_name.trim_end_matches(|c| c == '\0').to_string())
}
