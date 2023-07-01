use std::{
    sync::{Arc, RwLock},
    thread,
};

use image::RgbaImage;

use crate::Color;

pub fn get_free_spaces(canvas: RgbaImage, target_color: Color) -> u32 {
    let space = Arc::new(RwLock::new(0u32));

    let canvas = Arc::new(RwLock::new(canvas));

    let t_color = target_color.clone();
    let t_canvas = canvas.clone();
    let _t_tx = space.clone();
    let t1 = thread::spawn(move || {
        let color_threshold = 100;
        let canvas = t_canvas.read().unwrap();
        let min_x = canvas.width() / 2;
        let max_x = canvas.width() - 1;
        for x in min_x..max_x {
            for y in 0..canvas.height() {
                let pixel = canvas.get_pixel(x, y);

                let color_data = Color::from_slice(&pixel.0);

                if color_data.is_similar(&t_color, color_threshold) {
                    *_t_tx.write().unwrap() += 1;
                }
            }
        }
    });

    let t_color = target_color.clone();
    let t_canvas = canvas.clone();
    let _t_tx = space.clone();
    let t2 = thread::spawn(move || {
        let color_threshold = 100;
        let canvas = t_canvas.read().unwrap();
        let min_x = 0;
        let max_x = canvas.width() / 2;
        for x in min_x..max_x {
            for y in 0..canvas.height() {
                let pixel = canvas.get_pixel(x, y);

                let color_data = Color::from_slice(&pixel.0);

                if color_data.is_similar(&t_color, color_threshold) {
                    *_t_tx.write().unwrap() += 1;
                }
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let space = space.read().unwrap();
    let space = *space;

    drop(space);

    space
}
