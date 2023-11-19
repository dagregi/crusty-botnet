use std::io::Write;

use args::{
    Arguments,
    Commands::{Connections, Execute},
};
use clap::Parser;

mod args;

pub fn init_repl() -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line) {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(err) => {
                write!(std::io::stderr(), "{err}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }

    Ok(())
}

fn respond(line: &str) -> Result<bool, String> {
    let lns = shlex::split(line).ok_or("error: Invalid quoting")?;
    let args = Arguments::try_parse_from(Some("".to_owned()).into_iter().chain(lns))
        .map_err(|e| e.to_string())?;

    match args.sub_commands {
        Connections => println!("sup bitch"),
        Execute(val) => println!("whoa! :{:?}", val.commands),
    }

    Ok(false)
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
