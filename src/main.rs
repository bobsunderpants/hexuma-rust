//! Hexuma: The Eye of Kal - an English remake of the 1992 German adventure.
//!
//! The window is split into a picture area (top) and a scrolling text log
//! (bottom).  Pictures are the original `BILDnnn.SEQ` files, looked up through
//! `BILDDAT.SEQ` by the original room number.

use hexuma::assets::{load_bilddat, load_image_set, Image};
use hexuma::game::{self, Game};

use font8x8::{UnicodeFonts, BASIC_FONTS};
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::collections::HashMap;

const WIDTH: usize = 960;
const HEIGHT: usize = 720;
const PICTURE_TOP: usize = 6;
const PICTURE_HEIGHT: usize = 396;
const PANEL_TOP: usize = 412;

const BG: u32 = 0x00_10_10_18;
const PANEL_BG: u32 = 0x00_08_08_10;
const TEXT: u32 = 0x00_d8_d8_c8;
const BRIGHT: u32 = 0x00_f0_e8_b0;
const FRAME: u32 = 0x00_30_30_40;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(|s| s.as_str()) == Some("--text") {
        run_text();
        return;
    }

    let asset_dir = args
        .first()
        .cloned()
        .unwrap_or_else(|| "assets".to_string());

    let map = load_bilddat(&format!("{asset_dir}/BILDDAT.SEQ"));
    let pictures = load_image_set(&asset_dir, &map);
    if pictures.is_empty() {
        eprintln!(
            "warning: no pictures loaded from '{asset_dir}'. \
             Run from the project directory or pass the asset directory as the first argument."
        );
    }

    let mut window = Window::new(
        "Hexuma: The Eye of Kal",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .expect("could not open a window");
    window.set_target_fps(60);

    let mut game = Game::new();
    let mut input = String::new();
    let mut buffer = vec![BG; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for key in window.get_keys_pressed(KeyRepeat::Yes) {
            match key {
                Key::Enter | Key::NumPadEnter => {
                    let line = input.trim().to_string();
                    input.clear();
                    if line.is_empty() {
                        continue;
                    }
                    game.print(&format!("> {line}"));
                    game.command(&line);
                }
                Key::Backspace => {
                    input.pop();
                }
                _ => {
                    if let Some(c) = key_char(key) {
                        if input.len() < 58 {
                            input.push(c);
                        }
                    }
                }
            }
        }

        buffer.fill(BG);
        draw_picture(&mut buffer, &pictures, &map, game.room);
        draw_panel(&mut buffer, &game, &input);

        let alive = window.update_with_buffer(&buffer, WIDTH, HEIGHT).is_ok();
        if !alive {
            break;
        }
        // Cheap idle so a static log does not burn a full core.
        std::thread::sleep(std::time::Duration::from_millis(8));
    }
}

/// Line-oriented mode for playtesting and for terminals without a display.
fn run_text() {
    use std::io::{BufRead, Write};
    let mut game = Game::new();
    let stdin = std::io::stdin();
    let mut out = std::io::stdout();
    let mut shown = 0usize;
    let mut flush = |game: &Game, shown: &mut usize| {
        for line in &game.log[*shown..] {
            let _ = writeln!(out, "{line}");
        }
        *shown = game.log.len();
        let _ = writeln!(out);
        let _ = out.flush();
    };
    flush(&game, &mut shown);
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        if line == "quit" || line == "exit" {
            break;
        }
        game.print(&format!("> {line}"));
        game.command(&line);
        flush(&game, &mut shown);
    }
}

fn key_char(key: Key) -> Option<char> {    Some(match key {
        Key::A => 'a',
        Key::B => 'b',
        Key::C => 'c',
        Key::D => 'd',
        Key::E => 'e',
        Key::F => 'f',
        Key::G => 'g',
        Key::H => 'h',
        Key::I => 'i',
        Key::J => 'j',
        Key::K => 'k',
        Key::L => 'l',
        Key::M => 'm',
        Key::N => 'n',
        Key::O => 'o',
        Key::P => 'p',
        Key::Q => 'q',
        Key::R => 'r',
        Key::S => 's',
        Key::T => 't',
        Key::U => 'u',
        Key::V => 'v',
        Key::W => 'w',
        Key::X => 'x',
        Key::Y => 'y',
        Key::Z => 'z',
        Key::Key0 | Key::NumPad0 => '0',
        Key::Key1 | Key::NumPad1 => '1',
        Key::Key2 | Key::NumPad2 => '2',
        Key::Key3 | Key::NumPad3 => '3',
        Key::Key4 | Key::NumPad4 => '4',
        Key::Key5 | Key::NumPad5 => '5',
        Key::Key6 | Key::NumPad6 => '6',
        Key::Key7 | Key::NumPad7 => '7',
        Key::Key8 | Key::NumPad8 => '8',
        Key::Key9 | Key::NumPad9 => '9',
        Key::Space => ' ',
        Key::Period | Key::NumPadDot => '.',
        Key::Comma => ',',
        Key::Minus | Key::NumPadMinus => '-',
        Key::Apostrophe => '\'',
        Key::Slash | Key::NumPadSlash => '/',
        Key::Semicolon => ';',
        Key::Equal | Key::NumPadPlus => '+',
        Key::LeftBracket => '[',
        Key::RightBracket => ']',
        Key::Backslash => '\\',
        _ => return None,
    })
}

