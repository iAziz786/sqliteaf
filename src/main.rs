use std::io::{self, Write};
mod commands;

fn main() {
    while true {
        print_prompt();
        let mut input_buf = InputBuf::new();
        take_input(&mut input_buf);
        match input_buf.buffer.as_str() {
            ".exit" => {
                commands::exit::exit();
            }
            ".help" => {
                commands::help::help();
            }
            _ => println!(
                "Error: unknown command or invalid arguments: \"{}\". Enter \".help\" for help",
                &input_buf.buffer[1..]
            ),
        }
    }
}

#[derive(Debug)]
struct InputBuf {
    buffer: String,
    input_len: i32,
}

impl InputBuf {
    fn new() -> Self {
        InputBuf {
            buffer: String::from(""),
            input_len: 0,
        }
    }
}

fn print_prompt() {
    print!("sqliteaf > ");
    io::stdout().flush().unwrap()
}

fn take_input(input_buf: &mut InputBuf) {
    match std::io::stdin().read_line(&mut input_buf.buffer) {
        Ok(_) => {
            // remove the spaces which also removes the trailing newline
            input_buf.buffer = input_buf.buffer.trim().to_string();
        }
        Err(e) => eprintln!("problem reading stdin {}", e),
    };
}
