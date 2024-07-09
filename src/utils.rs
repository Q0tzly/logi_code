use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process::exit;

use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn new() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_input: Result<Vec<String>, _> = match args[1].as_str() {
            "help" => {
                help();
                exit(0)
            }
            "run" => {
                if args.len() == 3 {
                    Ok(file_input(args[2].clone()))
                } else {
                    Err("Usage: logi run <file>")
                }
            }
            _ => Err("Invalid command. If confirm usage, type `logi help`."),
        };

        if let Ok(input) = file_input {
            return input;
        } else {
            eprintln!("{}", file_input.unwrap_err());
            exit(1);
        }
    }
    eprintln!("Error: Missing arguments. If confirm usage, type `logi help`.");
    exit(0);
}

fn file_input(path: String) -> Vec<String> {
    let path = Path::new(&path);
    let f = match File::open(path) {
        Err(_) => {
            eprintln!("Error: Couldn't open file.");
            exit(0);
        }
        Ok(file) => file,
    };

    let reader = BufReader::new(f);
    let mut result = vec![];
    for line in reader.lines() {
        result.push(line.unwrap());
    }
    result
}

fn help() {
    println!("Usage\n  logi <option>\nOptions\n  help : put usage\n  run  : run file.lc");
}

pub fn std_input(options: &[String]) -> Vec<bool> {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdout = io::BufWriter::new(stdout);

    let mut selections = vec![false; options.len()];
    let mut selected_index = 0;

    write!(stdout, "{}[2J", 27 as char).unwrap();
    loop {
        write!(stdout, "{}[H", 27 as char).unwrap();

        for (index, option) in options.iter().enumerate() {
            write!(
                stdout,
                " {} {} {}",
                if index == selected_index { ">" } else { " " },
                option,
                if selections[index] { "■" } else { "□" }
            )
            .unwrap();
        }

        stdout.flush().unwrap();

        match io::stdin().keys().next().unwrap().unwrap() {
            Key::Char('\t') => {
                selected_index = (selected_index + 1) % options.len();
            }
            Key::Char(' ') => {
                selections[selected_index] = !selections[selected_index];
            }
            Key::Char('\n') => break,
            _ => {}
        }
    }

    write!(stdout, "{}[H", 27 as char).unwrap();
    write!(stdout, "in  > ").unwrap();
    for (index, (option, selected)) in options.iter().zip(selections.iter()).enumerate() {
        write!(stdout, "{} {}", option, if *selected { "■" } else { "□" }).unwrap();
        if index < options.len() - 1 {
            write!(stdout, " : ").unwrap();
        }
    }
    writeln!(stdout).unwrap();
    write!(stdout, "\n\r").unwrap();
    stdout.flush().unwrap();

    selections
}
