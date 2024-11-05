use clap::{arg, command, Command};

use super::cmd::Cmd;

pub(crate) fn parse_command() -> Cmd {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("jump-tag: Add a tag-dir binding")
                .arg(arg!([tag]).required(true))
                .arg(arg!([dir]).required(true)),
        )
        .subcommand(
            Command::new("del")
                .about("jump-tag: Delete a tag-dir binding")
                .arg(arg!([tag]).required(true)),
        )
        .subcommand(
            Command::new("get")
                .about("jump-tag: Get dir related to tag")
                .arg(arg!([tag]).required(true)),
        )
        .subcommand(Command::new("ls").about("jump-tag: List all tag-dir pairs"))
        .subcommand(
            Command::new("init")
                .about("jump-tag: Initialize .rc file")
                .arg(arg!([filename]).required(true)),
        )
        .arg_required_else_help(true)
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => Cmd::Set {
            tag: sub_matches.get_one::<String>("tag").unwrap().to_owned(),
            dir: sub_matches.get_one::<String>("dir").unwrap().to_owned(),
        },
        Some(("del", sub_matches)) => Cmd::Delete {
            tag: sub_matches.get_one::<String>("tag").unwrap().to_owned(),
        },
        Some(("get", sub_matches)) => Cmd::Get {
            tag: sub_matches.get_one::<String>("tag").unwrap().to_owned(),
        },
        Some(("ls", _)) => Cmd::List,
        Some(("init", sub_matches)) => Cmd::Init {
            filename: sub_matches
                .get_one::<String>("filename")
                .unwrap()
                .to_owned(),
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
