use nom::{
    branch::alt, bytes::complete::take_until, character::complete::char, multi::many0,
    sequence::delimited, IResult,
};

use crate::element::{delimited_elements, Element};

#[derive(Debug)]
pub struct Chunk<'a> {
    pub tag: &'a str,
    pub elements: Vec<Element<'a>>,
}

pub fn to_chunks(input: &str) -> IResult<&str, Vec<Chunk>> {
    alt((delimited_chunks, single_chunk_as_vec))(input)
}

pub fn single_chunk_as_vec(input: &str) -> IResult<&str, Vec<Chunk>> {
    let (remaining, chunk) = single_chunk(input)?;
    Ok((remaining, vec![chunk]))
}

pub fn single_chunk(input: &str) -> IResult<&str, Chunk> {
    let (line, tag_value) = take_until("(")(input.trim())?;
    let (line, elements) = delimited_elements(line)?;

    Ok((
        line,
        Chunk {
            tag: tag_value,
            elements,
        },
    ))
}

pub fn delimited_chunks(input: &str) -> IResult<&str, Vec<Chunk>> {
    let (remaining, chunks) = delimited(char('('), many0(single_chunk), char(')'))(input)?;
    Ok((remaining, chunks))
}
