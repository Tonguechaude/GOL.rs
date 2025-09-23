use parse_rle_macro::generate_pattern_functions;
use std::sync::OnceLock;

macro_rules! pattern {
    // Macro inline
    (inline $rle:literal) => {{
        const RLE: &str = $rle;
        static CELLS: OnceLock<Vec<(i32, i32)>> = OnceLock::new();
        CELLS.get_or_init(|| parse_rle(RLE))
    }};

    // Macro from file
    (file $path:literal) => {{
        const RLE: &str = include_str!($path);
        static CELLS: OnceLock<Vec<(i32, i32)>> = OnceLock::new();
        CELLS.get_or_init(|| parse_rle(RLE))
    }};
}

fn parse_rle(rle: &str) -> Vec<(i32, i32)> {
    let mut cells = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut num = 0;

    for byte in rle.bytes() {
        match byte {
            // Number of iteration
            b'0'..=b'9' => num = num * 10 + (byte - b'0') as i32,
            b'b' | b'.' => { // Cell is dead
                x += num.max(1);
                num = 0;
            }
            b'o' => { // Cell living
                let count = num.max(1);
                for i in 0..count {
                    cells.push((x + i, y));
                }
                x += count;
                num = 0;
            }
            b'$' => { // EOL
                y += num.max(1);
                x = 0;
                num = 0;
            }
            b'!' => break, // EOF
            _ => {}
        }
    }
    cells
}

pub struct Patterns;

impl Patterns {
    generate_pattern_functions!("assets");

    /// Parse RLE from string content (for dynamic loading)
    pub fn from_rle_string(rle_content: &str) -> Vec<(i32, i32)> {
        parse_rle(rle_content)
    }
}

