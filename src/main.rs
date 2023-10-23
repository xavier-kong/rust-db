use std::io::{self, Write};

#[allow(dead_code)]
enum MetaCommandResult { MetaCommandSuccess, MetaCommandUnrecognizedCommand, Exit }

enum  PrepareResult { PrepareSuccess, PrepareUnrecognizedStatement }

enum StatementType { StatementInsert, StatementSelect}

struct Statement {
    statement_type: StatementType
}

fn do_meta_command(line: &str) -> MetaCommandResult {
    if line.trim_end() == ".exit" {
        return MetaCommandResult::Exit;
    } else {
        return MetaCommandResult::MetaCommandUnrecognizedCommand;
    }
}

fn prepare_statement(line: &str, statement: &mut Statement) -> PrepareResult {
    if line.starts_with("insert") {
        statement.statement_type = StatementType::StatementInsert;
        return PrepareResult::PrepareSuccess;
    }

    if line.trim_end() == "select" {
        statement.statement_type = StatementType::StatementSelect;
        return PrepareResult::PrepareSuccess;
    }

    return PrepareResult::PrepareUnrecognizedStatement
}

fn execute_statement(statement: &mut Statement) {
    match statement.statement_type {
        StatementType::StatementInsert => {
            println!("This is where we would do an insert");
        }
        StatementType::StatementSelect =>  {
            println!("This is where we would do a select");
        }
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn main() {

    loop {
        print_prompt();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        if input.chars().next().unwrap() == '.' {
            match do_meta_command(&input) {
                MetaCommandResult::MetaCommandSuccess => continue,
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command {}", input);
                    continue;
                }
                MetaCommandResult::Exit => { break; }
            }
        }

        let mut statement = Statement { statement_type: StatementType::StatementInsert };

        match prepare_statement(&input, &mut statement) {
            PrepareResult::PrepareSuccess => () ,
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of {}", input);
                continue;
            }
        }

        execute_statement(&mut statement);
        println!("Executed");
    }
}

