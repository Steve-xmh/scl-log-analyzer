use nom::bytes::streaming::*;
use nom::character::{streaming::*};
use nom::combinator::*;

use nom::sequence::separated_pair;
use nom::IResult;

fn parse_time(input: &str) -> IResult<&str, &str> {
    recognize(separated_pair(
        separated_pair(tag("["), digit1, tag(":")),
        digit1,
        separated_pair(tag(":"), digit1, tag("] ")),
    ))(input)
}

pub fn parse_log_line(input: &str, find_next: bool) -> IResult<&str, &str> {
    let (input, _timestamp) = parse_time(input)?;
    if !find_next {
        return Ok((input, input));
    }
    for i in 0..input.len() + 1 {
        if input.is_char_boundary(i) {
            if parse_time(&input[i..]).is_ok() {
                return Ok((&input[i..], &input[..i]));
            }
        }
    }
    Err(nom::Err::Incomplete(nom::Needed::Unknown))
}
