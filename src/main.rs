use std::io::{self, Write};
use rand::distributions::Uniform;
use rand::Rng;
extern crate clipboard;
use clipboard::{ClipboardContext, ClipboardProvider};


fn main() -> io::Result<()> {

    println!("Password generator made by DynCake");

    let pass_length: u32 = loop {
        
        print!("Password length: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();

        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("There was an error reading the line!");
            continue;
        };

        input = input.trim().to_string();

        let parsed_input = match input.parse::<u32>() {

            Ok(num) => num,
            Err(_) => {
                println!(
                    "Could not parse your input\n\
                    Make sure your input is in the range of 0 - 1000\n\
                    "
                );
                continue;
            },

        };

        if parsed_input > 1000 {
            println!(
                "Your password length is out of range!\n\
                Make sure your input is in the range of 0 - 1000\n\
            ");
            continue;
        }

        if parsed_input < 1 {
            println!(
                "Why the hell do you want to generate a password that is 0 characters long"
            );
            continue;
        }

        break parsed_input;
    };

    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let range = Uniform::new('A', 'z');
    let password: String = (0..pass_length).map(|_| 
        rng.sample(range) as char
    ).collect();

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(password).unwrap();

    println!("Your password: {}\nCopied to your clipboard!", "*".repeat(pass_length as usize));

    println!("Press the enter key to continue...");
    io::stdin().read_line(&mut String::new()).unwrap();
    

    Ok(())
}
