use std::{collections::HashMap, io::Write, net::TcpStream};

use args::{
    Arguments,
    Commands::{Connections, Execute, Quit},
};
use clap::Parser;

use crate::handlers::get_connections;

mod args;

pub fn init_repl(
    stream: &mut TcpStream,
    connections: &mut HashMap<String, Option<TcpStream>>,
) -> Result<(), String> {
    loop {
        let line = readline()?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match respond(line, stream, connections) {
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

fn respond(
    line: &str,
    stream: &mut TcpStream,
    connections: &mut HashMap<String, Option<TcpStream>>,
) -> Result<bool, String> {
    let line = shlex::split(line).ok_or("error: Invalid quoting")?;
    let args = Arguments::try_parse_from(Some("".to_owned()).into_iter().chain(line))
        .map_err(|e| e.to_string())?;

    match args.sub_commands {
        Connections => {
            get_connections(connections).unwrap();
            Ok(false)
        }
        Execute(val) => {
            let cmds: String = val
                .commands
                .clone()
                .iter_mut()
                .map(|v| {
                    v.push(' ');
                    v.to_string()
                })
                .collect();
            stream.write_all(cmds.as_bytes()).unwrap();
            Ok(false)
        }
        Quit => Ok(true),
    }
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
