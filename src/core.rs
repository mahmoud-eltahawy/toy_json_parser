use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
    combinator::value,
    multi::separated_list0,
    number::complete::float,
    sequence::{delimited, pair, separated_pair},
};

#[derive(Debug)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f32),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

pub fn parse_null(input: &str) -> IResult<&str, JsonValue> {
    let target = tag("null");
    let mut delim = delimited(multispace0, target, multispace0);
    delim.parse(input).map(|(rest, _)| (rest, JsonValue::Null))
}

pub fn parse_number(input: &str) -> IResult<&str, JsonValue> {
    let mut delim = delimited(multispace0, float, multispace0);
    delim
        .parse(input)
        .map(|(rest, num)| (rest, JsonValue::Number(num)))
}

pub fn parse_bool(input: &str) -> IResult<&str, JsonValue> {
    let target = alt((value(true, tag("true")), value(false, tag("false"))));
    let mut delim = delimited(multispace0, target, multispace0);
    delim
        .parse(input)
        .map(|(rest, b)| (rest, JsonValue::Bool(b)))
}

pub fn parse_key_string(input: &str) -> IResult<&str, String> {
    let mut open = pair(multispace0, tag("\""));
    let mut close = pair(tag("\""), multispace0);
    let (rest, _) = open.parse(input)?;
    let (rest, result) = take_until("\"").parse(rest)?;
    let (rest, _) = close.parse(rest)?;
    Ok((rest, result.to_string()))
}

pub fn parse_json_string(input: &str) -> IResult<&str, JsonValue> {
    let mut open = pair(multispace0, tag("\""));
    let mut close = pair(tag("\""), multispace0);
    let (rest, _) = open.parse(input)?;
    let (rest, result) = take_until("\"").parse(rest)?;
    let (rest, _) = close.parse(rest)?;
    Ok((rest, JsonValue::String(result.to_string())))
}

// let separator = delimited(tag(" "), tag(","), tag(" "));
pub fn parse_array(input: &str) -> IResult<&str, JsonValue> {
    let open = delimited(multispace0, tag("["), multispace0);
    let close = delimited(multispace0, tag("]"), multispace0);
    let delim = delimited(multispace0, tag(","), multispace0);
    let (rest, arr) =
        delimited(open, separated_list0(delim, parse_json_value), close).parse(input)?;
    Ok((rest, JsonValue::Array(arr)))
}

pub fn parse_object(input: &str) -> IResult<&str, JsonValue> {
    let colon = delimited(multispace0, tag(":"), multispace0);
    let delim = delimited(multispace0, tag(","), multispace0);
    let pair_parser = separated_pair(parse_key_string, colon, parse_json_value);

    let (rest, arr) =
        delimited(tag("{"), separated_list0(delim, pair_parser), tag("}")).parse(input)?;
    Ok((rest, JsonValue::Object(arr)))
}

pub fn parse_json_value(input: &str) -> IResult<&str, JsonValue> {
    let res = alt((
        parse_null,
        parse_bool,
        parse_number,
        parse_json_string,
        parse_array,
        parse_object,
    ))
    .parse(input)?;
    Ok(res)
}
