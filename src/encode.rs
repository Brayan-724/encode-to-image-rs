use crate::{utils::ToHex, Color};

pub fn encode_bytes(bytes: &[u8], target_color: Color) -> Vec<Color> {
    let hex = bytes
        .iter()
        .map(|b| b.to_hex().chars().collect::<Vec<_>>())
        .flatten();

    let mut out: Vec<Color> = vec![];

    let mut carrier: [char; 6] = [' '; 6];
    let mut carrier_i: u8 = 0;

    for c in hex {
        if let Some(carrier_cell) = carrier.get_mut(carrier_i as usize) {
            *carrier_cell = 'F';
        }

        if let Some(carrier_cell) = carrier.get_mut(carrier_i as usize + 1) {
            *carrier_cell = c;
        }
        carrier_i += 2;

        if carrier_i >= 6 {
            let pixel_color: String = carrier.iter().collect();
            let pixel_color = Color::from_hex(pixel_color);
            let target_color = target_color.clone();
            out.push(Color::WHITE - pixel_color + target_color.clone());
            carrier_i = 0;
        }
    }

    if carrier_i >= 1 {
        let pixel_color = &carrier[0..(carrier_i as usize)];
        let pixel_color: String = pixel_color.iter().collect();
        let pixel_color = format!("{pixel_color:0<6}");
        let pixel_color = Color::from_hex(pixel_color);
        out.push(Color::WHITE - pixel_color + target_color);
    }

    out
}
