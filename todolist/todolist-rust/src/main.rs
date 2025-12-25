use std::{thread::sleep, time};

use crate::{
    liststuff::lists::List,
    tools::filesys::{load_file, read_file_to_string, write_all_to_file},
};
mod liststuff;
mod tools;
fn main() -> std::io::Result<()> {
    //BEWARE IF ERROR OCCURS YOUR TODO LIST WILL BE YEETED IF ITS NOT SAVEDDDDDD
    spriggy_talks("Welcome to Todo App!");
    let mut file = load_file("Joe.txt")?;
    let contents = read_file_to_string(&mut file)?;
    let mut todolist = if contents.is_empty() {
        List::new()
    } else {
        List::from_str(&contents)?
    };
    'TalkingBoop: loop {
        spriggy_talks(
            "What would you like to do?
Add a task;
Delete a task;
View current tasks;
Star a task;
Exit;
        ",
        );
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "add" => {
                spriggy_talks("Whats tasks to add?");
                let what_to_add = read_input()?;
                todolist.add_item(todolist.items.len() + 1, &what_to_add);
                spriggy_talks("\nTask added!");
                write_all_to_file(&mut file, &todolist.to_string())?;
            }
            "delete" => {
                spriggy_talks("What index to delete?");
                let what_to_delete = read_input()?
                    .parse::<usize>()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                todolist
                    .delete_item(what_to_delete)
                    .ok_or(std::io::ErrorKind::NotFound)?;
                spriggy_talks("Deletion Completed! Hopefully you finished it?\n");
                write_all_to_file(&mut file, &todolist.to_string())?;
            }
            "view" => {
                println!("{}\n", &format!("\n{}", &todolist.to_string()));
            }
            "star" => {
                spriggy_talks("What item index to star?\n");
                let idx = read_input()?
                    .parse::<usize>()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                todolist
                    .star_something(idx)
                    .ok_or(std::io::ErrorKind::NotFound)?;
                spriggy_talks("Item Starred! Better put the grind to it heh?\n");
                write_all_to_file(&mut file, &todolist.to_string())?;
            }
            _ => {
                spriggy_talks("And...");
                sleep(time::Duration::from_millis(1000));
                spriggy_talks("Done! Goodbye User!");
                break 'TalkingBoop;
            }
        }
    }
    Ok(())
}

fn spriggy_talks(content: &str) {
    println!("Helper Spriggy: {}", content)
}

fn read_input() -> std::io::Result<String> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?.to_string();

    Ok(buf.trim().to_string())
}
