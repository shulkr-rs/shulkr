use std::cell::Cell;

use super::{
    color::{NamedColor, Rgb, Rgba},
    component::TextComponent,
    style::{ClickEvent, HoverEvent, TextStyle},
};

pub struct MiniMessage;

impl MiniMessage {
    pub fn parse(input: &str) -> TextComponent {
        Parser::new(input).run()
    }
}

#[derive(Clone)]
enum TextFormat {
    Color(Rgb),
    Bold(bool),
    Italic(bool),
    Underlined(bool),
    Strikethrough(bool),
    Obfuscated(bool),
    Font(String),
    Insertion(String),
    Shadow(Rgba),
    Click(ClickEvent),
    Hover(HoverEvent),
    Gradient(Gradient),
}

impl TextFormat {
    fn apply(&self, style: &mut TextStyle) {
        match self {
            TextFormat::Color(c) => style.set_color(*c),
            TextFormat::Bold(v) => style.set_bold(*v),
            TextFormat::Italic(v) => style.set_italic(*v),
            TextFormat::Underlined(v) => style.set_underlined(*v),
            TextFormat::Strikethrough(v) => style.set_strikethrough(*v),
            TextFormat::Obfuscated(v) => style.set_obfuscated(*v),
            TextFormat::Font(f) => style.set_font(f.clone()),
            TextFormat::Insertion(i) => style.set_insertion(i.clone()),
            TextFormat::Shadow(c) => style.set_shadow_color(*c),
            TextFormat::Click(c) => style.set_on_click(c.clone()),
            TextFormat::Hover(h) => style.set_on_hover(h.clone()),
            TextFormat::Gradient(_) => {}
        }
    }
}

#[derive(Clone)]
struct Gradient {
    colors: Vec<Rgb>,
    phase: f32,
    len: usize,
    index: Cell<usize>,
}

impl Gradient {
    fn next_color(&self) -> Rgb {
        let i = self.index.get();
        self.index.set(i + 1);
        self.sample(i)
    }

    fn sample(&self, i: usize) -> Rgb {
        if self.colors.len() == 1 {
            return self.colors[0];
        }
        let denom = self.len.saturating_sub(1).max(1) as f32;
        let t = (i as f32 / denom + self.phase).clamp(0.0, 1.0);

        let segments = (self.colors.len() - 1) as f32;
        let scaled = t * segments;
        let seg = (scaled.floor() as usize).min(self.colors.len() - 2);
        lerp(self.colors[seg], self.colors[seg + 1], scaled - seg as f32)
    }
}

fn lerp(a: Rgb, b: Rgb, t: f32) -> Rgb {
    let mix = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    let r = mix(a.r(), b.r());
    let g = mix(a.g(), b.g());
    let b = mix(a.b(), b.b());
    Rgb::of(((r as u32) << 16) | ((g as u32) << 8) | b as u32)
}

