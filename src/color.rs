use std::fmt::{Debug, Display};

pub fn fg(color: u8) -> String {
    format!(r"\033[1;38;5;{}m", wrap(color as usize))
}
pub fn bg(color: u8) -> String {
    format!(r"\033[1;48;5;{}m", wrap(color as usize))
}

pub fn fore<T: std::fmt::Display>(text: T, color: u8) -> String {
    [fg(color), text.to_string()].join("")
}
pub fn back<T: std::fmt::Display>(text: T, color: u8) -> String {
    [bg(color), text.to_string()].join("")
}
pub fn reset() -> String {
    format!(r"\033[0m")
}
pub fn reset_text<T: std::fmt::Display>(text: T) -> String {
    [reset(), text.to_string()].join("")
}
pub fn bgfg<T: std::fmt::Display>(text: T, fore_color: u8, back_color: u8) -> String {
    back(fore(text, fore_color), back_color)
}
pub fn ansi<T: std::fmt::Display>(text: T, fore: u8, back: u8) -> String {
    reset_text(bgfg(text, fore, back))
}
pub fn ansi_clear() -> String {
    "\x1b[2J\x1b[3J\x1b[H".to_string()
}
/// wraps non-printable text
pub fn wrap_np<T: std::fmt::Display>(text: T) -> String {
    // text.to_string()
    format!(r"\[{}\]", text)
}
pub fn foreground<T: std::fmt::Display>(text: T, color: u8) -> String {
    reset_text(fore(text, color))
}
pub fn background<T: std::fmt::Display>(text: T, color: u8) -> String {
    reset_text(back(text, color))
}

pub fn from_string<T: Display>(word: T) -> u8 {
    from_bytes(word.to_string().as_bytes())
}
pub fn rgb_from_string<T: Display>(word: T) -> [u8; 3] {
    rgb_from_bytes(word.to_string().as_bytes())
}
pub fn from_bytes(bytes: &[u8]) -> u8 {
    eprintln!("");
    let mut color: u8 = 0;
    for rgb in rgb_from_bytes(bytes) {
        color = color ^ rgb
    }
    color
}
pub fn rgb_from_bytes(bytes: &[u8]) -> [u8; 3] {
    let mut color: [u8; 3] = [0, 0, 0];
    for (index, byte) in bytes.iter().enumerate() {
        color[index % 3] = *byte
    }
    color
}

pub fn couple(color: u8) -> (u8, u8) {
    let fore = wrap(color as usize);
    let back = invert_bw(fore);
    (fore, back)
}

pub fn invert_bw(color: u8) -> u8 {
    match color {
        0 | 8 | 16..21 | 52..61 | 88..93 | 232..239 => 231,
        _ => 16,
    }
}

pub fn wrap(color: usize) -> u8 {
    (if color > 0 {
        color % 255
    } else {
        color
    }) as u8
}
