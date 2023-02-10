use nom::{
    bytes::complete::{take, take_until},
    character::complete::char,
    IResult,
};

use crate::chunk::{to_chunks, Chunk};

#[derive(Debug)]
pub struct DataLine<'a> {
    number: u32,
    chunks: Vec<Chunk<'a>>,
}

pub fn data_line(input: &str) -> IResult<&str, DataLine> {
    let (remaining, line) = take_until(";")(input)?;
    let (next_line, _) = take(1usize)(remaining)?;

    let (line, _) = char('#')(line)?;
    let (line, number) = take_until("=")(line)?;
    let (line, _) = char('=')(line)?;
    let (_, chunks) = to_chunks(line)?;

    let data_line: DataLine = DataLine {
        number: number.parse::<u32>().unwrap(),
        chunks,
    };

    Ok((next_line, data_line))
}
