use std::io::{self, Write};
use std::mem;

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

// https://stackoverflow.com/a/70224634 
fn get_size_of_return_type<F, T, U>(_f: F) -> usize
where
F: FnOnce(T) -> U
{
    std::mem::size_of::<U>()
}

const ID_SIZE: u32 = get_size_of_return_type(|s: Row | s.id);
const USERNAME_SIZE: u32 = get_size_of_return_type(|s: Row | s.username);
const EMAIL_SIZE: u32 = get_size_of_return_type(|s: Row | s.email);
const ID_OFFSET: u32 = 0;

const USERNAME_OFFSET: u32 = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: u32 = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: u32 = ID_SIZE + USERNAME_SIZE + EMAIL_SIZE;

const PAGE_SIZE: u32 = 4096;
const TABLE_MAX_PAGES: u32 = 100;
const ROWS_PER_PAGE: u32 = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: u32 = ROWS_PER_PAGE * TABLE_MAX_PAGES;
const TABLE_MAX_PAGES_SIZE: usize = TABLE_MAX_PAGES as usize;

struct Table {
    num_rows: u32,
    pages: [u32; TABLE_MAX_PAGES_SIZE]
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
                statement.row_to_insert.username = v.1.to_string(); statement.row_to_insert.email = v.2.to_string();
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

fn serialize_row(source: &mut Row, destination: &[u32; TABLE_MAX_PAGES_SIZE]) {


    // bytes_buf[index*MAX_DATA_LENGTH..(index+1)*MAX_DATA_LENGTH-4].copy_from_slice(&buf[4..]);
}

fn row_slot(table: &mut Table, row_num: u32) -> u32 {
    let page_num: u32 = row_num / ROWS_PER_PAGE;
    let page = table.pages[page_num as usize];
    let row_offset: u32 = row_num % ROWS_PER_PAGE;
    let byte_offset: u32 = row_offset * ROW_SIZE;
    return table.pages[page + byte_offset as usize];
}

fn execute_insert(statement: &mut Statement, table: &mut Table) -> ExecuteResult {
    if table.num_rows >= TABLE_MAX_ROWS {
        return ExecuteResult::ExecuteTableFull;
    }

    let row_to_insert = &(statement.row_to_insert);

    serialize_row(& mut row_to_insert, row_slot(table, table.num_rows));
    table.num_rows += 1;

    return ExecuteResult::ExecuteSuccess;
}

fn execute_statement(statement: &mut Statement, table: &mut Table) -> ExecuteResult {
    match statement.statement_type {
        StatementType::StatementInsert => {
            return execute_insert(statement, table);
        }
        StatementType::StatementSelect =>  {
            return execute_select(statement, table);
        }
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn main() {
    let table = Table {
        num_rows: 0,
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

