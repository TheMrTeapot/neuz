use tfc::{traits::*, Context, Error};

use super::{Key, KeyMode};

pub const IGNORE_AREA_TOP: u32 = 0;
pub const IGNORE_AREA_BOTTOM: u32 = 0;

#[allow(clippy::just_underscores_and_digits)]
impl From<Key> for char {
    fn from(k: Key) -> char {
        use Key::*;
        match k {
            _0 => '0',
            _1 => '1',
            _2 => '2',
            _3 => '3',
            _4 => '4',
            _5 => '5',
            _6 => '6',
            _7 => '7',
            _8 => '8',
            _9 => '9',
            W => 'w',
            A => 'a',
            S => 's',
            D => 'd',
            Space => ' ', // TODO
        }
    }
}

pub fn send_keystroke(k: Key, mode: KeyMode) {
    let k: char = k.into();
    let k = k as u8;
    let mut ctx = tfc::Context::new().unwrap();
    match mode {
        KeyMode::Press => ctx.ascii_char(k),
        KeyMode::Hold => ctx.ascii_char_down(k),
        KeyMode::Release => ctx.ascii_char_up(k),
    }
    .unwrap();
}