struct Entry {
    name: String,
    fmt: TextFormat,
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
    stack: Vec<Entry>,
    buffer: String,
    runs: Vec<TextComponent>,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
            stack: Vec::new(),
            buffer: String::new(),
            runs: Vec::new(),
        }
    }

    fn run(mut self) -> TextComponent {
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            self.pos += 1;
            match c {
                '\\' => {
                    if let Some(&next) = self.chars.get(self.pos) {
                        if next == '<' || next == '\\' {
                            self.buffer.push(next);
                            self.pos += 1;
                            continue;
                        }
                    }
                    self.buffer.push('\\');
                }
                '<' => self.read_tag(),
                _ => self.buffer.push(c),
            }
        }
        self.flush();

        let mut root = TextComponent::EMPTY;
        root = root.children(self.runs);
        root
    }

    fn active_gradient(&self) -> Option<&Gradient> {
        for entry in self.stack.iter().rev() {
            match &entry.fmt {
                TextFormat::Gradient(g) => return Some(g),
                TextFormat::Color(_) => return None,
                _ => {}
            }
        }
        None
    }

    fn flush(&mut self) {
        if self.buffer.is_empty() {
            return;
        }
        let text = std::mem::take(&mut self.buffer);
        let mut out = Vec::new();
        match self.active_gradient() {
            Some(gradient) => {
                for ch in text.chars() {
                    let mut component = TextComponent::text(ch.to_string());
                    let style = component.style_mut();
                    for entry in &self.stack {
                        entry.fmt.apply(style);
                    }
                    style.set_color(gradient.next_color());
                    out.push(component);
                }
            }
            None => {
                let mut component = TextComponent::text(text);
                let style = component.style_mut();
                for entry in &self.stack {
                    entry.fmt.apply(style);
                }
                out.push(component);
            }
        }
        self.runs.extend(out);
    }

    fn emit_object(&mut self, atlas: &str, sprite: &str) {
        self.flush();
        let mut component = TextComponent::object(atlas, sprite);
        {
            let style = component.style_mut();
            for entry in &self.stack {
                entry.fmt.apply(style);
            }
            if let Some(gradient) = self.active_gradient() {
                style.set_color(gradient.next_color());
            }
        }
        self.runs.push(component);
    }

    fn read_tag(&mut self) {
        let mut body = String::new();
        let mut closed = false;
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            self.pos += 1;
            if c == '>' {
                closed = true;
                break;
            }
            body.push(c);
        }

        // Unterminated tag: treat as literal text.
        if !closed {
            self.buffer.push('<');
            self.buffer.push_str(&body);
            return;
        }

        if let Some(rest) = body.strip_prefix('/') {
            self.close_tag(rest);
        } else {
            self.open_tag(&body);
        }
    }

    fn gradient_len(&self) -> usize {
        let mut i = self.pos;
        let mut depth = 1usize;
        let mut count = 0usize;
        while i < self.chars.len() {
            let c = self.chars[i];
            if c == '\\' {
                match self.chars.get(i + 1) {
                    Some('<') | Some('\\') => {
                        count += 1;
                        i += 2;
                        continue;
                    }
                    _ => {}
                }
                count += 1;
                i += 1;
                continue;
            }
            if c == '<' {
                let mut j = i + 1;
                let mut body = String::new();
                while j < self.chars.len() && self.chars[j] != '>' {
                    body.push(self.chars[j]);
                    j += 1;
                }
                let closing = body.starts_with('/');
                let name = body
                    .trim_start_matches('/')
                    .split(':')
                    .next()
                    .unwrap_or("")
                    .to_ascii_lowercase();
                match (closing, name.as_str()) {
                    (false, "gradient") => depth += 1,
                    (true, "gradient") => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    (false, "newline" | "br" | "object") => count += 1,
                    _ => {}
                }
                i = if j < self.chars.len() { j + 1 } else { j };
                continue;
            }
            count += 1;
            i += 1;
        }
        count
    }

    fn close_tag(&mut self, body: &str) {
        let name = body
            .split(':')
            .next()
            .unwrap_or("")
            .trim_start_matches('!')
            .to_ascii_lowercase();

        self.flush();

        if name.is_empty() {
            self.stack.pop();
            return;
        }

        if let Some(pos) = self.stack.iter().rposition(|e| e.name == name) {
            self.stack.remove(pos);
        }
    }

    fn open_tag(&mut self, body: &str) {
        let tokens = tokenize(body);
        let raw_name = tokens[0].as_str();
        let name = raw_name.to_ascii_lowercase();
        let args: Vec<&str> = tokens[1..].iter().map(String::as_str).collect();

        if name == "reset" {
            self.flush();
            self.stack.clear();
            return;
        }

        if name == "newline" || name == "br" {
            self.buffer.push('\n');
            return;
        }

        if name == "object" {
            if let [atlas, sprite, ..] = args.as_slice() {
                self.emit_object(atlas, sprite);
                return;
            }
        }

        if name == "gradient" {
            self.flush();
            let len = self.gradient_len();
            let gradient = parse_gradient(&args, len);
            self.stack.push(Entry {
                name,
                fmt: TextFormat::Gradient(gradient),
            });
            return;
        }

        match resolve(&name, &args) {
            Some(fmt) => {
                self.flush();
                self.stack.push(Entry { name, fmt });
            }
            None => {
                self.buffer.push('<');
                self.buffer.push_str(body);
                self.buffer.push('>');
            }
        }
    }
}

