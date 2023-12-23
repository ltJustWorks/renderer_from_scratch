use tgaimage::{TGAImage, TGAColor, TGAColorRGBA};

pub fn convert_buf(image: &TGAImage) -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::new();

    for y in 0..image.height() {
        for x in 0..image.width() {
            let rgba: TGAColorRGBA = match image.get(x, y) {
                TGAColor::Rgb(rgb) => TGAColorRGBA{r: rgb.r, g: rgb.g, b: rgb.b, a: 255},
                TGAColor::Rgba(rgba) => rgba,
            };
            let pixel_value = ((rgba.a as u32) << 24) | ((rgba.r as u32) << 16) | ((rgba.g as u32) << 8) | (rgba.b as u32);
            buffer.push(pixel_value);
        }
    }

    buffer
}