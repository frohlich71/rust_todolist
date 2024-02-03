use std::io;

fn main() {
    println!("Welcome to the rust todo list!");
    let mut tasks:Vec<Task> = vec![];

    loop {
        let mut option = String::new();

        println!("What do you wanna do? (Type a number to choose)");
        println!("1 - Add a task.");
        println!("2 - List all tasks");

        io::stdin().read_line(&mut option).expect("Failed to read option.");

        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match option {
            1 => {
                let mut task_name = String::new();

                println!("Type your task");

                io::stdin().read_line(&mut task_name).expect("Failed to read your task");

                let task = Task {
                    name: task_name,
                    status: false,
                };

                tasks.push(task);

                println!("Task added!")
            }
            2 => {
                for task in &tasks {
                    let mut status = String::new();


                    println!("Name: {}Status: {}", task.name, task.status);
                }
            }
            _ => {println!("Not mapped")}
        }
    }

}


struct Task {
    name: String,
    status: bool
}

