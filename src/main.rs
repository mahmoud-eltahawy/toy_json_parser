pub mod core;

fn main() {
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
    let ob = r#"
        {
            "name":"mahmoud",
            "age":26
        }"#;
    let object = core::parse(ob);
    let array = core::parse(data);
    dbg!(object);
    dbg!(array);
}
