/*
0/5 routes complete
Written by: Herenti
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

struct Events {

}

impl Events {

    fn new(user: &mut User) {
        match user.route.as_str() {
            "start" => {
                match user.progress {
                    50 => {
                        print!("\r\n[{}] This is an event choice enter y/n:", "Event".blue().bold());
                        std::io::stdout().flush().unwrap();
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read input.\r\n");
                        let input = input.trim();
                        let input = input.to_lowercase();
                        let input = input.as_str();
                        match input{
                            "y" => {
                                user.route = "hanako".to_string();
                                user.progress = 1;
                                save(user);

                            }
                            "n" => {
                                println!("\r\nRoute not yet available. please choose another option.\r\n");
                                Events::new(user);
                            }
                            _ => {

                            }
                        }
                    }
                    _ => {

                    }
                }
            }
            _ => {

            }
        }
    }
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

        //story.start.insert(1, format!(""));

        story.start.insert(1, format!("It was a cloudy and brisk day in the small village where my story began. The story of finding the great treasure of my {}.", "heart".red().bold()));
        story.start.insert(2, format!("[{}] Moving is a huge pain.  I have just moved from Tokyo out to a small village in northern Japan, named {}. It is kind of in the middle of nowhere but the nature is completely stunning. Why did I move out here you ask? Well, I am a student of Daiten university, and there is a study program in this area for something quite interesting.", "October 16th".bold(), "Inumura".cyan()));
        story.start.insert(3, format!("There is a new species of bacteria growing in the mountain forests near the village that might lead to some serious developments in cancer research. This small area is the only place this bacteria has been discovered."));
        story.start.insert(4, format!("It's name is {}. In some studies with medicine made from this bacteria, the cancer in lab rats has completely dissapeared. Yet I have heard there are some... strange side effects... Not that the project heads were keen on telling us what they were.", "X-1".cyan()));

        story
    }
}
struct Help {

}

impl Help {
    fn lookup(term: &str, user: User) -> String {
        let resp = match term {
            "list" => {
                "[help, quit, new, history]".to_string()
            }
            "new" => {
                "Start a new game. You will need to enter your name. This name will be used for the main character. This will erase any previous saved game.".to_string()
            }
            "quit" => {
                "Quit the game. The game autosaves regardless.".to_string()
            }
            "history" => {
                "Show the last 5 story segments.".to_string()
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
        let start_events = vec![50];
        let hanako_events = vec![50];
        /*
        let futaba_events = vec![50];
        let takeshi_events = vec![50];
        let hijikata_events = vec![50];
        let kami_events = vec![50];
        */
        let start_event_mark = 50;
        let mut user = User::new("", "c");
        match command {
            "help" => {
                let resp = if args.len() == 0 {
                    format!("How to use the help command. You can look up secific terms or commands like so: [{}]. [{}]. Terms that can be looked up are in the {} color. Commands that can be used are in {}. To show a list of commands use [{}]. Press enter with no command to continue or start the story.", "help termhere".green(), "help commandnamehere".green(), "cyan".cyan(), "green".green(), "help list".green())
                    } else {
                        Help::lookup(&args, user)
                    };
                println!("\r\n[{}]: {}\r\n", "Help".blue().bold(), &resp);

                }

                "new" => {
                    print!("\r\n[{}]: ", "Enter name".blue().bold());
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read input.\r\n");
                    let input = input.trim();
                    let mut user = User::new(&input, "n");
                    user.progress = 1;
                    save(&mut user);
                    println!("\r\nWelcome to the game {}! This game autosaves and autoloads.\r\n", input.cyan());
                }
                "" => {
                    if user.progress > 0 {
                        let route = &user.route;
                        match route.as_str() {
                            "start" => {
                                if story.start.contains_key(&user.progress) {
                                    println!("\r\n[{}]: {}\r\n", "Story".blue().bold(), story.start[&user.progress]);
                                    if start_events.contains(&user.progress) {
                                        Events::new(&mut user);

                                    } else {
                                        user.progress += 1;
                                        save(&mut user);
                                    }

                                }
                            }

                            _ => {

                            }


                        }

                }
                else {
                    println!("\r\nYou have not started the game yet. Use the [{}] command.\r\n", "new".green());
                }
                }

            _ => {

                println!("\r\n{}\r\n", "INVALID COMMAND".red().bold());
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
    fn new(name: &str, operation: &str) -> Self {
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
                if operation == "c" {
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
                } else {
                    User {
                        name: name.to_string(),
                        items: vec![],
                        progress: 0,
                        route: "start".to_string(),
                    }
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
    println!("\r\nWelcome to the game. Please type the command [{}] to get started if you are new. If you know what to do, please use whatever {} you wish!!!\r\n", "help".green(), "command".green());

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
