use reedline::{ValidationResult, Validator};

pub struct AfshValidator;

impl Validator for AfshValidator {
    fn validate(&self, line: &str) -> ValidationResult {
        let mut brackets = Vec::new();
        let mut quote = None;
        let mut comment = false;

        for ch in line.chars() {
            if comment {
                if ch == '\n' { comment = false; }
                continue;
            }
            if let Some(delimiter) = quote {
                if ch == delimiter { quote = None; }
                continue;
            }
            match ch {
                '#' => comment = true,
                '\'' | '"' => quote = Some(ch),
                '{' | '[' => brackets.push(ch),
                '}' | ']' => {
                    let expected = if ch == '}' { '{' } else { '[' };
                    if brackets.pop() != Some(expected) {
                        return ValidationResult::Complete;
                    }
                }
                _ => {},
            }
        }
        if quote.is_some() || !brackets.is_empty() {
            ValidationResult::Incomplete
        } else {
            ValidationResult::Complete
        }
    }
}

