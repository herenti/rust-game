/*
romance novel, help command to look up terms and command usage. dating sim esque with multple romantic interests. choose gender/orientation in the beginning? set phrases based on character gender. have romantic interests be based on orientation. one hidden romantic interest is the divine. divine route attained by doing temple stuff, progress into a further story of spiritual activity. add items? probably wont need items in this game. maybe can have items but are not usable and just for the purpose of examining them. commision art of characters?
 each route? save user name and gender/orientation and items to file as well.
KEEP IT SIMPLE. only one choice in game should be for romantic interest. get to know interests before making decisions.
 */

use colored::Colorize;
use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::fs::File;


fn save(data: &mut User) {
    let mut file = File::create("utils.txt");
    let items = data.items.join("-");
    let info = format!("{}:{}:{}:{}", data.name, items, data.progress, data.route);
    let info = info.as_bytes();
    file.expect("reason").write_all(info);
}

struct Story{
    start: HashMap<i32, String>,
    hanako: HashMap<i32, String>,
    futaba: HashMap<i32, String>,
    takeshi: HashMap<i32, String>,
    hijikata: HashMap<i32, String>,
    kami: HashMap<i32, String>,
}

impl Story{
    fn init() -> Self {
        let mut story = Story {
            start: HashMap::new(),
            hanako: HashMap::new(),
            futaba: HashMap::new(),
            takeshi: HashMap::new(),
            hijikata: HashMap::new(),
            kami: HashMap::new(),
        };

        ///////////
        //       //
        // start //
        //       //
        ///////////

        story.start.insert(1, format!("It was a cloudy and brisk day in the small village where my story began. The story of finding the great treasure of my {}.", "heart".red().bold()));
        story
    }
}
struct Help {

}

impl Help {
    fn lookup(term: &str, user: User) -> String {
        let resp = match term {
            "list" => {
                "[save, help, load, quit, lastmessage, new]".to_string()
            }
            "me" => {
                if user.progress > 0 {
                    format!("\r\nName: {}\r\nItems: {:?}", user.name.yellow(), user.items)
                } else {
                    format!("You have not started a game yet. Use the [{}] new command.", "new".green())
                }
            }
            _ => {
                format!("{}", "INVALID HELP TERM".red().bold())
            }
        };
        resp
    }
}


struct Commands {
    user: User

}

impl Commands {
    fn new(command: &str, story: &mut Story) {
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
        let start_event_mark = 50;
        let mut user = User::new("");
        match command {
            "help" => {
                let resp = if args.len() == 0 {
                    format!("How to use the help command. You can look up a characters name, or an item name like so: [{}]. [{}]. To show a list of commands use [{}]. Press enter with no command to continue the story.", "help itemnamehere".green(), "help commandnamehere".green(), "help list".green())
                    } else {
                        Help::lookup(&args, user)
                    };
                println!("[{}]: {}", "Help".blue().bold(), &resp);

                }
                "load" => {
                    //load from file, play last progress message
                }
                "new" => {
                    print!("[{}]: ", "Enter name".blue().bold());
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read input.");
                    let input = input.trim();
                    let mut user = User::new(&input);
                    user.progress = 1;
                    save(&mut user);
                    println!("created new user: {}", input);
                }
                "" => {
                    if user.progress > 0 {
                        let route = &user.route;
                        match route.as_str() {
                            "start" => {
                                println!("{}", story.start[&user.progress]);
                            }

                        _ => {
                            println!("you have not started the game yet. use the new command.");
                        }

                        //continue story, if no user then prompt to new game. if user progress num == event mark, start event loop.
                    }
                }
                else {
                    println!("you have not started the game yet. use the new command.");
                }
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
    route: String,
}

impl User {
    fn new(name: &str) -> Self {
        if let Ok(contents) = std::fs::read_to_string("utils.txt"){
            let contents = contents.trim().split(":");
            let contents = contents.collect::<Vec<&str>>();
            if contents.len() < 4 {
            User {
                name: name.to_string(),
                items: vec![],
                progress: 0,
                route: "start".to_string(),
            }
            } else {
                let name = contents[0];
                let items = contents[1];
                let items = items.split("-");
                let items: Vec<String> = items.map(|x| x.trim().to_string()).collect();
                User {
                    name: name.to_string(),
                    items: items,
                    progress: contents[2].parse().expect("reason"),
                    route: contents[3].to_string(),
                }

            }
        } else {
            let _file = File::create("utils.txt");
            User {
                name: name.to_string(),
                items: vec![],
                progress: 0,
                route: "start".to_string(),
            }
        }
    }
}

fn main() {

    let mut gamerunning = true;
    let mut story = Story::init();
    println!("Welcome to the game. Please type the command [{}] to get started, or if you know what to do, please use whatever {} you wish!!!", "help".green(), "command".green());

    while gamerunning {
        print!("[{}]: ", "Command".blue().bold());
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let input = input.trim();
        if input == "quit" {
            gamerunning = false;
        } else {
            Commands::new(&input, &mut story);
        };
    }
}
