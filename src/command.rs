use std::path::PathBuf;

use clap::{arg, builder::ArgPredicate, ArgAction, ArgGroup, ArgMatches, Command, Id};

use crate::Color;

#[inline(always)]
pub fn command() -> Command {
    clap::command!()
        .help_expected(true)
        .propagate_version(true)
        .disable_help_subcommand(false)
        .arg(
            arg!(-m --mask <mask> "Set image to use as mask")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            arg!(-k --chameleon "Set mask mode to chameleon")
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!(-i --input <file> "Set file to use as input data")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            arg!(-s --stdin "If use stdin as input data")
                .action(ArgAction::SetTrue)
                .conflicts_with("input"),
        )
        .arg(
            arg!(-o --output <file> "Set output file")
                .required_if_eq("some-input", "stdin")
                .default_value_if(
                    "input",
                    ArgPredicate::IsPresent,
                    Some("%INPUT-FILE%.out.png"),
                )
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            arg!(-t --"target-color" <hex> "Color to change in the mask (In hex format)")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            arg!(-c --color <hex> "Color to emulate in the output (In hex format)")
                .value_parser(clap::value_parser!(String))
                .conflicts_with("chameleon"),
        )
        .arg(
            arg!(-y --overwrite "Don't prompt for overwrite the output file")
                .action(ArgAction::SetTrue)
                .requires("input"),
        )
        .group(
            ArgGroup::new("some-input")
                .arg("input")
                .arg("stdin")
                .required(true),
        )
        .group(
            ArgGroup::new("chameleon-or-target")
                .arg("chameleon")
                .arg("target-color")
                .requires("mask"),
        )
}

pub struct CommandMatches<'a> {
    pub output_path: PathBuf,
    pub mask: Option<&'a PathBuf>,
    pub chameleon: bool,
    pub target_color: Option<Color>,
    pub fake_color: Option<Color>,
    pub input_type: &'a str,
    pub overwrite: bool,
}

#[inline(always)]
pub fn command_matches(matches: &ArgMatches) -> CommandMatches<'_> {
    let chameleon = matches.get_flag("chameleon");

    let target_color = matches
        .get_one::<String>("target-color")
        .map(|c| Color::from_hex(c.to_owned()));

    let target_color = if chameleon {
        target_color.or(Some(Color::WHITE))
    } else {
        target_color
    };

    CommandMatches {
        output_path: matches
            .get_one::<PathBuf>("output")
            .expect("Always provide an output path")
            .clone(),
        mask: matches.get_one::<PathBuf>("mask"),
        chameleon,
        target_color,
        fake_color: matches
            .get_one::<String>("color")
            .map(|c| Color::from_hex(c.to_owned())),
        input_type: matches
            .get_one::<Id>("some-input")
            .expect("Required is true")
            .as_str(),
        overwrite: matches.get_flag("overwrite"),
    }
}
