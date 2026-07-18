use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::multispace0,
    combinator::value,
    multi::separated_list0,
    number::complete::float,
    sequence::{delimited, pair, separated_pair},
};

#[derive(Debug)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(f32),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

fn parse_null(input: &str) -> IResult<&str, JsonValue> {
    tag("null")(input).map(|(rest, _)| (rest, JsonValue::Null))
}
fn parse_number(input: &str) -> IResult<&str, JsonValue> {
    float(input).map(|(rest, num)| (rest, JsonValue::Number(num)))
}
fn parse_bool(input: &str) -> IResult<&str, JsonValue> {
    alt((value(true, tag("true")), value(false, tag("false"))))
        .parse(input)
        .map(|(rest, b)| (rest, JsonValue::Bool(b)))
}
fn parse_key_string(input: &str) -> IResult<&str, String> {
    let (rest, _) = tag("\"").parse(input)?;
    let (rest, result) = take_until("\"").parse(rest)?;
    let (rest, _) = tag("\"").parse(rest)?;
    Ok((rest, result.to_string()))
}
fn parse_json_string(input: &str) -> IResult<&str, JsonValue> {
    let (rest, _) = tag("\"").parse(input)?;
    let (rest, result) = take_until("\"").parse(rest)?;
    let (rest, _) = tag("\"").parse(rest)?;
    Ok((rest, JsonValue::String(result.to_string())))
}
// let separator = delimited(tag(" "), tag(","), tag(" "));
fn parse_array(input: &str) -> IResult<&str, JsonValue> {
    let (rest, arr) = delimited(
        tag("["),
        separated_list0(tag(","), parse_json_value),
        tag("]"),
    )
    .parse(input)?;
    Ok((rest, JsonValue::Array(arr)))
}
fn parse_object(input: &str) -> IResult<&str, JsonValue> {
    let pair_parser = separated_pair(parse_key_string, tag(":"), parse_json_value);

    let (rest, arr) =
        delimited(tag("{"), separated_list0(tag(","), pair_parser), tag("}")).parse(input)?;
    Ok((rest, JsonValue::Object(arr)))
}
fn parse_json_value(input: &str) -> IResult<&str, JsonValue> {
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

fn main() {
    let i = parse_null("null hello null").unwrap();
    let j = parse_number("77.2002 hello float").unwrap();
    let k = parse_bool("false hello bool").unwrap();
    let l = parse_json_string("\"hello string\" hello bool").unwrap();
    let data =
        r#"["hello",123.455,true,false,["hello",123.455,true,false],{"name":"mahmoud","age":26}]"#;
    let ob = r#"{"name":"mahmoud","age":26}"#;
    let o = parse_object(ob).unwrap();
    let arr = parse_array(data).unwrap();
    dbg!(i);
    dbg!(j);
    dbg!(k);
    dbg!(l);
    dbg!(o);
    dbg!(arr);
}
