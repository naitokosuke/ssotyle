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

    fn advance(&mut self) {
        self.pos += 1
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        self.skip_whitespace();
        match self.peek() {
            Some('"') => {
                let s = self.parse_string()?;
                Ok(JsonValue::Str(s))
            }
            Some('{') => self.parse_object(),
            Some('[') => self.parse_array(),
            Some('t') | Some('f') => self.parse_bool(),
            Some('n') => self.parse_null(),
            Some(c) if c.is_ascii_digit() || c == '-' => self.parse_number(),
            _ => Err("Unexpected Character".to_string()),
        }
    }

    fn parse_string(&mut self) -> Result<String, String> {
        let mut chars: Vec<char> = Vec::new();
        self.advance();
        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                return Ok(chars.iter().collect());
            }
            chars.push(c);
            self.advance();
        }
        Err("String is not closed".to_string())
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        todo!()
    }
    fn parse_array(&mut self) -> Result<JsonValue, String> {
        todo!()
    }
    fn parse_bool(&mut self) -> Result<JsonValue, String> {
        match self.peek() {
            Some('t') => {
                for _ in 0..4 {
                    self.advance();
                }
                Ok(JsonValue::Bool(true))
            }
            Some('f') => {
                for _ in 0..5 {
                    self.advance();
                }
                Ok(JsonValue::Bool(false))
            }
            _ => Err("Unexpected Character".to_string()),
        }
    }
    fn parse_null(&mut self) -> Result<JsonValue, String> {
        for _ in 0..4 {
            self.advance();
        }
        Ok(JsonValue::Null)
    }
    fn parse_number(&mut self) -> Result<JsonValue, String> {
        let mut chars: Vec<char> = Vec::new();
        while let Some(c) = self.peek() {
            match c {
                '0'..='9' | '.' | '-' => {
                    chars.push(c);
                    self.advance();
                }
                _ => break,
            }
        }
        let s: String = chars.iter().collect();
        let n: f64 = s.parse().map_err(|_| format!("Invalid Number: {}", s))?;
        Ok(JsonValue::Number(n))
    }
}
