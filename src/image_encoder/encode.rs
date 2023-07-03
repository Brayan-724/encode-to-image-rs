use std::{fs::File, path::Path};

use image::RgbaImage;

use crate::{encode::encode_bytes, Color};

pub fn encode_to_image<O>(data: &[u8], target_color: Color, output_path: O)
where
    O: AsRef<Path>,
{
    let encoded = encode_bytes(data, target_color);

    let size = (encoded.len() as f32).sqrt().ceil() as u32;

    let mut image = RgbaImage::new(size, size);

    let last_pixel = encoded.len() as u32 - 1;
    let last_pixel_x = last_pixel % size;
    let last_pixel_y = last_pixel / size;

    match (last_pixel_x < size, last_pixel_y < size) {
        (false, false) => (),
        (true, false) => {
            for i in last_pixel_x..size {
                image.put_pixel(i, size - 1, image::Rgba([0, 0, 0, 255]));
            }
        }
        (false, true) => {
            for i in 0..size {
                image.put_pixel(i, size - 1, image::Rgba([0, 0, 0, 255]));
            }
        }
        (true, true) => {
            for i in last_pixel_x..size {
                image.put_pixel(i, size - 2, image::Rgba([0, 0, 0, 255]));
                image.put_pixel(i, size - 1, image::Rgba([0, 0, 0, 255]));
            }
            for i in 0..last_pixel_x {
                image.put_pixel(i, size - 1, image::Rgba([0, 0, 0, 255]));
            }
        }
    };

    for (i, c) in encoded.iter().enumerate() {
        let x = (i as u32) % size;
        let y = (i as u32) / size;

        image.put_pixel(x, y, c.to_rgba_pixel());
    }

    let ref mut fout = File::create(output_path).unwrap();
    image.write_to(fout, image::ImageFormat::Png).unwrap();
}
