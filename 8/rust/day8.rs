use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

const IMG_WIDTH: usize = 25;
const IMG_HEIGHT: usize = 6;
const IMG_PIXEL_COUNT: usize = IMG_WIDTH * IMG_HEIGHT;

type Layer = Vec<Vec<u8>>;
type Image = Vec<Layer>;

fn get_digit_hist(layer: &Layer) -> HashMap<u8, usize> {
    let mut digit_hist: HashMap<u8, usize> = HashMap::new();
    assert!(layer.len() == IMG_HEIGHT);
    for y in 0..IMG_HEIGHT {
        assert!(layer[y].len() == IMG_WIDTH);
        for x in 0..IMG_WIDTH {
            let digit = layer[y][x];
            let entry = digit_hist.entry(digit).or_insert(0);
            *entry += 1;
        }
    }
    return digit_hist;
}

fn compose_pixel(image: &Image, x: usize, y: usize) -> u8 {
    for layer in 0..image.len() {
        let layer_pixel = image[layer][y][x];
        if layer_pixel != 2 {
            return layer_pixel;
        }
    }
    return 2;
}

fn compose_layers(image: &Image) -> Layer {
    let mut composed = Layer::new();
    for y in 0..IMG_HEIGHT {
        composed.push(Vec::new());
        for x in 0..IMG_WIDTH {
            composed[y].push(compose_pixel(&image, x, y));
        }
    }
    return composed;
}

fn load_image(filename: &str) -> Image {
    let file = BufReader::new(File::open(filename).expect("open failed"));
    let mut digits: Vec<u8> = Vec::new();
    for line in file.lines() {
        for c in line.expect("lines failed").chars() {
            digits.push(c.to_digit(10).expect("to_digit failed") as u8);
        }
    }

    let layer_count = digits.len() / IMG_PIXEL_COUNT;
    assert!(digits.len() == IMG_PIXEL_COUNT * layer_count);
    let mut image: Image = Vec::new();
    let mut index = 0;
    for layer in 0..layer_count {
        image.push(Layer::new());
        for y in 0..IMG_HEIGHT {
            image[layer].push(Vec::new());
            for _x in 0..IMG_WIDTH {
                image[layer][y].push(digits[index]);
                index += 1;
            }
        }
    }
    return image;
}

fn main() {
    let image = load_image("../input");

    let mut min_zeros = IMG_PIXEL_COUNT;
    let mut min_zeros_checksum = 0;
    for layer in 0..image.len() {
        let hist = get_digit_hist(&image[layer]);
        if hist[&0] < min_zeros {
            min_zeros = hist[&0];
            min_zeros_checksum = hist[&1] * hist[&2];
        }
    }
    println!("Min zeros: {}. Checksum: {}", min_zeros, min_zeros_checksum);

    let composed = compose_layers(&image);
    for y in 0..IMG_HEIGHT {
        for x in 0..IMG_WIDTH {
            print!("{}", match composed[y][x] {
                0 => "\x1B[30mX\x1B[0m",
                1 => "\x1B[37mX\x1B[0m",
                _ => " "
            });
        }
        println!("");
    }
}