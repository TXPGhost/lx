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
        Self::SAND
    }
}

impl Colorscheme {
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

    pub const HABA: Self = Self {
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
}
