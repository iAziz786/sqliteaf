use std::io::{self, Write};
mod commands;

fn main() {
    while true {
        print_prompt();
        let mut input_buf = InputBuf::new();
        take_input(&mut input_buf);
        if input_buf.buffer.chars().nth(0) == Some('.') {
            match do_meta_command(&input_buf) {
                MetaCommandResult::MetaCommandSuccess => {}
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Error: unknown command or invalid arguments: \"{}\". Enter \".help\" for help", &input_buf.buffer[1..])
                }
            }
            continue;
        }
        let (prep_status, _statement) = prepare_statement(&input_buf);
        if prep_status == PrepareResult::PrepareUnrecognizedStatement {
            println!("Parse error: near \"{}\": syntax error", &input_buf.buffer)
        }
        let statement = _statement.unwrap();
        execute_statement(statement);
        println!("executed.")
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

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

#[derive(PartialEq)]
enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

enum StatementType {
    StatementInsert,
    StatementSelect,
}

struct Statement {
    r#type: StatementType,
}

fn do_meta_command(input_buf: &InputBuf) -> MetaCommandResult {
    match input_buf.buffer.as_str() {
        ".exit" => {
            commands::exit::exit();
            MetaCommandResult::MetaCommandSuccess
        }
        ".help" => {
            commands::help::help();
            MetaCommandResult::MetaCommandSuccess
        }
        _ => MetaCommandResult::MetaCommandUnrecognizedCommand,
    }
}

fn prepare_statement(input_buf: &InputBuf) -> (PrepareResult, Option<Statement>) {
    if input_buf.buffer.starts_with("insert") {
        return (
            PrepareResult::PrepareSuccess,
            Some(Statement {
                r#type: StatementType::StatementInsert,
            }),
        );
    } else if input_buf.buffer.starts_with("select") {
        return (
            PrepareResult::PrepareSuccess,
            Some(Statement {
                r#type: StatementType::StatementSelect,
            }),
        );
    }
    return (PrepareResult::PrepareUnrecognizedStatement, None);
}

fn execute_statement(statement: Statement) {
    match statement.r#type {
        StatementType::StatementInsert => {
            println!("executing insert statement")
        }
        StatementType::StatementSelect => {
            println!("executing select statement")
        }
    }
}
