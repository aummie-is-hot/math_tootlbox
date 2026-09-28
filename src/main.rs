/*
By: <aum markandey>
Date: 2026-09-25
Program Details: <a script that turns your program into a simple math toolbox. It should ask the user for two numbers, then apply different math operations to those numbers and show the results.>
*/

fn main() {
    use std::io;
    println!("Enter a first number:");
     let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    println!("Enter a second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    
let num1 = num1.trim().parse::<f64>();
let num2 = num2.trim().parse::<f64>();

if let (Ok(num1), Ok(num2)) = (num1, num2) {
    if num1<0.0 || num2<0.0{
    println!("Note: You entered a negative number. Be careful with square roots!");
    
}
    println!("Sum: {:.3}", num1 + num2);
    println!("Product: {:.3}", num1 * num2);
    println!("Difference of absolute values: {:.3}", num1.abs() - num2.abs());
    println!("Absolute Square root of first number: {:.3}", num1.abs().sqrt());
    println!("First number rounded: {:.5}", num1.round());
}
else {
    
    println!("Fuck you, you entered invalid numbers. Please enter valid numbers next time.");
}


}
