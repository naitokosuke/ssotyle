fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    match args.as_slice() {
        [_, filepath] => {
            let content = std::fs::read_to_string(filepath)?;
            println!("{}, {}", filepath, content);
        }
        _ => {
            eprintln!("Usage: ssotyle <file>");
            std::process::exit(1);
        }
    }
    Ok(())
}

enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}
