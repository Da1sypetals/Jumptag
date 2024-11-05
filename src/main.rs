use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
    process::Termination,
};

use colorful::Colorful;
use interface::{cmd::Cmd, parser::parse_command};
use script::{CHECKER, SCRIPT};
use tagbase::{
    errors::{ReportError, TagbaseError, TagbaseResult},
    tagbase::Tagbase,
};
mod interface;
mod script;
mod tagbase;

fn uinfo(info: String) {
    println!("[jump-tag] {}", info);
}

fn uerr(info: String) {
    let err = format!("[jump-tag] {}", info).red();
    eprintln!("{}", err);
}

impl<T> ReportError<T> for TagbaseResult<T> {
    fn report(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                uerr(e.to_string());
                std::process::exit(0);
            }
        }
    }
}

fn main() {
    let cmd = parse_command();

    let base_result = Tagbase::try_new();
    let mut base = match base_result {
        Ok(base) => base,
        Err(e) => {
            uerr(e.to_string());
            std::process::exit(0);
        }
    };

    match cmd {
        interface::cmd::Cmd::Set { tag, dir } => base.set(tag.as_str(), dir.as_str()).unwrap(),
        interface::cmd::Cmd::Get { tag } => {
            let res = base.get(tag.as_str());

            let home_dir = env::var("HOME")
                .map_err(|e| TagbaseError::Internal(e.to_string()))
                .report();

            let jumptag_dir = Path::new(&home_dir).join(".jumptag");

            ReportError::report(
                fs::create_dir_all(&jumptag_dir).map_err(|e| TagbaseError::Internal(e.to_string())),
            );

            let temp_file_path = jumptag_dir.join("temp");

            // 创建或打开文件
            let mut file = File::create(&temp_file_path)
                .map_err(|e| TagbaseError::Internal(e.to_string()))
                .report();

            // 根据 Result 的结果写入内容
            match res {
                Ok(string) => file.write_all(format!("1 {}", string).as_bytes()),
                Err(e) => file.write_all(format!("0 {}", e).as_bytes()),
            }
            .report();
        }
        interface::cmd::Cmd::Delete { tag } => base.delete(tag.as_str()).unwrap(),
        interface::cmd::Cmd::List => {
            let res = base.list().unwrap();
            let binds = res
                .into_iter()
                .map(|(k, v)| format!("\n{} => {}", k, v))
                .collect::<Vec<_>>();

            uinfo(format!("({} bindings) {}", binds.len(), binds.join("")));
        }
        Cmd::Init { filename } => {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .append(true)
                .open(filename)
                .map_err(|e| TagbaseError::Internal(e.to_string()))
                .report();

            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|e| TagbaseError::Internal(e.to_string()))
                .report();

            if content.contains(CHECKER) {
                uinfo("already initialized!".into());
            } else {
                ReportError::report(
                    write!(file, "{}", SCRIPT).map_err(|e| TagbaseError::Internal(e.to_string())),
                );
            }
        }
    }
}