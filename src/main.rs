mod user_input;

fn main() {
    println!("Todo App! What do you want to do?\n1     List all todos\n2     Add todo\n3     Delete todo");
    let choice: i32 = user_input::get_user_input("Enter your choice:");

    match choice {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Invalid input")
    }
}
