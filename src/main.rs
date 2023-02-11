use std::{fs, io::Write};

use data_line::{data_line, DataLine};

use flate2::{write::ZlibEncoder, Compression};
use nom::{
    bytes::complete::{tag, take_until},
    multi::many1,
    IResult,
};

mod chunk;
mod data_line;
mod element;

fn main() {
    let file = fs::read_to_string("test.step")
        .unwrap()
        .replace('\n', "")
        .replace('\r', "");

    let (remaining, parsed) = express_file(&file).unwrap();

    if !remaining.is_empty() {
        panic!("Got to data ref #{}", parsed.last().unwrap().number)
    }

    // Encode to json
    let json = serde_json::to_string(&parsed).unwrap();
    fs::write("test.json", json).unwrap();

    // Encode to sgcad
    let encoded: Vec<u8> = bincode::serialize(&parsed).unwrap();
    let mut zlib_encoded = ZlibEncoder::new(Vec::new(), Compression::default());
    zlib_encoded.write_all(&encoded).unwrap();
    fs::write("test.sgcad", zlib_encoded.finish().unwrap()).unwrap();
}

fn express_file(input: &str) -> IResult<&str, Vec<DataLine>> {
    let (data, _preamble) = take_until("DATA;")(input)?;
    let (data, _) = tag("DATA;")(data)?;

    let (remaining, data_lines) = many1(data_line)(data)?;

    let (_, _) = tag("ENDSEC;")(remaining)?;

    Ok(("", data_lines))
}
