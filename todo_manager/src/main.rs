use chrono::{NaiveDate, Utc};
use std::io;

struct Task {
        title: String,
        desc: String,
        priority: u8,
        date: NaiveDate,
        open: bool,
    }

struct Info {
    vector_size: u32,
    saved: bool,
}

fn main()
{
    let mut tasks_vector: Vec<Task> = Vec::new();
    let mut state: u8 = 5;
    let mut choise: u8 = 5;
    let mut input = String::new();
    let mut current_info = Info {
        vector_size: 0,
        saved: false,
    };


    while state != 4
    {
        display_main_menu();
        state = input_validity(&mut input, 0, 4);
        input.clear();
        if state == 0 {
            print!("\x1B[2J\x1B[1;1H");
            println!("                                  ===============");
            println!("                                  New task mode : ");
            println!("                                  ===============");
            println!("          Are you sure you want to create a new task ? (yes = 1/ no = 0)       ");
            println!("\n");

            choise = input_validity(&mut input, 0, 1);
            input.clear();

            if choise == 1 {
                create_new_task(&mut tasks_vector);
                current_info.vector_size += 1;
                print!("\x1B[2J\x1B[1;1H");
                continue;
            } else if choise == 0 {
                print!("\x1B[2J\x1B[1;1H");
                continue;
            }
        } else if state == 1 {
            print!("\x1B[2J\x1B[1;1H");
            display_tasks_title(&tasks_vector);
            println!("                                  Type 1 to fully display tasks, type 0 to return to main menu : ");
            choise = input_validity(&mut input, 0, 1);
            input.clear();

            if choise == 1 {
                display_tasks_detail(&tasks_vector);
                continue;
            } else if choise == 0 {
                print!("\x1B[2J\x1B[1;1H");
                continue;
            }
        }
     
    }
    // for element in &tasks_vector
    // {
    //     println!("Titre : '{}', description : '{}'\n", element.title, element.desc);
    // }
}

fn create_new_task(tasks_vector: &mut Vec<Task>) 
{
    let mut input: String = String::new();
    let mut title: String = String::new();
    let mut desc: String = String::new();

    println!("Rentrer le titre de la tache : ");

    io::stdin().read_line(&mut input).expect("Erreur de lecture de l'entrée");

    title = input.trim().to_string();
    input.clear();

    println!("Rentrer la description de la tache '{}' : ", title);

    io::stdin().read_line(&mut input).expect("Erreur de lecture de l'entrée");

    desc = input.trim().to_string();
    input.clear();

    println!("\n");

    tasks_vector.push(create_new_struct(
        title,
        desc,
        3));
}

fn create_new_struct(title: String, desc: String, priority: u8) -> Task{
    Task { 
        title,
        desc,
        priority,
        date: Utc::now().date_naive(),
        open: true
    }
}

fn input_validity(input: &mut String, low: u8, high: u8) -> u8 {
    loop {
        io::stdin().read_line(input).expect("Failed to read line");

        match input.trim().parse::<u8>() {
            Ok(number) => {
                if number >= low && number <= high {
                    return number;
                } else {
                    println!("Invalid input. Please enter a number between {} and {}. Please try again.", low, high);
                    input.clear();
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a number between {} and {}. Please try again.", low, high);
                input.clear();
            }
        }
    }
}

fn display_main_menu()
{
    println!("\n");
    println!(r"   /$$                     /$$                              /$$        /$$$$$$");
    println!(r"  | $$                    | $$                            /$$$$       /$$$_  $$");
    println!(r"  /$$$$$$   /$$$$$$   /$$$$$$$  /$$$$$$        /$$    /$$|_  $$      | $$$$\ $$");
    println!(r" |_  $$_/  /$$__  $$ /$$__  $$ /$$__  $$      |  $$  /$$/  | $$      | $$ $$ $$");
    println!(r"  | $$    | $$  \ $$| $$  | $$| $$  \ $$       \  $$/$$/   | $$      | $$\ $$$$");
    println!(r"  | $$ /$$| $$  | $$| $$  | $$| $$  | $$        \  $$$/    | $$      | $$ \ $$$");
    println!(r"  |  $$$$/|  $$$$$$/|  $$$$$$$|  $$$$$$/         \  $/    /$$$$$$ /$$|  $$$$$$/");
    println!(r"   \___/   \______/  \_______/ \______/           \_/    |______/|__/ \______/");
    println!("\n");
    println!(r"                                [0] : Create mode                           ");
    println!(r"                                [1] : Display mode                          ");
    println!(r"                                [2] : Edit mode                             ");
    println!(r"                                [3] : Delete mode                           ");
    println!(r"                                [4] : Quit                                  ");
    println!("\n");
    println!("                               Please choose a mode :                        ");
}

fn display_tasks_title(task_vector: &Vec<Task>)
{
    let mut i: u32 = 0;
    println!("                                  ===============");
    println!("                                  Current tasks :");
    println!("                                  ===============");
    println!("\n");
    for element in task_vector
    {
        println!("                                  [{}] Title '{}'", i, element.title);
        i += 1;
    }
}

fn display_tasks_detail(task_vector: &Vec<Task>)
{
    let mut i: u32 = 0;
    println!("                                  ===============");
    println!("                                  Tasks details :");
    println!("                                  ===============");
    println!("\n");
    for element in task_vector
    {
        println!("                                  [{}] Tilte : '{}'", i, element.title);
        println!("                                       Description : '{}'", element.desc);
        println!("                                       Priority : '{}'", element.priority);
        println!("                                       Creation date : '{}'", element.date);
        i += 1;
    }
}