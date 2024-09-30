use std::{io::{stdin, stdout, Write}, mem};

fn main() {
    let mut prev_input = String::new();
    let mut stdout = stdout().lock();
    
    loop {
        let mut cur_input = String::new();
        stdin().read_line(&mut cur_input).unwrap();
        
        if &cur_input != &prev_input {
            writeln!(stdout, "Output: {cur_input}").unwrap();

            // Так как String хранится в куче, то же самое ли просто сделать move cur_input -> prev_input ?
            // prev_input = cur_input
            mem::swap(&mut prev_input, &mut cur_input);
        }
    }
}