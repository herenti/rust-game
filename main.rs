/*
romance novel, help command to look up terms and command usage. dating sim esque with multple romantic interests. choose gender/orientation in the beginning? set phrases based on character gender. have romantic interests be based on orientation. one hidden romantic interest is the divine. divine route attained by doing temple stuff, progress into a further story of spiritual activity. add items? probably wont need items in this game. maybe can have items but are not usable and just for the purpose of examining them. commision art of characters?
romantic interest story routes are defined by number code. the 100000s will be one route. the 200000s will be another route. 000000 will be before a route is set. progress in route will be defined by 100001 and the game will load on the progress number from save file. perhaps 1a0001, 1b0001 etc. for sub routes in each route? save user name and gender/orientation and items to file as well.
KEEP IT SIMPLE.
 */

use colored::Colorize;
use std::io;
use std::io::Write;


struct Help {

}

impl Help {
    fn lookup(term: &str) -> String {
        let resp = match term {
            "list" => "[save, help, load, quit, lastmessage, new]".to_string(),
            _ => format!("{}", "INVALID HELP TERM".red().bold()),
        };
        resp
    }
}


struct Commands {

}

impl Commands {
    fn new(command: &str) {
        let args = command.split(" ");
        let args: Vec<&str> = args.collect();
        let command = args[0];
        let args = if args.len() > 1 {
            args[1..].join(" ")
        } else {
            "".to_string()
        };
        let command = command.to_lowercase();
        let command = command.as_str();
        match command {
            "help" => {
                let resp = if args.len() == 0 {
                    format!("How to use the help command. You can look up a characters name, or an item name like so: [{}]. [{}]. To show a list of commands use [{}]. Press enter with no command to continue the story.", "help itemnamehere".green(), "help commandnamehere".green(), "help list".green())
                    } else {
                        Help::lookup(&args)
                    };
                println!("[{}]: {}", "Help".blue().bold(), &resp);

                }

            _ => {

                println!("{}", "INVALID COMMAND".red().bold());
            }
        }

    }

}



struct User {
    name: String,
    items: Vec<String>,
    progress: i32,
}

fn main() {
    let mut gamerunning = true;
    while gamerunning {
        print!("[{}]: ", "Command".blue().bold());
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let input = input.trim();
        if input == "quit" {
            gamerunning = false;
        } else {
            Commands::new(&input);
        };
    }
}
