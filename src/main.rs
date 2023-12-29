mod command;

use std::{
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
    process,
};

use command::{command, command_matches};
use encode_to_image_rs::{chameleon_encoder, image_encoder, mask_encoder, Color};

fn main() {
    let matches = command().get_matches();

    let mut command = command_matches(&matches);

    let data: Vec<u8> = match command.input_type {
        "input" => {
            let input = matches
                .get_one::<PathBuf>("input")
                .expect("Group make it required");

            if command.output_path.to_str().unwrap() == "%INPUT-FILE%.out.png" {
                let input_path = input.file_stem().unwrap().to_str().unwrap();
                command
                    .output_path
                    .set_file_name(input_path.to_owned() + ".out.png");
            }

            if !command.overwrite {
                if let Ok(_) = fs::metadata(&command.output_path) {
                    print!(
                        "The file \"{}\" already exists.\nDo you want to overwrite it? (Y/n) ",
                        command.output_path.display()
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

    println!("Output path: {}", command.output_path.display());
    println!("Data length: {}", data.len());

    // Arbitrary size for enable compression
    let data = if data.len() > 10_000 {
        let data = lzma::compress(&data, 9).unwrap();
        println!("Compressed: {}", data.len());
        data
    } else {
        data
    };

    if command.chameleon {
        chameleon_encoder::encode();
    } else {
        let target_color = command.target_color;

        if let Some(mask) = command.mask {
            mask_encoder::encode(
                &data,
                target_color.expect("Mask needs a target color to change."),
                command.fake_color,
                mask,
                command.output_path,
            );
        } else {
            let fake_color = command.fake_color.or(target_color).unwrap_or_default();
            image_encoder::encode(&data, fake_color, command.output_path);
        }
    }
}
