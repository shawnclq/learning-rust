use std::io;

struct Task {
    deadline: String,
    completed: bool,
    description: String
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    
    loop {
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Exit");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),// no need to pass mutable reference as we are not modifying tasks
            "3" => mark_task_completed(&mut tasks),
            "4" => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    let mut deadline = String::new();
    let mut description = String::new();
    
    println!("Enter task deadline:");
    io::stdin().read_line(&mut deadline).unwrap();
    
    println!("Enter task description:");
    io::stdin().read_line(&mut description).unwrap();
    
    tasks.push(Task {
        deadline: deadline.trim().to_string(),
        completed: false,
        description: description.trim().to_string(),
    });
    
    println!("Task added successfully.");
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
        return;
    }
    
    for (i, task) in tasks.iter().enumerate() {
        println!("Task {}: {} - {} (Completed: {})", i + 1, task.description, task.deadline, task.completed);
    }
}

fn mark_task_completed(tasks: &mut Vec<Task>) {
    let mut task_number = String::new();
    
    println!("Enter task number to mark as completed:");
    io::stdin().read_line(&mut task_number).unwrap();
    
    let task_index: usize = match task_number.trim().parse::<usize>() {
        Ok(num) => num - 1,
        Err(_) => {
            println!("Invalid input, please enter a valid task number.");
            return;
        }
    };
    
    if task_index < tasks.len() {
        tasks[task_index].completed = true;
        println!("Task marked as completed.");
    } else {
        println!("Task number out of range.");
    }
}