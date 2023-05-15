use std::env;
use std::str;
use std::io;
use std::io::Write;

fn tokenize(program: &str) {
    // TODO Create Token Trait
    let mut tokens: Vec<char> = Vec::new();
    for prog_char in program.chars() {
        match prog_char {
            '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => tokens.push(prog_char),
            _ => {},
        }
    }
    // println!("{:?}", tokens);
    process(&tokens);
}

fn process(tokens: &Vec<char>) {
    let mut cells: [usize; 30000] = [0; 30000];
    let mut pointer: usize = 0;
    let mut token_pointer: usize = 0;
    let brackets_positions: Vec<(usize, usize)> = compute_brackets_positions(&tokens);
    while token_pointer < tokens.len() {
        // TODO Break out when lots of steps?
        // if num_steps > MAX_STEPS {break}
        let token = tokens[token_pointer];
        match token {
            '>' => {
                if pointer < cells.len() - 1 {
                    pointer += 1;
                    token_pointer += 1;
                } // TODO Else error
            }
            '<' => {
                if pointer > 0 {
                    pointer -= 1;
                    token_pointer += 1;
                } // TODO Else error
            }
            '+' => {
                cells[pointer] += 1;
                token_pointer += 1;
            }
            '-' => {
                if cells[pointer] > 0 {
                    cells[pointer] -= 1;
                    token_pointer += 1;
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
                token_pointer += 1;
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
                token_pointer += 1;
            }
            '[' => {
                if cells[pointer] == 0 {
                    for pair in &brackets_positions {
                        if pair.0 == token_pointer {
                            token_pointer = pair.1;
                        }
                    }
                } else {
                    token_pointer += 1;
                }
            }
            ']' => {
                if cells[pointer] != 0 {
                    for pair in &brackets_positions {
                        if pair.1 == token_pointer {
                            token_pointer = pair.0;
                        }
                    }
                } else {
                    token_pointer += 1;
                }
            }
            _ => {}
        }
    }
}

fn compute_brackets_positions(tokens: &Vec<char>)-> Vec<(usize, usize)> {
    let mut open_positions: Vec<usize> = Vec::new();
    let mut brackets_positions: Vec<(usize, usize)> = Vec::new();
    for i in 0..tokens.len() {
        match tokens[i] {
            '[' => open_positions.push(i),
            ']' => if open_positions.len() > 0 {
                brackets_positions.push((open_positions.pop().unwrap(), i));
            } else {
                // TODO Unmatched Brackets Error
            }
            _ => {}
        }
    }
    if open_positions.len() > 0 {
        // TODO Unmatched Brackets Error
    }
    return brackets_positions;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // println!("{}", args[1]);
        tokenize(&args[1]);
    }
}
