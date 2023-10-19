use serde_json;

fn main() {
    let json = r#"
    {
        "name": "John Doe",
        "age": bin(0x10FF1A00),
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    println!("json: {}", json);

    let parsed: serde_json::Value = serde_json::from_str(json).unwrap();
    println!("name: {}", parsed["name"]);
    println!("age: {}", parsed["age"]);
    println!("phones: {}", parsed["phones"]);
}
