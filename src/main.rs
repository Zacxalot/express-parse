use std::fs;

use data_line::{data_line, DataLine};

use nom::{
    bytes::complete::{take, take_until},
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

    println!("{:?}", parsed.last().unwrap());

    if !remaining.is_empty() {
        panic!("Got to data ref #{}", parsed.last().unwrap().number)
    }

    for line in parsed {
        println!("{:?}", line)
    }
}

fn express_file(input: &str) -> IResult<&str, Vec<DataLine>> {
    let (data, _preamble) = take_until("DATA;")(input)?;
    let (data, _) = take(5usize)(data)?;

    let (remaining, data) = many1(data_line)(data)?;

    Ok((remaining, data))
}
