use std::char;
use std::str::Chars;

pub fn unescape(chars: &mut Chars) -> Option<char> {
    match chars.next() {
        Some('\\') => {
            match chars.next() {
                Some('0') => Some('\x00'), // NUL '\0' (null character)
                Some('a') => Some('\x07'), // BEL '\a' (bell)
                Some('b') => Some('\x08'), // BS  '\b' (backspace)
                Some('t') => Some('\x09'), // HT  '\t' (horizontal tab)
                Some('n') => Some('\x0a'), // LF  '\n' (new line)
                Some('v') => Some('\x0b'), // VT  '\v' (vertical tab)
                Some('f') => Some('\x0c'), // FF  '\f' (form feed)
                Some('r') => Some('\x0d'), // CR  '\r' (carriage ret)
                // \u**** inserts the unicode character U+****
                Some('u') => {
                    let hex: String = chars.take(4).collect();
                    u32::from_str_radix(&hex[..], 16)
                        .ok()
                        .and_then(char::from_u32)
                },
                // \x** inserts the byte 0x**
                Some('x') => {
                    let hex: String = chars.take(2).collect();
                    u32::from_str_radix(&hex[..], 16)
                        .ok()
                        .and_then(char::from_u32)
                },
                // If any other character is escaped just insert it
                Some(c) => Some(c),
                None => None
            }
        },
        Some(c) => Some(c),
        None => None
    }
}
