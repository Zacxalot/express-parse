


use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{char, digit1},
    multi::{separated_list0},
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
pub enum Element<'a> {
    Reference(u32),
    Dollar,
    Elements(Vec<Element<'a>>),
    String(&'a str),
    DotString(&'a str),
}

pub fn delimited_elements(input: &str) -> IResult<&str, Vec<Element>> {
    delimited(char('('), separated_list0(tag(","), element), char(')'))(input)
}

pub fn element(input: &str) -> IResult<&str, Element> {
    alt((reference, dollar, nested_elements, string, dot_string))(input)
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
        Element::Reference(number.parse::<u32>().unwrap()),
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
