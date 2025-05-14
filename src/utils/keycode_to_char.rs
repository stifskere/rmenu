use sdl2::keyboard::Keycode;

pub fn keycode_to_char(keycode: Keycode, shift: bool) -> Option<char> {
    Some(match keycode {
        Keycode::A => {
            if shift {
                'A'
            } else {
                'a'
            }
        },
        Keycode::B => {
            if shift {
                'B'
            } else {
                'b'
            }
        },
        Keycode::C => {
            if shift {
                'C'
            } else {
                'c'
            }
        },
        Keycode::D => {
            if shift {
                'D'
            } else {
                'd'
            }
        },
        Keycode::E => {
            if shift {
                'E'
            } else {
                'e'
            }
        },
        Keycode::F => {
            if shift {
                'F'
            } else {
                'f'
            }
        },
        Keycode::G => {
            if shift {
                'G'
            } else {
                'g'
            }
        },
        Keycode::H => {
            if shift {
                'H'
            } else {
                'h'
            }
        },
        Keycode::I => {
            if shift {
                'I'
            } else {
                'i'
            }
        },
        Keycode::J => {
            if shift {
                'J'
            } else {
                'j'
            }
        },
        Keycode::K => {
            if shift {
                'K'
            } else {
                'k'
            }
        },
        Keycode::L => {
            if shift {
                'L'
            } else {
                'l'
            }
        },
        Keycode::M => {
            if shift {
                'M'
            } else {
                'm'
            }
        },
        Keycode::N => {
            if shift {
                'N'
            } else {
                'n'
            }
        },
        Keycode::O => {
            if shift {
                'O'
            } else {
                'o'
            }
        },
        Keycode::P => {
            if shift {
                'P'
            } else {
                'p'
            }
        },
        Keycode::Q => {
            if shift {
                'Q'
            } else {
                'q'
            }
        },
        Keycode::R => {
            if shift {
                'R'
            } else {
                'r'
            }
        },
        Keycode::S => {
            if shift {
                'S'
            } else {
                's'
            }
        },
        Keycode::T => {
            if shift {
                'T'
            } else {
                't'
            }
        },
        Keycode::U => {
            if shift {
                'U'
            } else {
                'u'
            }
        },
        Keycode::V => {
            if shift {
                'V'
            } else {
                'v'
            }
        },
        Keycode::W => {
            if shift {
                'W'
            } else {
                'w'
            }
        },
        Keycode::X => {
            if shift {
                'X'
            } else {
                'x'
            }
        },
        Keycode::Y => {
            if shift {
                'Y'
            } else {
                'y'
            }
        },
        Keycode::Z => {
            if shift {
                'Z'
            } else {
                'z'
            }
        },

        Keycode::Num0 => {
            if shift {
                ')'
            } else {
                '0'
            }
        },
        Keycode::Num1 => {
            if shift {
                '!'
            } else {
                '1'
            }
        },
        Keycode::Num2 => {
            if shift {
                '@'
            } else {
                '2'
            }
        },
        Keycode::Num3 => {
            if shift {
                '#'
            } else {
                '3'
            }
        },
        Keycode::Num4 => {
            if shift {
                '$'
            } else {
                '4'
            }
        },
        Keycode::Num5 => {
            if shift {
                '%'
            } else {
                '5'
            }
        },
        Keycode::Num6 => {
            if shift {
                '^'
            } else {
                '6'
            }
        },
        Keycode::Num7 => {
            if shift {
                '&'
            } else {
                '7'
            }
        },
        Keycode::Num8 => {
            if shift {
                '*'
            } else {
                '8'
            }
        },
        Keycode::Num9 => {
            if shift {
                '('
            } else {
                '9'
            }
        },

        Keycode::Space => ' ',
        Keycode::Minus => {
            if shift {
                '_'
            } else {
                '-'
            }
        },
        Keycode::Equals => {
            if shift {
                '+'
            } else {
                '='
            }
        },
        Keycode::LeftBracket => {
            if shift {
                '{'
            } else {
                '['
            }
        },
        Keycode::RightBracket => {
            if shift {
                '}'
            } else {
                ']'
            }
        },
        Keycode::Backslash => {
            if shift {
                '|'
            } else {
                '\\'
            }
        },
        Keycode::Semicolon => {
            if shift {
                ':'
            } else {
                ';'
            }
        },
        Keycode::Quote => {
            if shift {
                '"'
            } else {
                '\''
            }
        },
        Keycode::Comma => {
            if shift {
                '<'
            } else {
                ','
            }
        },
        Keycode::Period => {
            if shift {
                '>'
            } else {
                '.'
            }
        },
        Keycode::Slash => {
            if shift {
                '?'
            } else {
                '/'
            }
        },
        Keycode::Backquote => {
            if shift {
                '~'
            } else {
                '`'
            }
        },

        _ => return None,
    })
}
