pub struct Colored<'a> {
    text: &'a str,
    color: Color,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    Purple,
    Cyan,
    White,
}

impl Color {
    fn get_bg_code<'a>(&self) -> &'a str {
        match self {
            Color::BgBlack => "40",
            Color::BgRed => "41",
            Color::BgGreen => "42",
            Color::BgYellow => "43",
            Color::BgBlue => "44",
            Color::Purple => "45",
            Color::Cyan => "46",
            Color::White => "47",
        }
    }

    fn to_vec() -> Vec<Color> {
        vec![
            Color::BgBlack,
            Color::BgRed,
            Color::BgGreen,
            Color::BgYellow,
            Color::BgBlue,
            Color::Purple,
            Color::Cyan,
            Color::White,
        ]
    }
}

impl<'a> Colored<'a> {
    pub fn new(text: &'a str, color: Color) -> Self {
        Self { text, color }
    }

    pub fn new_with_idx(text: &'a str, idx: usize) {
        let colors = Color::to_vec();
        let color = colors[idx % colors.len()];
        Colored::new(text, color);
    }
}

impl<'a> std::fmt::Display for Colored<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bg_code = self.color.get_bg_code();
        write!(f, "\x1b[{};4m{}\x1b[0m", bg_code, self.text)
    }
}
