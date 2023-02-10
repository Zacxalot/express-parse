use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{char, digit1},
    combinator::{map_res, opt},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

use crate::chunk::{single_chunk, Chunk};

#[derive(Debug)]
pub enum Element<'a> {
    Reference(u64),
    Dollar,
    Asterisk,
    Elements(Vec<Element<'a>>),
    String(&'a str),
    DotString(&'a str),
    Float(f64),
    Integer(i64),
    Chunk(Chunk<'a>),
}

pub fn delimited_elements(input: &str) -> IResult<&str, Vec<Element>> {
    delimited(char('('), separated_list0(tag(","), element), char(')'))(input)
}

pub fn element(input: &str) -> IResult<&str, Element> {
    alt((
        reference,
        dollar,
        asterisk,
        nested_elements,
        string,
        dot_string,
        float,
        integer,
        chunk,
    ))(input.trim())
}

pub fn nested_elements(input: &str) -> IResult<&str, Element> {
    let (remaining, elements) = delimited_elements(input)?;

    Ok((remaining, Element::Elements(elements)))
}

pub fn reference(input: &str) -> IResult<&str, Element> {
    let (line, _) = char('#')(input)?;
    let (remaining, number) = digit1(line)?;

    Ok((
        remaining,
        Element::Reference(number.parse::<u64>().unwrap()),
    ))
}

pub fn dollar(input: &str) -> IResult<&str, Element> {
    let (remaining, _) = char('$')(input)?;
    Ok((remaining, Element::Dollar))
}

pub fn string(input: &str) -> IResult<&str, Element> {
    let (remaining, string_value) = delimited(tag("'"), take_until("'"), tag("'"))(input)?;
    Ok((remaining, Element::String(string_value)))
}

pub fn dot_string(input: &str) -> IResult<&str, Element> {
    let (remaining, string_value) = delimited(tag("."), take_until("."), tag("."))(input)?;
    Ok((remaining, Element::DotString(string_value)))
}

pub fn float(input: &str) -> IResult<&str, Element> {
    let (remaining, float_val) =
        map_res(take_till(|x| x == ')' || x == ','), |float_str: &str| {
            float_str.parse::<f64>()
        })(input)?;

    Ok((remaining, Element::Float(float_val)))
}

pub fn integer(input: &str) -> IResult<&str, Element> {
    let (line, negative) = opt(char('-'))(input)?;
    let (remaining, number) = digit1(line)?;

    let sign = negative.unwrap_or(' ');

    Ok((
        remaining,
        Element::Integer(format!("{}{}", sign, number).trim().parse::<i64>().unwrap()),
    ))
}

pub fn asterisk(input: &str) -> IResult<&str, Element> {
    let (remaining, _) = char('*')(input)?;
    Ok((remaining, Element::Asterisk))
}

pub fn chunk(input: &str) -> IResult<&str, Element> {
    let (remaining, chunk) = single_chunk(input)?;
    Ok((remaining, Element::Chunk(chunk)))
}
