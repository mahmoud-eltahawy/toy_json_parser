use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
    combinator::{cut, value},
    error::context,
    multi::separated_list0,
    number::complete::float,
    sequence::{delimited, pair, separated_pair},
};
use nom_language::error::{VerboseError, convert_error};

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f32),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

pub fn parse(input: &str) -> Result<JsonValue, String> {
    match parse_json_value(input) {
        Ok((_, val)) => Ok(val),
        Err(err) => match err {
            nom::Err::Error(e) | nom::Err::Failure(e) => Err(convert_error(input, e)),
            nom::Err::Incomplete(needed) => Err(format!("INCOMPLETE : {needed:#?}")),
        },
    }
}

fn parse_null(input: &str) -> IResult<&str, JsonValue, VerboseError<&str>> {
    let target = tag("null");
    let mut delim = delimited(multispace0, target, multispace0);
    delim.parse(input).map(|(rest, _)| (rest, JsonValue::Null))
}

fn parse_number(input: &str) -> IResult<&str, JsonValue, VerboseError<&str>> {
    let mut delim = delimited(multispace0, float, multispace0);
    delim
        .parse(input)
        .map(|(rest, num)| (rest, JsonValue::Number(num)))
}

fn parse_bool(input: &str) -> IResult<&str, JsonValue, VerboseError<&str>> {
    let target = alt((value(true, tag("true")), value(false, tag("false"))));
    let mut delim = delimited(multispace0, target, multispace0);
    delim
        .parse(input)
        .map(|(rest, b)| (rest, JsonValue::Bool(b)))
}

fn parse_key_string(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    let mut open = pair(multispace0, tag("\""));
    let close = pair(tag("\""), multispace0);
    let (rest, _) = open.parse(input)?;
    let (rest, result) = cut(take_until("\"")).parse(rest)?;
    let (rest, _) = cut(close).parse(rest)?;
    Ok((rest, result.to_string()))
}

fn parse_json_string(input: &str) -> IResult<&str, JsonValue, VerboseError<&str>> {
    parse_key_string(input).map(|(rest, s)| (rest, JsonValue::String(s)))
}

// let separator = delimited(tag(" "), tag(","), tag(" "));
fn parse_array(input: &str) -> IResult<&str, JsonValue, VerboseError<&str>> {
    let open = delimited(multispace0, tag("["), multispace0);
    let close = delimited(multispace0, tag("]"), multispace0);
    let delim = delimited(multispace0, tag(","), multispace0);
    let (rest, arr) = delimited(
        open,
        cut(separated_list0(delim, parse_json_value)),
        cut(close),
    )
    .parse(input)?;
    Ok((rest, JsonValue::Array(arr)))
}

fn parse_object(input: &str) -> IResult<&str, JsonValue, VerboseError<&str>> {
    let open = delimited(multispace0, tag("{"), multispace0);
    let close = delimited(multispace0, tag("}"), multispace0);
    let colon = delimited(multispace0, tag(":"), multispace0);
    let delim = delimited(multispace0, tag(","), multispace0);
    let pair_parser = separated_pair(parse_key_string, colon, parse_json_value);

    let (rest, arr) =
        delimited(open, cut(separated_list0(delim, pair_parser)), cut(close)).parse(input)?;
    Ok((rest, JsonValue::Object(arr)))
}

fn parse_json_value(input: &str) -> IResult<&str, JsonValue, VerboseError<&str>> {
    let res = alt((
        context("null", parse_null),
        context("bool", parse_bool),
        context("number", parse_number),
        context("string", parse_json_string),
        context("array", parse_array),
        context("object", parse_object),
    ))
    .parse(input)?;
    Ok(res)
}
