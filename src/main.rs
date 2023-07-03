mod command;

use std::{
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
    process,
};

use command::{command, command_matches};
use encode_to_image_rs::{image_encoder, mask_encoder, Color};

fn main() {
    let matches = command().get_matches();

    let (mut output_path, target_color, fake_color, input_type, overwrite) =
        command_matches(&matches);

    let data: Vec<u8> = match input_type {
        "input" => {
            let input = matches
                .get_one::<PathBuf>("input")
                .expect("Group make it required");

            if output_path.to_str().unwrap() == "%INPUT-FILE%.out.png" {
                let input_path = input.file_stem().unwrap().to_str().unwrap();
                output_path.set_file_name(input_path.to_owned() + ".out.png");
            }

            if !overwrite {
                if let Ok(_) = fs::metadata(&output_path) {
                    print!(
                        "The file \"{}\" already exists.\nDo you want to overwrite it? (Y/n) ",
                        output_path.display()
                    );
                    std::io::stdout().lock().flush().unwrap();

                    let mut answer = String::new();
                    std::io::stdin()
                        .read_line(&mut answer)
                        .expect("Something got wrong while reading stdin");

                    if answer.len() > 1 {
                        let answer = &answer[0..1];
                        if answer != "Y" && answer != "y" {
                            eprintln!("Cancelled!");
                            process::exit(0);
                        }
                    }
                };
            }

            let input = fs::read(input);
            match input {
                Ok(input) => input,
                Err(error) => {
                    match error.kind() {
                        io::ErrorKind::NotFound => {
                            eprintln!("IO Error: The input file doesn't exist")
                        }
                        _ => {
                            eprintln!("IO Error (input): {}", error.to_string());
                        }
                    };
                    process::exit(1);
                }
            }
        }
        "stdin" => {
            let stdin = std::io::stdin();
            let mut input = Vec::new();
            stdin
                .lock()
                .read_until(27, &mut input)
                .expect("Stdin input is not UTF-8. Please be sure your input is plain text");

            input
        }
        _ => unreachable!(),
    };

    println!("Output path: {}", output_path.display());
    println!("Data length: {}", data.len());

    // Arbitrary size for enable compression
    let data = if data.len() > 10_000 {
        let data = lzma::compress(&data, 9).unwrap();
        println!("Compressed: {}", data.len());
        data
    } else {
        data
    };

    let mask = matches.get_one::<PathBuf>("mask");

    if let Some(mask) = mask {
        mask_encoder::encode::encode_to_mask(
            &data,
            // Color::new(207, 151, 87),
            target_color,
            fake_color,
            // Some(Color::new(255, 255, 47)),
            mask,
            output_path,
        );
    } else {
        image_encoder::encode::encode_to_image(
            &data,
            fake_color.unwrap_or(target_color),
            output_path,
        );
    }
}
