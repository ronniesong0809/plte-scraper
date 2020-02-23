mod calendar;
mod common;
mod events;
mod venues;

fn main() {
    println!("
    __________ .____   ______________________           _________  .__   .__
    \\______   \\|    |  \\__    ___/\\_   _____/           \\_   ___ \\ |  |  |__|
     |     ___/|    |    |    |    |    __)_    ______  /    \\  \\/ |  |  |  |
     |    |    |    |___ |    |    |        \\  /_____/  \\     \\____|  |__|  |
     |____|    |_______ \\|____|   /_______  /            \\______  /|____/|__|
                       \\/                 \\/                    \\/           ");
    println!("\t\t\t  - Portland Local Tech Events Command-line Tool Sets.");
    println!("\n\n---- PLTE Menu ----");
    loop {
        println!("\n1. Scraping home page events data");
        println!("2. Scraping all events data");
        println!("3. Scraping all venues data");
        println!("0. Exit");

        // use the > as the prompt
        print!("\n> ");

        let input = common::user_input();
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
