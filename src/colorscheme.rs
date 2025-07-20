use std::collections::HashMap;

use colored::Color;

#[derive(Clone, Copy, Debug)]
pub struct Colorscheme {
    pub normal: Color,
    pub member: Color,
    pub type_: Color,
    pub function: Color,
    pub constant: Color,
    pub operator: Color,
    pub punctuation: Color,
    pub string: Color,
}

impl Default for Colorscheme {
    fn default() -> Self {
        Self::PASTEL
    }
}

impl Colorscheme {
    pub fn colorschemes() -> HashMap<String, Colorscheme> {
        HashMap::from([
            ("base16".into(), Self::BASE16),
            ("sand".into(), Self::SAND),
            ("ocean".into(), Self::OCEAN),
            ("evil".into(), Self::EVIL),
            ("pastel".into(), Self::PASTEL),
        ])
    }

    pub const BASE16: Self = Self {
        normal: Color::White,
        member: Color::Blue,
        type_: Color::Cyan,
        function: Color::BrightBlue,
        constant: Color::Yellow,
        operator: Color::BrightBlack,
        punctuation: Color::BrightBlack,
        string: Color::Green,
    };

    pub const SAND: Self = Self {
        normal: Color::TrueColor {
            r: 210,
            g: 201,
            b: 187,
        },
        member: Color::TrueColor {
            r: 183,
            g: 158,
            b: 117,
        },
        type_: Color::TrueColor {
            r: 198,
            g: 165,
            b: 109,
        },
        function: Color::TrueColor {
            r: 209,
            g: 167,
            b: 100,
        },
        constant: Color::TrueColor {
            r: 198,
            g: 70,
            b: 64,
        },
        operator: Color::TrueColor {
            r: 158,
            g: 151,
            b: 140,
        },
        punctuation: Color::TrueColor {
            r: 138,
            g: 132,
            b: 122,
        },
        string: Color::TrueColor {
            r: 98,
            g: 147,
            b: 187,
        },
    };

    pub const OCEAN: Self = Self {
        normal: Color::TrueColor {
            r: 210,
            g: 201,
            b: 187,
        },
        member: Color::TrueColor {
            r: 155,
            g: 155,
            b: 173,
        },
        type_: Color::TrueColor {
            r: 143,
            g: 175,
            b: 167,
        },
        function: Color::TrueColor {
            r: 149,
            g: 174,
            b: 167,
        },
        constant: Color::TrueColor {
            r: 204,
            g: 139,
            b: 102,
        },
        operator: Color::TrueColor {
            r: 158,
            g: 151,
            b: 140,
        },
        punctuation: Color::TrueColor {
            r: 138,
            g: 132,
            b: 122,
        },
        string: Color::TrueColor {
            r: 175,
            g: 175,
            b: 135,
        },
    };

    pub const EVIL: Self = Self {
        normal: Color::TrueColor {
            r: 255,
            g: 255,
            b: 255,
        },
        member: Color::TrueColor {
            r: 150,
            g: 150,
            b: 150,
        },
        type_: Color::TrueColor {
            r: 180,
            g: 180,
            b: 180,
        },
        function: Color::TrueColor {
            r: 220,
            g: 220,
            b: 220,
        },
        constant: Color::TrueColor {
            r: 198,
            g: 70,
            b: 64,
        },
        operator: Color::TrueColor {
            r: 150,
            g: 150,
            b: 150,
        },
        punctuation: Color::TrueColor {
            r: 120,
            g: 120,
            b: 120,
        },
        string: Color::TrueColor {
            r: 180,
            g: 180,
            b: 180,
        },
    };

    pub const PASTEL: Self = Self {
        normal: Color::TrueColor {
            r: 209,
            g: 203,
            b: 194,
        },
        member: Color::TrueColor {
            r: 163,
            g: 185,
            b: 206,
        },
        type_: Color::TrueColor {
            r: 158,
            g: 206,
            b: 211,
        },
        function: Color::TrueColor {
            r: 102,
            g: 155,
            b: 188,
        },
        constant: Color::TrueColor {
            r: 229,
            g: 91,
            b: 103,
        },
        operator: Color::TrueColor {
            r: 150,
            g: 150,
            b: 150,
        },
        punctuation: Color::TrueColor {
            r: 120,
            g: 120,
            b: 120,
        },
        string: Color::TrueColor {
            r: 216,
            g: 161,
            b: 71,
        },
    };
}
