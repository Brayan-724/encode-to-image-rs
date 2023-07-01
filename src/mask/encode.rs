use image::Rgba;
use std::{collections::HashSet, fs::File, path::Path};

use super::common::get_free_spaces;
use crate::{encode::encode_bytes, Color};

pub fn calculate_next_size(
    remain: i32,
    size: u32,
    old_size: f32,
    data_len: f32,
    space: f32,
) -> u32 {
    match remain {
        -10_000..=10_000 => {
            let is_neg = remain.is_negative();
            let modifier = (remain as f32).abs().log(2.7182818284590452353602874713527);
            let modifier = if is_neg { modifier } else { -modifier };
            (size as f32 + modifier.round()).abs().round() as u32
        }
        ..=-10_001 => size + (remain as f32).abs().sqrt().floor() as u32 / 2,
        _ => (old_size * data_len.sqrt().floor() / space.sqrt().floor())
            .round()
            .abs() as u32,
    }
}

pub fn encode_to_mask<M, O>(
    data: &[u8],
    target_color: Color,
    fake_color: Option<Color>,
    mask: M,
    output_path: O,
) where
    M: AsRef<Path>,
    O: AsRef<Path>,
{
    let color_threshold = 100;
    let fake_color = match fake_color {
        Some(c) => c,
        None => target_color.clone(),
    };
    let encoded = encode_bytes(data, fake_color.clone());

    let image = image::io::Reader::open(mask).unwrap();
    let image = image.decode().unwrap();
    let mut tmp_image = image.clone();
    let aspect_ratio = image.height() as f32 / image.width() as f32;

    let mut size: u32 = image.width();
    let mut remain = i32::MAX;
    let mut results: HashSet<u32> = HashSet::new();

    for iter in 0..10 {
        println!(" --- Iter {}", iter);
        println!("Width: {}", size);
        println!("Height: {}", (size as f32 * aspect_ratio).round());

        let space = get_free_spaces(tmp_image.to_rgba8(), target_color.clone());

        println!("Free pixels in mask: {}", space);

        let data_len = encoded.len() as f32;
        let space = space as f32;
        let old_size = size as f32;
        remain = (space - data_len) as i32;

        if remain == 0 {
            break;
        }

        println!("Remain data: {}", remain);

        // Calculate size to fit data into free cells
        size = calculate_next_size(remain, size, old_size, data_len, space);

        tmp_image = image.resize_exact(
            size,
            (size as f32 * aspect_ratio).round() as u32,
            image::imageops::FilterType::Triangle,
        );

        if results.contains(&size) {
            let space = get_free_spaces(tmp_image.to_rgba8(), target_color.clone());
            remain = (space as f32 - data_len) as i32;

            println!(" --- NEXT");
            println!("Size: {}", size);
            println!("Free pixels in mask: {}", space);
            println!("Remain data: {}", remain);
            break;
        }

        // Prevent loops
        results.insert(size);
    }

    if remain < 0 {
        size = calculate_next_size(remain, size, 0.0, 0.0, 0.0);
        // size = size + ((remain as f32).abs().sqrt() / 4f32).ceil() as u32;
        tmp_image = image.resize_exact(
            size,
            (size as f32 * aspect_ratio).round() as u32,
            image::imageops::FilterType::Triangle,
        );
        let space = get_free_spaces(tmp_image.to_rgba8(), target_color.clone());
        remain = (space as f32 - encoded.len() as f32) as i32;
    }

    let image = tmp_image;

    println!(" -- Matched");
    println!(" Size: {size}");
    println!(" Remain: {remain}");

    // Save the scaled mask
    let ref mut fout = File::create("test_mask.png").unwrap();
    image.write_to(fout, image::ImageFormat::Png).unwrap();

    let mut image = image.to_rgba8();

    let mut idx = 0;
    for pixel in image.pixels_mut() {
        let color = Color::from_slice(&pixel.0);

        if color.is_similar(&target_color, color_threshold) {
            *pixel = encoded
                .get(idx)
                .unwrap_or(&Color::new_alpha(
                    fake_color.0,
                    fake_color.1,
                    fake_color.2,
                    240,
                ))
                .to_rgba_pixel();
            idx += 1;
        } else {
            let new_pixel = pixel.0;
            let alpha = if new_pixel[3] > 240 {
                240
            } else {
                new_pixel[3]
            };
            let new_pixel = Rgba([new_pixel[0], new_pixel[1], new_pixel[2], alpha]);
            *pixel = new_pixel;
        }
    }

    let ref mut fout = File::create(output_path).unwrap();
    image.write_to(fout, image::ImageFormat::Png).unwrap();
}
