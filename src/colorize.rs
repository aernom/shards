use std::fmt::Display;

pub enum Color {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    RGB(u8, u8, u8),
}

impl Color {
    fn fg(&self) -> String {
        match self {
            Color::Default => "39".into(),
            Color::Black => "30".into(),
            Color::Red => "31".into(),
            Color::Green => "32".into(),
            Color::Yellow => "33".into(),
            Color::Blue => "34".into(),
            Color::Magenta => "35".into(),
            Color::Cyan => "36".into(),
            Color::White => "37".into(),
            Color::BrightBlack => "90".into(),
            Color::BrightRed => "91".into(),
            Color::BrightGreen => "92".into(),
            Color::BrightYellow => "93".into(),
            Color::BrightBlue => "94".into(),
            Color::BrightMagenta => "95".into(),
            Color::BrightCyan => "96".into(),
            Color::BrightWhite => "97".into(),
            Color::RGB(r, g, b) => format!("38;2;{};{};{}", r, g, b),
        }
    }

    fn bg(&self) -> String {
        match self {
            Color::Default => "49".into(),
            Color::Black => "40".into(),
            Color::Red => "41".into(),
            Color::Green => "42".into(),
            Color::Yellow => "43".into(),
            Color::Blue => "44".into(),
            Color::Magenta => "45".into(),
            Color::Cyan => "46".into(),
            Color::White => "47".into(),
            Color::BrightBlack => "100".into(),
            Color::BrightRed => "101".into(),
            Color::BrightGreen => "102".into(),
            Color::BrightYellow => "103".into(),
            Color::BrightBlue => "104".into(),
            Color::BrightMagenta => "105".into(),
            Color::BrightCyan => "106".into(),
            Color::BrightWhite => "107".into(),
            Color::RGB(r, g, b) => format!("48;2;{};{};{}", r, g, b),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Default
    }
}

#[derive(Default)]
pub struct ColoredString {
    pub string: String,
    pub foreground: Color,
    pub background: Color,
}

impl ColoredString {
    pub fn black(mut self) -> Self {
        self.foreground = Color::Black;
        self
    }
    pub fn red(mut self) -> Self {
        self.foreground = Color::Red;
        self
    }
    pub fn green(mut self) -> Self {
        self.foreground = Color::Green;
        self
    }
    pub fn yellow(mut self) -> Self {
        self.foreground = Color::Yellow;
        self
    }
    pub fn blue(mut self) -> Self {
        self.foreground = Color::Blue;
        self
    }
    pub fn magenta(mut self) -> Self {
        self.foreground = Color::Magenta;
        self
    }
    pub fn cyan(mut self) -> Self {
        self.foreground = Color::Cyan;
        self
    }
    pub fn white(mut self) -> Self {
        self.foreground = Color::White;
        self
    }
    pub fn bright_black(mut self) -> Self {
        self.foreground = Color::BrightBlack;
        self
    }
    pub fn bright_red(mut self) -> Self {
        self.foreground = Color::BrightRed;
        self
    }
    pub fn bright_green(mut self) -> Self {
        self.foreground = Color::BrightGreen;
        self
    }
    pub fn bright_yellow(mut self) -> Self {
        self.foreground = Color::BrightYellow;
        self
    }
    pub fn bright_blue(mut self) -> Self {
        self.foreground = Color::BrightBlue;
        self
    }
    pub fn bright_magenta(mut self) -> Self {
        self.foreground = Color::BrightMagenta;
        self
    }
    pub fn bright_cyan(mut self) -> Self {
        self.foreground = Color::BrightCyan;
        self
    }
    pub fn bright_white(mut self) -> Self {
        self.foreground = Color::BrightWhite;
        self
    }
    pub fn rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.foreground = Color::RGB(r, g, b);
        self
    }
    pub fn on_black(mut self) -> Self {
        self.background = Color::Black;
        self
    }
    pub fn on_red(mut self) -> Self {
        self.background = Color::Red;
        self
    }
    pub fn on_green(mut self) -> Self {
        self.background = Color::Green;
        self
    }
    pub fn on_yellow(mut self) -> Self {
        self.background = Color::Yellow;
        self
    }
    pub fn on_blue(mut self) -> Self {
        self.background = Color::Blue;
        self
    }
    pub fn on_magenta(mut self) -> Self {
        self.background = Color::Magenta;
        self
    }
    pub fn on_cyan(mut self) -> Self {
        self.background = Color::Cyan;
        self
    }
    pub fn on_white(mut self) -> Self {
        self.background = Color::White;
        self
    }
    pub fn on_bright_black(mut self) -> Self {
        self.background = Color::BrightBlack;
        self
    }
    pub fn on_bright_red(mut self) -> Self {
        self.background = Color::BrightRed;
        self
    }
    pub fn on_bright_green(mut self) -> Self {
        self.background = Color::BrightGreen;
        self
    }
    pub fn on_bright_yellow(mut self) -> Self {
        self.background = Color::BrightYellow;
        self
    }
    pub fn on_bright_blue(mut self) -> Self {
        self.background = Color::BrightBlue;
        self
    }
    pub fn on_bright_magenta(mut self) -> Self {
        self.background = Color::BrightMagenta;
        self
    }
    pub fn on_bright_cyan(mut self) -> Self {
        self.background = Color::BrightCyan;
        self
    }
    pub fn on_bright_white(mut self) -> Self {
        self.background = Color::BrightWhite;
        self
    }
    pub fn on_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.background = Color::RGB(r, g, b);
        self
    }
}

impl Display for ColoredString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\x1B[{};{}m{}\x1B[0m",
            self.foreground.fg(),
            self.background.bg(),
            self.string
        ))
    }
}

impl From<&str> for ColoredString {
    fn from(value: &str) -> Self {
        Self {
            string: value.into(),
            foreground: Color::Default,
            background: Color::Default,
        }
    }
}

impl From<String> for ColoredString {
    fn from(value: String) -> Self {
        Self {
            string: value,
            foreground: Color::Default,
            background: Color::Default,
        }
    }
}

impl From<&String> for ColoredString {
    fn from(value: &String) -> Self {
        Self {
            string: value.into(),
            foreground: Color::Default,
            background: Color::Default,
        }
    }
}

pub trait Colored {
    fn colored(self) -> ColoredString;
}

impl Colored for &str {
    fn colored(self) -> ColoredString {
        self.into()
    }
}

impl Colored for String {
    fn colored(self) -> ColoredString {
        self.into()
    }
}

impl Colored for &String {
    fn colored(self) -> ColoredString {
        self.into()
    }
}
