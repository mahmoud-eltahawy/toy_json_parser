pub mod core;

fn main() {
    let i = core::parse_null("null hello null").unwrap();
    let j = core::parse_number("77.2002 hello float").unwrap();
    let k = core::parse_bool("false hello bool").unwrap();
    let l = core::parse_json_string("\"hello string\" hello bool").unwrap();
    let data = r#"
    [
        null,
        "hello",
        123.455,
        true,
        false,
        [
            "hello",
            123.455,
            true,
            false
        ],
        {
            "name" : "mahmoud",
            "age" : 26.23,
            "langs": ["javascript","typescript","rust"]
        }
    ]"#;
    let ob = r#"{"name":"mahmoud","age":26}"#;
    let o = core::parse_object(ob).unwrap();
    let arr = core::parse_array(data).unwrap();
    dbg!(i);
    dbg!(j);
    dbg!(k);
    dbg!(l);
    dbg!(o);
    dbg!(arr);
}
