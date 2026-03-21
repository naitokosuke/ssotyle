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

struct Parser {
    input: Vec<char>, // input string as a vector of characters
    pos: usize,       // current position in the input
}

impl Parser {
    /// Returns the character at the current position without advancing.
    /// Returns `None` if the position is past the end of input.
    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }
}
