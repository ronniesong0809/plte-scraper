mod calendar;
mod events;
mod venues;
use std::io::{stdin, stdout, Write};

fn user_input() -> std::string::String {
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn main() {
    println!("
    __________ .____   ______________________           _________  .__   .__
    \\______   \\|    |  \\__    ___/\\_   _____/           \\_   ___ \\ |  |  |__|
     |     ___/|    |    |    |    |    __)_    ______  /    \\  \\/ |  |  |  |
     |    |    |    |___ |    |    |        \\  /_____/  \\     \\____|  |__|  |
     |____|    |_______ \\|____|   /_______  /            \\______  /|____/|__|
                       \\/                 \\/                    \\/           ");
    println!("\t\t\t  - Portland Local Tech Events Command-line Tool Sets.");
    loop {
        println!("\n1. Scraping home page events data");
        println!("2. Scraping all events data");
        println!("3. Scraping all venues data");
        println!("0. Exit");

        // use the > as the prompt
        print!("\n> ");

        let input = user_input();
        let command = input.trim().split_whitespace().next().unwrap();

        match &*command {
            "1" => calendar::menu(),
            "2" => events::menu(),
            "3" => venues::menu(),
            "0" => return,
            "q" => return,
            "quit" => return,
            "exit" => return,
            _ => println!("[{}]: command not found, Please try again!", command),
        }
    }
}
