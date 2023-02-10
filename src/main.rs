use core::num;
use std::fs;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_until, take_while},
    character::{
        complete::{char, digit1},
        is_digit,
    },
    multi::{many1, separated_list0},
    sequence::delimited,
    IResult,
};

fn main() {
    let file = fs::read_to_string("2.step")
        .unwrap()
        .replace('\n', "")
        .replace('\r', "");

    let (_, parsed) = express_file(&file).unwrap();

    println!("{:?}", parsed[0]);

    // println!("{:?}", parsed);
}

fn express_file(input: &str) -> IResult<&str, Vec<DataLine>> {
    let (data, preamble) = take_until("DATA;")(input)?;
    let (data, _) = take(5usize)(data)?;

    let (remaining, data) = many1(data_line)(data)?;

    // let mut chunks = data.split(";");
    // chunks.next();

    // chunks.map(|line| )

    Ok((remaining, data))
}

#[derive(Debug)]
struct DataLine<'a> {
    number: u32,
    tag: &'a str,
    val: Vec<Element>,
}

fn data_line(input: &str) -> IResult<&str, DataLine> {
    let (remaining, line) = take_until(";")(input)?;
    let (next_line, _) = take(1usize)(remaining)?;

    let (line, _) = char('#')(line)?;
    let (line, number) = take_until("=")(line)?;
    let (line, _) = char('=')(line)?;
    let (line, line_tag) = take_until("(")(line)?;

    let (_, val) = delimited_elements(line)?;

    let data_line: DataLine = DataLine {
        number: number.parse::<u32>().unwrap(),
        tag: line_tag,
        val,
    };

    Ok((next_line, data_line))
}

#[derive(Debug)]
enum Element {
    Reference(u32),
    Dollar,
    Elements(Vec<Element>),
}

fn delimited_elements(input: &str) -> IResult<&str, Vec<Element>> {
    delimited(char('('), separated_list0(tag(","), element), char(')'))(input)
}

fn element(input: &str) -> IResult<&str, Element> {
    alt((reference, dollar, nested_elements))(input)
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
