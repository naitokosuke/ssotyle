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

#[derive(Debug, PartialEq)]
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
        let mut entries: Vec<(String, JsonValue)> = Vec::new();
        self.advance();

        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('}') => {
                    self.advance();
                    return Ok(JsonValue::Object(entries));
                }
                Some(':' | ',') => self.advance(),
                Some(_) => {
                    let key = self.parse_string()?;
                    self.skip_whitespace();
                    self.advance();
                    let value = self.parse_value()?;
                    entries.push((key, value))
                }
                None => return Err("Object is not closed".to_string()),
            }
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        let mut elements: Vec<JsonValue> = Vec::new();
        self.advance();

        loop {
            self.skip_whitespace();
            match self.peek() {
                Some(']') => {
                    self.advance();
                    return Ok(JsonValue::Array(elements));
                }
                Some(',') => self.advance(),
                Some(_) => {
                    let value = self.parse_value()?;
                    elements.push(value);
                }
                None => return Err("Array is not closed".to_string()),
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let mut parser = Parser {
            input: r#""hello""#.chars().collect(),
            pos: 0,
        };
        assert_eq!(parser.parse_string().unwrap(), "hello");
    }

    #[test]
    fn test_parse_string_empty() {
        let mut parser = Parser {
            input: r#""""#.chars().collect(),
            pos: 0,
        };
        assert_eq!(parser.parse_string().unwrap(), "");
    }

    #[test]
    fn test_parse_string_with_spaces() {
        let mut parser = Parser {
            input: r#""hello world""#.chars().collect(),
            pos: 0,
        };
        assert_eq!(parser.parse_string().unwrap(), "hello world");
    }

    #[test]
    fn test_parse_string_unclosed() {
        let mut parser = Parser {
            input: r#""hello"#.chars().collect(),
            pos: 0,
        };
        assert!(parser.parse_string().is_err());
    }

    #[test]
    fn test_parse_number() {
        let mut parser = Parser {
            input: "42".chars().collect(),
            pos: 0,
        };
        assert_eq!(parser.parse_number().unwrap(), JsonValue::Number(42.0));
    }

    #[test]
    fn test_parse_number_decimal() {
        let mut parser = Parser {
            input: "3.14".chars().collect(),
            pos: 0,
        };
        assert_eq!(parser.parse_number().unwrap(), JsonValue::Number(3.14));
    }

    #[test]
    fn test_parse_null() {
        let mut parser = Parser {
            input: "null".chars().collect(),
            pos: 0,
        };
        assert_eq!(parser.parse_null().unwrap(), JsonValue::Null);
    }

    #[test]
    fn test_parse_bool_true() {
        let mut parser = Parser {
            input: "true".chars().collect(),
            pos: 0,
        };
        assert_eq!(parser.parse_bool().unwrap(), JsonValue::Bool(true));
    }

    #[test]
    fn test_parse_bool_false() {
        let mut parser = Parser {
            input: "false".chars().collect(),
            pos: 0,
        };
        assert_eq!(parser.parse_bool().unwrap(), JsonValue::Bool(false));
    }

    #[test]
    fn test_parse_array() {
        let mut parser = Parser {
            input: "[1, 2, 3]".chars().collect(),
            pos: 0,
        };
        assert_eq!(
            parser.parse_array().unwrap(),
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
            ])
        );
    }

    #[test]
    fn test_parse_array_empty() {
        let mut parser = Parser {
            input: "[]".chars().collect(),
            pos: 0,
        };
        assert_eq!(parser.parse_array().unwrap(), JsonValue::Array(vec![]));
    }

    #[test]
    fn test_parse_object() {
        let mut parser = Parser {
            input: r#"{"name": "test", "value": 42}"#.chars().collect(),
            pos: 0,
        };
        assert_eq!(
            parser.parse_object().unwrap(),
            JsonValue::Object(vec![
                ("name".to_string(), JsonValue::Str("test".to_string())),
                ("value".to_string(), JsonValue::Number(42.0)),
            ])
        );
    }

    #[test]
    fn test_parse_object_empty() {
        let mut parser = Parser {
            input: "{}".chars().collect(),
            pos: 0,
        };
        assert_eq!(parser.parse_object().unwrap(), JsonValue::Object(vec![]));
    }

    #[test]
    fn test_parse_value_nested() {
        let mut parser = Parser {
            input: r##"{"colors": {"black": {"$value": "#000000"}}}"##.chars().collect(),
            pos: 0,
        };
        assert_eq!(
            parser.parse_value().unwrap(),
            JsonValue::Object(vec![(
                "colors".to_string(),
                JsonValue::Object(vec![(
                    "black".to_string(),
                    JsonValue::Object(vec![(
                        "$value".to_string(),
                        JsonValue::Str("#000000".to_string()),
                    )]),
                )]),
            )])
        );
    }
}
