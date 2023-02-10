use core::num;
use std::fs;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::{char, digit1},
    multi::{many0, many1, separated_list0},
    sequence::delimited,
    IResult,
};

fn main() {
    let file = fs::read_to_string("1.step")
        .unwrap()
        .replace('\n', "")
        .replace('\r', "");

    let (_, parsed) = express_file(&file).unwrap();

    for line in parsed {
        println!("{:?}", line)
    }

    // println!("{:?}", parsed);
}

fn express_file(input: &str) -> IResult<&str, Vec<DataLine>> {
    let (data, _preamble) = take_until("DATA;")(input)?;
    let (data, _) = take(5usize)(data)?;

    let (remaining, data) = many1(data_line)(data)?;

    Ok((remaining, data))
}

#[derive(Debug)]
struct DataLine<'a> {
    number: u32,
    chunks: Vec<Chunk<'a>>,
}

fn data_line(input: &str) -> IResult<&str, DataLine> {
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

fn to_chunks(input: &str) -> IResult<&str, Vec<Chunk>> {
    alt((delimited_chunks, single_chunk_as_vec))(input)
}

fn single_chunk_as_vec(input: &str) -> IResult<&str, Vec<Chunk>> {
    let (remaining, chunk) = single_chunk(input)?;
    Ok((remaining, vec![chunk]))
}

fn single_chunk(input: &str) -> IResult<&str, Chunk> {
    let (line, tag_value) = take_until("(")(input)?;
    let (line, elements) = delimited_elements(line)?;

    Ok((
        line,
        Chunk {
            tag: tag_value,
            elements,
        },
    ))
}

fn delimited_chunks(input: &str) -> IResult<&str, Vec<Chunk>> {
    let (remaining, chunks) = delimited(char('('), many0(single_chunk), char(')'))(input)?;
    Ok((remaining, chunks))
}

#[derive(Debug)]
struct Chunk<'a> {
    tag: &'a str,
    elements: Vec<Element<'a>>,
}

#[derive(Debug)]
enum Element<'a> {
    Reference(u32),
    Dollar,
    Elements(Vec<Element<'a>>),
    String(&'a str),
    DotString(&'a str),
}

fn delimited_elements(input: &str) -> IResult<&str, Vec<Element>> {
    delimited(char('('), separated_list0(tag(","), element), char(')'))(input)
}

fn element(input: &str) -> IResult<&str, Element> {
    alt((reference, dollar, nested_elements, string, dot_string))(input)
}

fn nested_elements(input: &str) -> IResult<&str, Element> {
    let (remaining, elements) = delimited_elements(input)?;

    Ok((remaining, Element::Elements(elements)))
}

fn reference(input: &str) -> IResult<&str, Element> {
    let (line, _) = char('#')(input)?;
    let (remaining, number) = digit1(line)?;

    Ok((
        remaining,
        Element::Reference(number.parse::<u32>().unwrap()),
    ))
}

fn dollar(input: &str) -> IResult<&str, Element> {
    let (remaining, _) = char('$')(input)?;
    Ok((remaining, Element::Dollar))
}

fn string(input: &str) -> IResult<&str, Element> {
    let (remaining, string_value) = delimited(tag("'"), take_until("'"), tag("'"))(input)?;
    Ok((remaining, Element::String(string_value)))
}

fn dot_string(input: &str) -> IResult<&str, Element> {
    let (remaining, string_value) = delimited(tag("."), take_until("."), tag("."))(input)?;
    Ok((remaining, Element::DotString(string_value)))
}