fn resolve(name: &str, args: &[&str]) -> Option<TextFormat> {
    let (deco_name, value) = match name.strip_prefix('!') {
        Some(rest) => (rest, false),
        None => (name, true),
    };
    match deco_name {
        "bold" | "b" => return Some(TextFormat::Bold(value)),
        "italic" | "i" | "em" => return Some(TextFormat::Italic(value)),
        "underlined" | "u" => return Some(TextFormat::Underlined(value)),
        "strikethrough" | "st" => return Some(TextFormat::Strikethrough(value)),
        "obfuscated" | "obf" => return Some(TextFormat::Obfuscated(value)),
        _ => {}
    }

    match name {
        "color" | "colour" | "c" => args
            .first()
            .and_then(|a| parse_color(a))
            .map(TextFormat::Color),
        "font" if !args.is_empty() => Some(TextFormat::Font(args.join(":"))),
        "insertion" if !args.is_empty() => Some(TextFormat::Insertion(args.join(":"))),
        "shadow" => args
            .first()
            .and_then(|a| parse_shadow(a))
            .map(TextFormat::Shadow),
        "click" => parse_click(args).map(TextFormat::Click),
        "hover" => parse_hover(args).map(TextFormat::Hover),
        _ => parse_color(name).map(TextFormat::Color),
    }
}

fn parse_color(name: &str) -> Option<Rgb> {
    if let Some(hex) = name.strip_prefix('#') {
        if hex.len() == 6 {
            if let Ok(v) = u32::from_str_radix(hex, 16) {
                return Some(Rgb::of(v));
            }
        }
        return None;
    }

    let normalized = match name {
        "grey" => "gray",
        "dark_grey" => "dark_gray",
        other => other,
    };
    NamedColor::try_from(normalized).ok().map(Rgb::from)
}

fn parse_shadow(value: &str) -> Option<Rgba> {
    if value == "none" || value == "transparent" {
        return Some(Rgba::of(0));
    }

    if let Some(hex) = value.strip_prefix('#') {
        if hex.len() == 8 {
            let rgba = u32::from_str_radix(hex, 16).ok()?;
            let packed = (rgba << 24) | (rgba >> 8);
            return Some(Rgba::of(packed));
        }
    }

    parse_color(value).map(Rgba::from)
}

fn parse_gradient(args: &[&str], len: usize) -> Gradient {
    let mut colors = Vec::new();
    let mut phase = 0.0;
    for arg in args {
        if let Some(color) = parse_color(arg) {
            colors.push(color);
        } else if let Ok(p) = arg.parse::<f32>() {
            phase = p;
        }
    }
    if colors.is_empty() {
        colors = vec![Rgb::of(0x000000), Rgb::of(0xffffff)];
    }
    Gradient {
        colors,
        phase,
        len,
        index: Cell::new(0),
    }
}

fn parse_click(args: &[&str]) -> Option<ClickEvent> {
    let action = args.first()?;
    let value = args.get(1).copied().unwrap_or("");
    match *action {
        "open_url" => Some(ClickEvent::open_url(value)),
        "run_command" => Some(ClickEvent::run_command(value)),
        "suggest_command" => Some(ClickEvent::suggest_command(value)),
        "copy_to_clipboard" => Some(ClickEvent::copy_to_clipboard(value)),
        "change_page" => value.parse().ok().map(ClickEvent::change_page),
        _ => None,
    }
}

fn parse_hover(args: &[&str]) -> Option<HoverEvent> {
    let action = args.first()?;
    match *action {
        "show_text" => {
            let value = args.get(1).copied().unwrap_or("");
            Some(HoverEvent::show_text(MiniMessage::parse(value)))
        }
        _ => None,
    }
}

