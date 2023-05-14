use std::env;
use std::str;
use std::io;
use std::io::Write;

fn tokenize(program: &str) {
    // TODO Create Token Trait
    let mut tokens: Vec<char> = Vec::new();
    for prog_char in program.chars() {
        match prog_char {
            '>' | '<' | '+' | '-' | '.' | ',' => tokens.push(prog_char),
            _ => {},
        }
    }
    // println!("{:?}", tokens);
    process(&tokens);
}

fn process(tokens: &Vec<char>) {
    let mut cells: [usize; 30000] = [0; 30000];
    let mut pointer: usize = 0;
    for token in tokens {
        match token {
            '>' => {
                if pointer < cells.len() - 1 {
                    pointer += 1;
                } // TODO Else error
            }
            '<' => {
                if pointer > 0 {
                    pointer -= 1;
                } // TODO Else error
            }
            '+' => {
                cells[pointer] += 1;
            }
            '-' => {
                if cells[pointer] > 0 {
                    cells[pointer] -= 1;
                } // TODO Else error
            }
            '.' => {
                // let utf8_char:u8 = cells[pointer] as u8;
                let utf8_char:u8;
                match u8::try_from(cells[pointer]) {
                    Ok(val) => {
                        utf8_char = val;
                        print!("{}", str::from_utf8(&[utf8_char]).expect("Error"));
                    }
                    Err(_) => {
                        print!("0x{:X}", cells[pointer]);
                    }
                }
                io::stdout().flush().unwrap();
            }
            ',' => {
                let mut inp: String = String::new();
                io::stdin().read_line(&mut inp)
                    .expect("Error getting input");
                inp = inp.trim().to_owned();
                match inp.parse::<usize>() {
                    Ok(val) if val > 0 && val < std::usize::MAX => {
                        cells[pointer] = val;
                    }
                    Ok(_) | Err(_) => {
                        // TODO Error
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // println!("{}", args[1]);
        tokenize(&args[1]);
    }
}
