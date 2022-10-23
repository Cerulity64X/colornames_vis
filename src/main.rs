use std::{collections::HashMap, fs, ops::{Div, Rem}, env::{Args, args}, fmt::Display};

use bmp::{Image, Pixel, consts::{WHITE, RED, BLACK, LIME}};

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
impl Display for HPixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:X}{:X}{:X}", self.r, self.g, self.b)
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
        let color_packed: u32 = match u32::from_str_radix(col, 16) {
            Ok(i) => i,
            Err(_) => {
                println!("{col} not a valid hex code!");
                continue;
            }
        };
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
    (col.r as u32 + (col.b as u32 * 256) % 16, col.g as u32 + (col.b as u32 * 256) / 16)
}

fn main() {
    let mut argv: Args = args();
    argv.next();
    match &argv.next().expect("No instruction provided!")[..] {
        "gen" => {
            println!("Decoding file...");
            let colors: HashMap<HPixel, String> = hmify(&argv.next().expect("No input file provided!")[..]);
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
            img.save("all_colors.bmp").expect("Could not save output!");
        }
        "diff" => {
            println!("Decoding first file...");
            let colors: HashMap<HPixel, String> = hmify(&argv.next().expect("No first input file provided!")[..]);
            println!("Decoding second file...");
            let s_colors: HashMap<HPixel, String> = hmify(&argv.next().expect("No second input file provided!")[..]);
            let lut_bg: bool = match argv.next() {
                Some(s) => match &s[..] {
                    "black_bg" => {
                        false
                    }
                    "lut_bg" => {
                        true
                    }
                    f => {
                        println!("Unrecognized format {f} (black_bg, lut_bg are correct), defaulting to showing LUT background.");
                        true
                    }
                }
                None => {
                    println!("Defaulting to showing LUT background.");
                    true
                }
            };
            let print_diff: bool = match argv.next() {
                Some(s) => match &s[..] {
                    "print_diff" => {
                        true
                    }
                    "no_print" => {
                        false
                    }
                    f => {
                        println!("Unrecognized format {f} (print_diff, no_print are correct), defaulting to not printing.");
                        false
                    }
                }
                None => {
                    println!("Defaulting to not printing.");
                    false
                }
            };
            let only_diff: bool = match argv.next() {
                Some(s) => match &s[..] {
                    "only_diff" => {
                        true
                    }
                    "show_both" => {
                        false
                    }
                    f => {
                        println!("Unrecognized format {f} (only_diff, show_both are correct), defaulting to showing both.");
                        false
                    }
                }
                None => {
                    println!("Defaulting to showing both.");
                    false
                }
            };
            let mut img: Image = Image::new(4096, 4096);
            println!("LUT mapping...");
            for w in 0..16u32 {
                for h in 0..16u32 {
                    for x in 0..256u32 {
                        for y in 0..256u32 {
                            let col: HPixel = HPixel { r: x as u8, g: y as u8, b: (w + h * 16) as u8 };
                            let xpos: u32 = x + w * 256;
                            let ypos: u32 = y + h * 256;
                            let first_contains: bool = colors.contains_key(&col);
                            let second_contains: bool = s_colors.contains_key(&col);
                            if first_contains && second_contains {
                                // color is in both
                                img.set_pixel(xpos, ypos, if only_diff {BLACK} else {WHITE});
                            } else if !first_contains && !second_contains {
                                // color is in neither
                                img.set_pixel(xpos, ypos, if lut_bg {col.into()} else {BLACK});
                            } else if !first_contains && second_contains {
                                // color was added
                                img.set_pixel(xpos, ypos, LIME);
                                if print_diff {
                                    println!("Color {} is new.", col);
                                }
                            } else if first_contains && !second_contains {
                                // color was removed
                                img.set_pixel(xpos, ypos, RED);
                                if print_diff {
                                    println!("Color {} was removed.", col);
                                }
                            }
                        }
                    }
                }
            }
            img.save("diff_colors.bmp").expect("Couldn't save the image!");
        }
        "help" => {
            println!(r"
Cerulity32K's Colornames.org Visualizer

help: Shows this screen.

gen [database]: Generates a bmp file with the specified colornames.org database.

diff [first] [second] [lut_bg|<any>|<none>] [print_diff|<any>|<none>] [only_diff|<any>|<none>]: Differentiates two colornames.org CSV databases with the second database (supposedly) newer than the first.
Green pixels appear for an added color, red for a removed color, white for colors in both databases, black/LUT val for colors in neither.
You can supply a third argument that defines whether an LUT should be placed for colors in neither database, or a black background. Defaults to an LUT background.
A fourth argument can be provided to define if new/removed values should be printed. Defaults to no printing.
Another fifth argument can be specified to define if only the new values should be shown (values in both are set to black).

detail [database] [color]: Get's the details of a specified color (location on LUT, name of color).");
        }
        "detail" => {
            let col: String = argv.next().expect("No color provided!");
            println!("Decoding file...");
            let colors: HashMap<HPixel, String> = hmify(&argv.next().expect("No input file provided!")[..]);
            let color_packed: u32 = u32::from_str_radix(&col, 16).expect(&format!("{col} is not a valid hex code!"));
            let color: HPixel = HPixel::from_packed(color_packed);
            let (x, y) = coordify(color);
            print!("The color {col} is located at X: {x}, Y: {y}");
            if let Some(c) = colors.get(&color) {
                println!(", named \"{c}\".");
            } else {
                println!(".");
            }
        }
        _ => {}
    }
}
