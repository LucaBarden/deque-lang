use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Please provide a deque file to interpret");
        return;
    }
    let file_path = args[1].to_string();
    let mut input_file = match File::open(file_path) {
        Ok(x) => x,
        Err(e) => panic!("Couldn't open file {e}"),
    };
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();

    let tokens: Vec<&str> = input.split(" ").collect();

    let mut data: VecDeque<char> = VecDeque::new();
    for i in 0..tokens.len() {
        let mut currentToken = tokens[i];
        let mut s = currentToken.to_string();
        if currentToken.ends_with("\n") {
            s.truncate(s.len() - 1);
            currentToken = s.as_str();
        }
        if currentToken.len() != 2 {
            eprintln!("Instructions must be at least two characters long {currentToken}");
            return;
        }
        run_instruction(currentToken, &mut data);
    }
    println!("");
}

fn run_instruction(token: &str, deque: &mut VecDeque<char>) {
    let instruction = token.chars().nth(0).unwrap();
    let var = token.chars().nth(1).unwrap();
    match instruction {
        '!' => deque.push_front(var),
        ':' => deque.push_back(var),
        '?' => _ = deque.pop_front(),
        ';' => _ = deque.pop_back(),
        '^' => three_print_op(var, deque),
        'u' => {
            print!("{var}");
            io::stdout().flush().unwrap()
        }
        '\n' => return,
        _ => panic!("Invalid Operation at {instruction}"),
    };
}

fn three_print_op(token: char, deque: &mut VecDeque<char>) {
    let mut output = String::new();
    output.push(deque.pop_front().unwrap());
    output.push(deque.pop_front().unwrap());
    output.push(token);
    unsafe {
        output.as_mut_vec().reverse();
    }
    println!("{output}");
}
