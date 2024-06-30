use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn std_input(options: &Vec<String>) -> Vec<bool> {
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
