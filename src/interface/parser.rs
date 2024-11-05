use clap::{arg, command, Command};
use regex::Regex;

use super::cmd::Cmd;

pub(crate) fn legal_tag(tag: &str) -> Result<String, String> {
    let re = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    if re.is_match(&tag) {
        Ok(tag.into())
    } else {
        Err("Require tag to be alphanumeric!".into())
    }
}

pub(crate) fn parse_command() -> Cmd {
    let matches = command!() // requires `cargo` feature
        .propagate_version(true)
        .arg_required_else_help(true)
        .arg(arg!([tag]).value_parser(legal_tag))
        .subcommand(
            Command::new("-add")
                .about("Add a tag-dir binding")
                .arg(arg!([tag]).required(true).value_parser(legal_tag))
                .arg(arg!([dir]).required(true)),
        )
        .subcommand(
            Command::new("-del")
                .about("Delete a tag-dir binding")
                .arg(arg!([tag]).required(true).value_parser(legal_tag)),
        )
        .subcommand(Command::new("-ls").about("List all tag-dir pairs"))
        .subcommand(
            Command::new("-init")
                .about("Initialize .rc file")
                .arg(arg!([filename]).required(true)),
        )
        .arg_required_else_help(true)
        .get_matches();

    match matches.subcommand() {
        Some(("-add", sub_matches)) => Cmd::Set {
            tag: sub_matches.get_one::<String>("tag").unwrap().to_owned(),
            dir: sub_matches.get_one::<String>("dir").unwrap().to_owned(),
        },
        Some(("-del", sub_matches)) => Cmd::Delete {
            tag: sub_matches.get_one::<String>("tag").unwrap().to_owned(),
        },
        Some(("-ls", _)) => Cmd::List,
        Some(("-init", sub_matches)) => Cmd::Init {
            filename: sub_matches
                .get_one::<String>("filename")
                .unwrap()
                .to_owned(),
        },
        None => Cmd::Get {
            tag: matches.get_one::<String>("tag").unwrap().to_owned(),
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
