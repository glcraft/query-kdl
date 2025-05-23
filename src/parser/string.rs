use super::{error::ParseStringError, Value};
use std::borrow::Cow;

pub type Result<T> = std::result::Result<T, ParseStringError>;

pub fn parse_string<'b>(input: &'b str) -> Result<Cow<'b, str>> {
    match input.chars().next() {
        None => return Err(ParseStringError::EmptyString),
        Some(c) if c != '"' => return Err(ParseStringError::MissingBeginOfString),
        Some(_) => (),
    }

    match input.chars().last() {
        None => unreachable!(),
        Some(c) if c != '"' => return Err(ParseStringError::MissingEndOfString),
        Some(_) => (),
    }
    if input.chars().any(|c| c == '\\') {
        Ok(Cow::Owned(unescape_string(&input[1..input.len() - 1])?))
    } else {
        Ok(Cow::Borrowed(&input[1..input.len() - 1]))
    }
}

#[derive(Clone, Copy)]
enum State {
    None,
    Escape,
    Ascii(u8, u32),
    EnterUnicode,
    Unicode(u32, u32),
}
struct UnescapeString {
    state: State,
    output: String,
}
impl UnescapeString {
    pub fn new(capacity: usize) -> Self {
        Self {
            state: State::None,
            output: String::with_capacity(capacity),
        }
    }
    pub fn push_char(&mut self, c: char) {
        self.output.push(c);
        self.change_state(State::None);
    }
    pub fn change_state(&mut self, state: State) {
        self.state = state;
    }
}
pub fn unescape_string<'a>(input: &'a str) -> Result<String> {
    let mut state = UnescapeString::new(input.len());
    for c in input.chars() {
        match state.state {
            State::None => {
                if c == '\\' {
                    state.change_state(State::Escape);
                } else {
                    state.push_char(c);
                }
            }
            State::Escape => {
                match c {
                    'n' => state.push_char('\n'),
                    't' => state.push_char('\t'),
                    'r' => state.push_char('\r'),
                    // '0' => state.push_char('\0'),
                    '\\' => state.push_char('\\'),
                    'x' => state.change_state(State::Ascii(0, 0)),
                    'u' => state.change_state(State::EnterUnicode),
                    _ => return Err(ParseStringError::UnknownEscape(c)),
                }
            }
            State::Ascii(mut codepoint, mut len) => {
                codepoint =
                    (codepoint << 4) + c.to_digit(16).ok_or(ParseStringError::NotHexDigit)? as u8;
                len += 1;
                if len == 2 {
                    if matches!(codepoint, 0 | 0x80..) {
                        return Err(ParseStringError::AsciiNotValid(codepoint));
                    }
                    state.push_char(codepoint.into());
                } else {
                    state.change_state(State::Ascii(codepoint, len));
                }
            }
            State::Unicode(mut codepoint, mut len) => {
                if c == '}' {
                    if codepoint > 0x10FFFF {
                        return Err(ParseStringError::UnicodeOutOfBound(codepoint));
                    }
                    state.push_char(
                        char::from_u32(codepoint)
                            .ok_or(ParseStringError::UnicodeNotValid(codepoint))?,
                    );
                } else {
                    codepoint =
                        (codepoint << 4) + c.to_digit(16).ok_or(ParseStringError::NotHexDigit)?;
                    len += 1;
                    state.change_state(State::Unicode(codepoint, len));
                    if len > 6 {
                        return Err(ParseStringError::UnicodeMoreThanSixDigits);
                    }
                }
            }
            State::EnterUnicode => {
                if c != '{' {
                    return Err(ParseStringError::ExpectedCurlyBracket);
                }
                state.change_state(State::Unicode(0, 0));
            }
        }
    }
    Ok(state.output)
}

pub fn parse_alphanumeric<'a>(input: &'a str) -> Result<Value<'a>> {
    enum Kind {
        Int(u32),
        Float,
        Str,
    }
    use Kind::*;
    let (sign, input2) = match input.chars().next() {
        None => unreachable!(),
        Some('-') => (-1, &input[1..]),
        Some('+') => (1, &input[1..]),
        Some(_) => (1, input),
    };

    let kind = if input2.starts_with("0x") {
        Int(16)
    } else if input2.starts_with("0o") {
        Int(8)
    } else if input2.starts_with("0b") {
        Int(2)
    } else if input2.chars().next().map(char::is_numeric) == Some(true) {
        match input2.chars().filter(|c| *c == '.').count() {
            0 => Int(10),
            1 => Float,
            _ => return Err(ParseStringError::MalformedNumber),
        }
    } else {
        Str
    };
    let result = match kind {
        Int(radix) => i128::from_str_radix(&input2[(2 * ((radix != 10) as usize))..], radix)
            .map(|v| Value::Integer(v * sign))
            .map_err(|_| ParseStringError::MalformedNumber)?,
        Float => input
            .parse()
            .map(Value::FloatingPoing)
            .map_err(|_| ParseStringError::MalformedNumber)?,
        Str => Value::String(Cow::Borrowed(input)),
    };
    Ok(result)
}
pub fn parse_keyword<'a>(input: &'a str) -> super::Result<'a, Value<'a>> {
    match input {
        "true" => Ok(Value::Boolean(true)),
        "false" => Ok(Value::Boolean(false)),
        "null" => Ok(Value::Null),
        _ => Err(super::ParseError::UnknownKeyword(input)),
    }
}
