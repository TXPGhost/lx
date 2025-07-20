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
        Self::DEFAULT
    }
}

impl Colorscheme {
    pub const DEFAULT: Self = Self {
        normal: Color::White,
        member: Color::Blue,
        type_: Color::Cyan,
        function: Color::BrightBlue,
        constant: Color::Yellow,
        operator: Color::BrightBlack,
        punctuation: Color::BrightBlack,
        string: Color::Green,
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
