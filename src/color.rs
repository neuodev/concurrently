pub struct Colored<'a> {
    text: &'a str,
    color: Color,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    Fuchsia,
    Indigo,
}

impl Color {
    fn get_color_code<'a>(&self) -> &'a str {
        match self {
            Color::Red => "248;113;113",
            Color::Fuchsia => "236;72;153",
            Color::Green => "74;222;128",
            Color::Yellow => "250;204;21",
            Color::Blue => "96;165;250",
            Color::Purple => "192;132;252",
            Color::Cyan => "34;211;238",
            Color::Indigo => "129;140;248",
        }
    }

    fn to_vec() -> Vec<Color> {
        vec![
            Color::Yellow,
            Color::Indigo,
            Color::Fuchsia,
            Color::Blue,
            Color::Purple,
            Color::Cyan,
            Color::Red,
            Color::Green,
        ]
    }
}

impl<'a> Colored<'a> {
    pub fn new(text: &'a str, color: Color) -> Self {
        Self { text, color }
    }

    pub fn new_with_idx(text: &'a str, idx: usize) -> Self {
        let colors = Color::to_vec();
        let color = colors[idx % colors.len()];
        Colored::new(text, color)
    }
}

impl<'a> std::fmt::Display for Colored<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_code = self.color.get_color_code();
        write!(f, "\x1B[38;2;{}m{}\x1B[0m", color_code, self.text)
    }
}
