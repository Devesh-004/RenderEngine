use std::io;
fn main(){
    println!("keybord input- ");
    let mut input= String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    println!("Enter an integer- ");
    let mut num= String::new();
    io::stdin()
    .read_line(&mut num)
        .expect("Not valid integer");
    let number:i32= num
        .trim()
        .parse()
        .expect("Invalid integer");
    
    println!("Enter new integer- ");
    let mut num2= String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("");
    let num2: u32= num2.trim().parse().expect("Not valid");

    println!("Entered String is {}", input.trim());
    println!("The input integer is {}", number);
    println!("The new input integer is {}", num2);
}