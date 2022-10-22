use std::{collections::HashMap, fs, ops::{Div, Rem}};

use bmp::{Image, Pixel, consts::WHITE};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct HPixel {
    r: u8,
    g: u8,
    b: u8
}
impl Into<Pixel> for HPixel {
    fn into(self) -> Pixel {
        Pixel { r: self.r, g: self.g, b: self.b }
    }
}
impl From<Pixel> for HPixel {
    fn from(value: Pixel) -> Self {
        HPixel { r: value.r, g: value.g, b: value.b }
    }
}
#[allow(dead_code)]
impl HPixel {
    fn from_packed(color_packed: u32) -> HPixel {
        HPixel { r: (color_packed % 256) as u8, g: ((color_packed / 256) % 256) as u8, b: ((color_packed / 65536) % 256) as u8 }
    }
    fn new(r: u8, g: u8, b: u8) -> HPixel {
        HPixel { r, g, b }
    }
    fn to_packed(&self) -> u32 {
        self.r as u32 + self.g as u32 * 256 + self.b as u32 * 32768
    }
}

fn hmify(p: &str) -> HashMap<HPixel, String> {
    let input: Vec<Vec<String>> = fs::read_to_string(p).expect(&format!("{p} not found!")[..]).split('\n').map(|st|st.split(',').map(|s|s.to_owned()).collect::<Vec<String>>()).collect();
    let mut hm: HashMap<HPixel, String> = HashMap::new();
    let mut l = 0;
    for i in &input {
        let col: &String = match i.get(0) {
            Some(s) => s,
            None => { println!("Found an empty line! ({l})"); continue; }
        };
        let name: &String = match i.get(1) {
            Some(s) => s,
            None => { println!("Color {col} has no name, skipping!"); continue; }
        };
        let color_packed: u32 = u32::from_str_radix(col, 16).expect(&format!("{col} not a valid hex code!"));
        let color: HPixel = HPixel::from_packed(color_packed);
        //println!("{:?}", color);
        hm.insert(color, name.clone());
        l += 1;
    }
    hm
}

fn mod_div<T: Div<Output = T> + Rem<Output = T> + Copy>(l: T, r: T) -> (T, T) {
    (l % r, l / r)
}

fn mod_div_s<T: Div<Output = T> + Rem<Output = T> + Copy>(l: T, rd: T, rm: T) -> (T, T) {
    (l % rd, l / rm)
}

fn coordify<T: Into<HPixel>>(col: T) -> (u32, u32) {
    let col: HPixel = col.into();
    let packed: u32 = col.to_packed();
    mod_div(packed, 4096)
    //(col.r as u32 + (col.b as u32 * 256) % 16, col.g as u32 + (col.b as u32 * 256) / 16)
}

fn main() {
    println!("Decoding file...");
    let colors: HashMap<HPixel, String> = hmify("colornames.txt");
    let mut img: Image = Image::new(4096, 4096);
    println!("LUT mapping...");
    for w in 0..16u32 {
        for h in 0..16u32 {
            for x in 0..256u32 {
                for y in 0..256u32 {
                    let col: HPixel = HPixel { r: x as u8, g: y as u8, b: (w + h * 16) as u8 };
                    if colors.contains_key(&col) {
                        img.set_pixel(x + w * 256, y + h * 256, WHITE);
                    } else {
                        img.set_pixel(x + w * 256, y + h * 256, col.into());
                    }
                }
            }
        }
    }
    img.save("all_colors.bmp").expect("Couldn't save the image!");
}
