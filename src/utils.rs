use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Read, Stdout, Write};
use std::path::Path;
use std::process::exit;

use termion::clear::CurrentLine;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

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
    println!(
        "Usage\n  logi <option>\nOptions\n  help          : put usage\n  run <file.lc> : run file.lc"
    );
}

pub fn stdin(options: &[String]) -> Vec<bool> {
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    let mut selections = vec![false; options.len()];
    let mut selected_index = 0;

    loop {
        for (index, option) in options.iter().enumerate() {
            write!(
                stdout,
                "{} {} {}",
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
        clear_line(&mut stdout);
    }

    if let Some(row) = get_row(&mut stdout) {
        write!(stdout, "{}", Goto(1, row)).unwrap();

        clear_line(&mut stdout);

        for (index, (option, selected)) in options.iter().zip(selections.iter()).enumerate() {
            print!("{} {}", option, if *selected { "■" } else { "□" });
            if index < options.len() - 1 {
                print!(" : ");
            }
        }
        write!(stdout, "{}", Goto(1, row)).unwrap();
    }
    println!("");

    selections
}

fn clear_line(stdout: &mut RawTerminal<Stdout>) {
    if let Some(row) = get_row(stdout) {
        write!(stdout, "{}", Goto(1, row)).unwrap();
        write!(stdout, "{}", CurrentLine).unwrap();
    }
}

fn get_row(stdout: &mut RawTerminal<Stdout>) -> Option<u16> {
    let stdin = io::stdin();
    let mut stdin_bytes = stdin.bytes();

    write!(stdout, "\x1B[6n").unwrap();
    stdout.flush().unwrap();

    let mut response = Vec::new();
    let mut started = false;

    while let Some(Ok(byte)) = stdin_bytes.next() {
        if byte == b'\x1B' {
            started = true;
        }

        if started {
            response.push(byte);

            if byte == b'R' {
                break;
            }
        }
    }

    let response = String::from_utf8_lossy(&response);

    if let Some(pos) = response.strip_prefix("\x1B[") {
        if let Some((row, _)) = pos.split_once(';') {
            if let Ok(row) = row.parse::<u16>() {
                return Some(row);
            }
        }
    }
    None
}
