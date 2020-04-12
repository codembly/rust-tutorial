// Functions in RUST
// Codebly - Rust Tutorial
fn main() {
    let var1 = 10;
    let var2 = 5;
    let sum = sum_integers(var1,var2);
    // Print var1, var2 and their sum values to the console
    println!("The sum of {} and {} is {}", var1, var2, sum);
}
//Function definition
fn sum_integers (x:i32, y:i32) -> i32
{
    // explicit value return format
    //return x+y; 
    //Tail return expression format
    x+y
}
