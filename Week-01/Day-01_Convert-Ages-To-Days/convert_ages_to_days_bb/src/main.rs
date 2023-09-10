use std::io;

fn main() {
    let mut input = String::new();
    println!("Please, enter your age:");

    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    let input: u16 = input.trim().parse()
        .expect("please give me correct string number!");
    
    println!("user input: {}.", input);
    println!("That is {} days.", input*365);
}
