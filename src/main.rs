use std::io::{self, Write};

fn main() {

    loop {
        print_prompt();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

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


