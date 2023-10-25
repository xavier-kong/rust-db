use std::io::{self, Write};

#[allow(dead_code)]
enum MetaCommandResult { MetaCommandSuccess, MetaCommandUnrecognizedCommand, Exit }

enum  PrepareResult { PrepareSuccess, PrepareUnrecognizedStatement, PrepareSyntaxError }

enum StatementType { StatementInsert, StatementSelect }

enum ExecuteResult { ExecuteSuccess, ExecuteTableFull }

struct Statement {
    statement_type: StatementType,
    row_to_insert: Row
}

impl Default for Statement {
    fn default() -> Self {
        Statement {
            statement_type: StatementType::StatementInsert,
            row_to_insert: Default::default()
        }
    }
}

const COLUMN_USERNAME_SIZE: u8 = 32;
const COLUMN_EMAIL_SIZE: u8 = 255;

struct Row {
    id: u32,
    username: String,
    email: String
}

impl Default for Row {
    fn default() -> Self {
        Row {
            id: 0,
            username: "".to_string(),
            email: "".to_string()
        }
    }
}

const TABLE_MAX_PAGES: usize = 100;

struct Table {
    new_rows: u32,
    pages: [usize; TABLE_MAX_PAGES]
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
        let parsed = sscanf::sscanf!(line, "insert {} {} {}", u32, str, str);
        match parsed {
            Ok(v) => {
                statement.row_to_insert.id = v.0;
                statement.row_to_insert.username = v.1.to_string();
                statement.row_to_insert.email = v.2.to_string();
            },
            Err(_) => return PrepareResult::PrepareSyntaxError
        }
        return PrepareResult::PrepareSuccess;
    }

    if line.trim_end() == "select" {
        statement.statement_type = StatementType::StatementSelect;
        return PrepareResult::PrepareSuccess;
    }

    return PrepareResult::PrepareUnrecognizedStatement
}

fn execute_statement(statement: &mut Statement, table: &mut Table) -> ExecuteResult {
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
    let table = Table {
        new_rows: 0,
        pages: [0; TABLE_MAX_PAGES]
    };

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

        let mut statement: Statement = Default::default();

        match prepare_statement(&input, &mut statement) {
            PrepareResult::PrepareSuccess => () ,
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of {}", input);
                continue;
            },
            PrepareResult::PrepareSyntaxError => {
                println!("Syntax error. Could not parse statement.");
                continue;
            }
        }

        match execute_statement(&mut statement, table) {

        }
    }
}