fn tokenize(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut in_quote = false;

    for c in body.chars() {
        match c {
            '\'' => in_quote = !in_quote,
            ':' if !in_quote => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

#[macro_export]
#[cfg(feature = "minimessage")]
macro_rules! mm {
    ($($arg:tt)*) => {
        $crate::text::minimessage::MiniMessage::parse(&::std::format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::NamedColor;

    fn root(children: Vec<TextComponent>) -> TextComponent {
        TextComponent::EMPTY.children(children)
    }

    #[test]
    fn plain_text() {
        assert_eq!(
            MiniMessage::parse("hello"),
            root(vec![TextComponent::text("hello")])
        );
    }

    #[test]
    fn colored_and_decorated() {
        let parsed = MiniMessage::parse("<blue>Blue</blue> <bold>bold</bold>");
        let expected = root(vec![
            TextComponent::text("Blue").color(NamedColor::Blue),
            TextComponent::text(" "),
            TextComponent::text("bold").bold(),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn nested_styles_inherit() {
        let parsed = MiniMessage::parse("<red><bold>x</bold></red>");
        let expected = root(vec![TextComponent::text("x").color(NamedColor::Red).bold()]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn hex_color() {
        let parsed = MiniMessage::parse("<#ff5555>x");
        let expected = root(vec![TextComponent::text("x").color(Rgb::of(0xff5555))]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn object_sprite() {
        let parsed = MiniMessage::parse("<object:items:item/emerald>");
        let expected = root(vec![TextComponent::object("items", "item/emerald")]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn object_quoted_namespaced_atlas() {
        let parsed = MiniMessage::parse("<object:'minecraft:items':'item/emerald'>");
        let expected = root(vec![TextComponent::object(
            "minecraft:items",
            "item/emerald",
        )]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn font_namespaced() {
        let parsed = MiniMessage::parse("<font:minecraft:uniform>x</font>");
        let expected = root(vec![TextComponent::text("x").font("minecraft:uniform")]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn shadow_hex_with_alpha() {
        let parsed = MiniMessage::parse("<shadow:#ff000080>x");
        let expected = root(vec![
            TextComponent::text("x").shadow_color(Rgba::of(0x80ff0000)),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn shadow_none() {
        let parsed = MiniMessage::parse("<shadow:none>x");
        let expected = root(vec![TextComponent::text("x").shadow_color(Rgba::of(0))]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn gradient_endpoints() {
        let parsed = MiniMessage::parse("<gradient:#ff0000:#0000ff>AB</gradient>");
        let expected = root(vec![
            TextComponent::text("A").color(Rgb::of(0xff0000)),
            TextComponent::text("B").color(Rgb::of(0x0000ff)),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn gradient_midpoint() {
        let parsed = MiniMessage::parse("<gradient:#000000:#ffffff>ABC</gradient>");
        let expected = root(vec![
            TextComponent::text("A").color(Rgb::of(0x000000)),
            TextComponent::text("B").color(Rgb::of(0x808080)),
            TextComponent::text("C").color(Rgb::of(0xffffff)),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn gradient_continuous_across_nested_style() {
        let parsed = MiniMessage::parse("<gradient:#000000:#ffffff>A<b>B</b>C</gradient>");
        let expected = root(vec![
            TextComponent::text("A").color(Rgb::of(0x000000)),
            TextComponent::text("B").color(Rgb::of(0x808080)).bold(),
            TextComponent::text("C").color(Rgb::of(0xffffff)),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn gradient_named_colors() {
        let parsed = MiniMessage::parse("<gradient:red:blue>XY</gradient>");
        let expected = root(vec![
            TextComponent::text("X").color(NamedColor::Red),
            TextComponent::text("Y").color(NamedColor::Blue),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn reset_clears_stack() {
        let parsed = MiniMessage::parse("<red><bold>a<reset>b");
        let expected = root(vec![
            TextComponent::text("a").color(NamedColor::Red).bold(),
            TextComponent::text("b"),
        ]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn unknown_tag_is_literal() {
        let parsed = MiniMessage::parse("<notatag>x");
        let expected = root(vec![TextComponent::text("<notatag>x")]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn escaped_angle_bracket() {
        let parsed = MiniMessage::parse("\\<red>");
        let expected = root(vec![TextComponent::text("<red>")]);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn macro_interpolates() {
        let name = "Steve";
        let parsed = mm!("<blue>{name}</blue>");
        let expected = root(vec![TextComponent::text("Steve").color(NamedColor::Blue)]);
        assert_eq!(parsed, expected);
    }
}
