use byteorder::LittleEndian as le;
use byteorder::ReadBytesExt;
use std::io;
use std::io::Cursor;

#[derive(Debug)]
pub struct Vertex {
    x_position: i16,
    y_position: i16,
}

impl Vertex {
    pub fn load(buffer: Vec<u8>) -> io::Result<Vec<Vertex>> {
        let num_vertexes = buffer.len() / 4;
        let mut stream = Cursor::new(buffer);

        let mut verts = Vec::with_capacity(num_vertexes);

        for _ in 0..num_vertexes {
            let x_position = stream.read_i16::<le>()?;
            let y_position = stream.read_i16::<le>()?;

            verts.push(Vertex {
                x_position,
                y_position,
            })
        }

        Ok(verts)
    }
}