fn draw_picture(
    buffer: &mut [u32],
    pictures: &HashMap<u16, Image>,
    map: &HashMap<u16, u16>,
    room: u16,
) {
    // Backdrop for the picture area.
    for y in PICTURE_TOP..(PICTURE_TOP + PICTURE_HEIGHT) {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = PANEL_BG;
        }
    }

    let Some(&num) = map.get(&room) else {
        draw_frame(buffer);
        return;
    };
    let Some(img) = pictures.get(&num) else {
        draw_frame(buffer);
        return;
    };
    if img.w == 0 || img.h == 0 {
        draw_frame(buffer);
        return;
    }

    // Integer scale that fits the picture area: prefer 3x, fall back to 2x/1x.
    let mut scale = 3;
    while scale > 1 && (img.w * scale > WIDTH - 8 || img.h * scale > PICTURE_HEIGHT - 8) {
        scale -= 1;
    }
    let dw = img.w * scale;
    let dh = img.h * scale;
    let ox = (WIDTH - dw) / 2;
    let oy = PICTURE_TOP + (PICTURE_HEIGHT - dh) / 2;

    for sy in 0..img.h {
        for sx in 0..img.w {
            let c = img.pixel(sx, sy);
            if c == 0 {
                continue;
            }
            for dy in 0..scale {
                let y = oy + sy * scale + dy;
                if y >= PICTURE_TOP + PICTURE_HEIGHT {
                    continue;
                }
                for dx in 0..scale {
                    let x = ox + sx * scale + dx;
                    if x < WIDTH {
                        buffer[y * WIDTH + x] = c;
                    }
                }
            }
        }
    }

    draw_frame(buffer);
}

fn draw_frame(buffer: &mut [u32]) {
    let top = PICTURE_TOP + PICTURE_HEIGHT;
    for x in 0..WIDTH {
        buffer[top * WIDTH + x] = FRAME;
        buffer[(top + 1) * WIDTH + x] = FRAME;
    }
}

fn draw_panel(buffer: &mut [u32], game: &Game, input: &str) {
    for y in PANEL_TOP..HEIGHT {
        for x in 0..WIDTH {
            buffer[y * WIDTH + x] = PANEL_BG;
        }
    }

    let scale = 2;
    let glyph = 8 * scale;
    let line_h = glyph;

    // Status line.
    let status = format!(
        "{}   |   Score {} / {}   |   Moves {}",
        game.room_name(game.room),
        game.score,
        game::MAX_SCORE,
        game.moves
    );
    draw_text(buffer, 8, PANEL_TOP + 6, &status, BRIGHT, scale);

    // Scrolling log: as many rows as fit above the input line.
    let log_top = PANEL_TOP + 6 + line_h + 4;
    let input_y = HEIGHT - line_h - 8;
    let rows = (input_y.saturating_sub(log_top)) / line_h;
    let total = game.log.len();
    let start = total.saturating_sub(rows);
    for (i, line) in game.log[start..].iter().enumerate() {
        let y = log_top + i * line_h;
        draw_text(buffer, 8, y, line, TEXT, scale);
    }

    // Input line.
    draw_text(buffer, 8, input_y, ">", BRIGHT, scale);
    draw_text(buffer, 8 + 2 * glyph, input_y, input, TEXT, scale);
    let cx = 8 + 2 * glyph + text_width(input) as usize * glyph;
    if (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
        / 500)
        % 2
        == 0
    {
        for y in input_y..(input_y + glyph).min(HEIGHT) {
            for x in cx..(cx + 2).min(WIDTH) {
                buffer[y * WIDTH + x] = TEXT;
            }
        }
    }
}

fn text_width(s: &str) -> u32 {
    s.chars().count() as u32
}

fn draw_text(buffer: &mut [u32], x: usize, y: usize, text: &str, color: u32, scale: usize) {
    let mut cx = x;
    for ch in text.chars() {
        if ch.is_ascii() {
            if let Some(glyph) = BASIC_FONTS.get(ch) {
                for (row, bits) in glyph.iter().enumerate() {
                    for col in 0..8 {
                        if bits & (1 << col) != 0 {
                            for dy in 0..scale {
                                let py = y + row * scale + dy;
                                if py >= HEIGHT {
                                    continue;
                                }
                                for dx in 0..scale {
                                    let px = cx + col * scale + dx;
                                    if px < WIDTH {
                                        buffer[py * WIDTH + px] = color;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        cx += 8 * scale;
        if cx >= WIDTH {
            break;
        }
    }
}
