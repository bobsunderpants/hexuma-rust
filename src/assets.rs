//! Loaders for the original Hexuma asset formats.
//!
//! Pictures (`BILDnnn.SEQ`) are stored as
//!   u16 width, u16 height, width*height palette indices, 256*4 byte palette
//! for normal pictures (12 bytes of header: the two sizes plus eight bytes of
//! metadata), and as a 4-byte header, 320*200 indices and a palette for the few
//! full-screen pictures.

use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
pub struct Image {
    pub w: usize,
    pub h: usize,
    pub rgb: Vec<u32>,
}

impl Image {
    pub fn pixel(&self, x: usize, y: usize) -> u32 {
        if x >= self.w || y >= self.h {
            0
        } else {
            self.rgb[y * self.w + x]
        }
    }
}

fn palette_rgb(pal: &[u8], index: u8) -> u32 {
    let i = index as usize * 4;
    if i + 2 >= pal.len() {
        return 0;
    }
    ((pal[i] as u32) << 16) | ((pal[i + 1] as u32) << 8) | pal[i + 2] as u32
}

pub fn load_seq(path: &str) -> Result<Image, String> {
    let data = fs::read(path).map_err(|e| format!("{path}: {e}"))?;
    let n = data.len();

    let (w, h, pixel_start) = if n == 65028 {
        // Full-screen 320x200 picture: four header bytes.
        (320usize, 200usize, 4usize)
    } else if n >= 14 {
        let w = u16::from_le_bytes([data[0], data[1]]) as usize;
        let h = u16::from_le_bytes([data[2], data[3]]) as usize;
        if w == 0 || h == 0 || w > 2000 || h > 1000 {
            return Err(format!("{path}: implausible picture size {w}x{h}"));
        }
        (w, h, 12usize)
    } else {
        return Err(format!("{path}: file too small"));
    };

    if n < pixel_start + w * h + 1024 {
        return Err(format!("{path}: truncated ({n} bytes, expected {})", pixel_start + w * h + 1024));
    }

    let pixels = &data[pixel_start..pixel_start + w * h];
    let pal = &data[n - 1024..];

    let mut rgb = Vec::with_capacity(w * h);
    for &p in pixels {
        rgb.push(palette_rgb(pal, p));
    }
    Ok(Image { w, h, rgb })
}

/// Reads BILDDAT.SEQ, a CR/LF separated list of picture numbers indexed by
/// room number (1 based).  A zero means "no picture".
pub fn load_bilddat(path: &str) -> HashMap<u16, u16> {
    let mut map = HashMap::new();
    let Ok(data) = fs::read(path) else { return map };
    let text = String::from_utf8_lossy(&data);
    let mut idx: u16 = 1;
    for line in text.split(['\r', '\n']) {
        let line = line.trim_matches('\u{1a}').trim();
        if line.is_empty() {
            continue;
        }
        if let Ok(n) = line.parse::<u16>() {
            if n != 0 {
                map.insert(idx, n);
            }
            idx += 1;
        }
    }
    map
}

pub fn load_image_set(dir: &str, map: &HashMap<u16, u16>) -> HashMap<u16, Image> {
    let mut out = HashMap::new();
    for &n in map.values() {
        if out.contains_key(&n) {
            continue;
        }
        let path = format!("{dir}/BILD{n:03}.SEQ");
        match load_seq(&path) {
            Ok(img) => {
                out.insert(n, img);
            }
            Err(e) => eprintln!("warning: {e}"),
        }
    }
    out
}
