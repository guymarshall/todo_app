mod user_input;

fn main() {
    println!("Todo App! What do you want to do?");
    println!("1     List all todos");
    println!("2     Add todo");
    println!("3     Delete todo");
    let choice: i32 = user_input::get_user_input("Enter your choice:");

    match choice {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Invalid input")
    }
}
