use std::{fs::File, path::Path};

use image::RgbaImage;

use crate::{encode::encode_bytes, Color};

pub fn encode_to_image<O>(data: &[u8], target_color: Color, output_path: O)
where
    O: AsRef<Path>,
{
    let encoded = encode_bytes(data, target_color);

    let size = f32::sqrt(encoded.len() as f32).ceil() as u32;

    let mut image = RgbaImage::new(size, size);

    let last_pixel = encoded.len() - 1;
    let last_pixel = (last_pixel as u32) % size;

    if last_pixel < size {
        for i in last_pixel..size {
            image.put_pixel(i, size - 1, image::Rgba([0, 0, 0, 255]));
        }
    }

    for (i, c) in encoded.iter().enumerate() {
        let x = (i as u32) % size;
        let y = (i as u32) / size;

        image.put_pixel(x, y, c.to_rgba_pixel());
    }

    let ref mut fout = File::create(output_path).unwrap();
    image.write_to(fout, image::ImageFormat::Png).unwrap();
}
