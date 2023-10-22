use std::io::{self, Write};

enum MetaCommandResult { MetaCommandSuccess, MetaCommandUnrecognizedCommand }

enum  PrepareResult { PrepareSuccess, PrepareUnrecognizedStatement }

enum StatementType { StatementInsert, StatementSelect}

struct Statement {
    statement_type: StatementType
}

fn do_meta_command(line: &str) -> MetaCommandResult {
    if line.trim_end() == ".exit" {

    } else {
        return MetaCommandResult::MetaCommandUnrecognizedCommand;
    }
}

fn main() {

    loop {
        print_prompt();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        if input.chars().next().unwrap() == '.' {
            match do_meta_command(line) {
                MetaCommandResult::MetaCommandSuccess => continue,
            }
        }

        if input.trim_end() == ".exit" {
            break;
        } else {
            println!("Unrecognized command {}", input);
        }

    }

    fn print_prompt() {
        print!("db > ");
        io::stdout().flush().unwrap();
    }
}

