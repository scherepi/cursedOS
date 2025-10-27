use alloc::string::{String, ToString};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::{print, println};
use crate::vga_buffer;

//store what the user is typing b4 pressing enter
lazy_static! {
    pub static ref BUFFER: Mutex<String> = Mutex::new(String::new());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoginStage {Username, Password}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CreateStage { Username, Password1, Password2 }

enum InputMode { Cli,
Login {
    stage: LoginStage,
    username: String,
    buffer: String,
},
CreateUser {
    stage: CreateStage,
    username: String,
    password1: String,
    buffer: String,
},
}

lazy_static! {
    static ref MODE: Mutex<InputMode> = Mutex::new(InputMode::Cli);
}






pub fn startup_screen(){ //startup screen - will probably customise a lil more or something
    println!(" CursedOS v0.1.0 ");
    println!(" 'help' for commands ");
    print_prompt();
}

pub fn print_prompt(){ 
    let who = crate::users::current_user().unwrap_or_else(||"guest".to_string());
    print!("{}@cursedOS> ", who); } //the > thingy you see before you type

pub fn back_to_cli(){
    *MODE.lock() = InputMode::Cli;
    println!("");
    print_prompt();
}


pub fn execute_command(command: &str){//called when the user presses enter. basically takes the command buffer and executes it based on the available commands.
    let trimmed_command = command.trim();
    //self explanatory matches the command written by the user to the commands available.
    match trimmed_command {
        "" => {}, 
        "help" => {
            println!("Available commands:");
            println!("help - you get this message!");
            println!("clear - clear screen");
            println!("about - about cursedOS");
            println!("login - login to an account");
            println!("create user - create a new account");
            println!("create - alias for 'create user'");
            println!("logout - logout of your account");
            println!("whoami - show current user");
        },
        "clear" => {clear_screen();}, 
        "about" => {println!("add some good about text here");},
        "whoami" => {
            let who = crate::users::current_user().unwrap_or_else(|| "guest".to_string());
            println!("current user: {}", who);
        }
        "logout" => {
            crate::users::logout();
            println!("logged out");
        }
        "login" => {
            enter_login_mode();
            return;
        },
        "create" | "create user" => {
            enter_create_user_mode();
            return;
        },
        
        _ => {println!("Unknown command: {}", trimmed_command);},
    }
}




pub fn submit_command(){ //when user presses enter it moves to the next line + clears the buffer for the next command
    // avoiding holding mode lock while running cli commands that may change mode to prevent deadlocking 
    {
        let mode = MODE.lock();
        if matches!(*mode, InputMode::Cli) {
            drop(mode);
            let command = {
                let mut buffer = BUFFER.lock();
                let cmd = buffer.clone();
                buffer.clear();
                cmd
            };
            println!("");            
            execute_command(&command);
            if matches!(*MODE.lock(), InputMode::Cli) {
                print_prompt();
            }
            return;
        }
    }

    let mut mode = MODE.lock();
    match &mut *mode {
        InputMode::Cli => { }//not used anymore

        InputMode::Login{stage, username, buffer} => {
            match *stage {
                LoginStage::Username => {
                    *username = buffer.clone();
                    buffer.clear();
                    *stage = LoginStage::Password;
                    println!("");
                    print!("Password: ");
                    return; 
                } 
            LoginStage::Password => {
                let username = username.clone();
                let password = buffer.clone();
                if crate::users::auth(&username, &password) {
                    println!("");
                    println!("logged in:3 nyan~!");
                    *mode = InputMode::Cli;
                        println!("");
                        print_prompt();
                } else {
                    println!("");
                    login_error("incorrect password");
                    *mode = InputMode::Cli;
                    println!("");
                    print_prompt();
                }
                }
            }
        }


    
        InputMode::CreateUser { stage, username, password1, buffer } => {
        match *stage {
            CreateStage::Username => {
                let name_input = buffer.trim().to_string();
                if name_input.is_empty() {
                    create_error("username cannot be empty");
                } else if crate::users::user_exists(&name_input) {
                    create_error("user already exists");
                } else {
                    *username = name_input;
                    buffer.clear();
                    *stage = CreateStage::Password1;
                    println!("");
                    create_ui(username, password1, buffer, 1);
                }
            },
            CreateStage::Password1 => {
                *password1 = buffer.clone();
                buffer.clear();
                *stage = CreateStage::Password2;
                println!("");
                create_ui(username, password1, buffer, 2);
            }

            CreateStage::Password2 => {
                let password2 = buffer.clone();
                if password2 != *password1 {
                    create_error("passwords do not match");
                    *stage = CreateStage::Password1;
                    buffer.clear(); 
                    println!("");
                    create_ui(username, password1, buffer, 1);
                } else {
                
                    match crate::users::add_user(username, password1) {
                        Ok(()) => {
                            vga_buffer::clear_screen();
                            println!("User created successfully!");
                            *mode = InputMode::Cli;
                            println!("");
                            print_prompt();

                        }
                        Err(e) => {
                            create_error(e);
                            *stage = CreateStage::Username;
                            *username = String::new();
                            *password1 = String::new();
                            buffer.clear();
                            create_ui(&String::new(), &String::new(), &String::new(), 0);
                        }
                    }
                }
            }
        }
    }
}
}

pub fn add_character(c:char){//adds a character to the command buffer
    let mut mode = MODE.lock();
    match &mut *mode {
        InputMode::Cli => {
            BUFFER.lock().push(c);
            print!("{}", c);
        }
        InputMode::Login{stage, username: _, buffer} => {
            match *stage {
                LoginStage::Username => {
                    if c != '\n' {
                        buffer.push(c);
                        print!("{}", c);
                    }
                }
                LoginStage::Password => {
                    if c != '\n' {
                        buffer.push(c);
                        print!("*");
                    }
                }
            }
        }
        InputMode::CreateUser { stage, buffer, .. } => {
            if c != '\n' {
                match *stage {
                    CreateStage::Username => {
                        buffer.push(c);
                        print!("{}", c);
                    }
                    CreateStage::Password1 | CreateStage::Password2 => {
                        buffer.push(c);
                        print!("*");
                    }
                }
            }
        }
    }
}

    //backspace function
pub fn backspace(){
    let mut mode = MODE.lock();
    match &mut *mode {
        InputMode::Cli => {
            let mut buffer = BUFFER.lock();
            if buffer.pop().is_some() {
                print!("\x08 \x08");
            }
        }
        InputMode::Login { stage: _, username: _, buffer } => {
            if buffer.pop().is_some() {
                print!("\x08 \x08");
            }
        }
        InputMode::CreateUser { stage: _, username: _, password1: _, buffer } => {
            if buffer.pop().is_some() {
                print!("\x08 \x08");
            }
        }
    }
}
pub fn clear_screen() {
    vga_buffer::clear_screen_full();
}

fn enter_login_mode(){
    *MODE.lock() = InputMode::Login {
        stage: LoginStage::Username,
        username: String::new(),
        buffer: String::new(),
    };
    login_ui(&String::new(), &String::new(), false);
}

fn login_ui(username: &str, _buffer: &str, _masking: bool){
    if username.is_empty() {
        print!("Username: ");
    } else {
        print!("Password: ");
    }
}

fn login_error(message: &str){
    println!("\nError: {}", message);
}

fn enter_create_user_mode() {
    *MODE.lock() = InputMode::CreateUser {
        stage: CreateStage::Username,
        username: String::new(),
        password1: String::new(),
        buffer: String::new(),
    };
    create_ui(&String::new(), &String::new(), &String::new(), 0);
}

fn create_ui(username: &str, password1: &str, buffer: &str, step: usize){
    match step {
        0 => {
            print!("Enter username: {}", buffer);
        },
        1 => {
            println!("Username: {}", username);
            let password1_shown = "*".repeat(buffer.len());
            print!("Enter password: {}", password1_shown);
        },
        2 => {
            println!("Username: {}", username);
            let password1_shown = "*".repeat(password1.len());
            println!("Password: {}", password1_shown);
            let password2_shown = "*".repeat(buffer.len());
            print!("Confirm password: {}", password2_shown);
        },
        _ => {}
    }
}   

fn create_error(msg: &str) {
    println!("\nError: {}", msg);
}