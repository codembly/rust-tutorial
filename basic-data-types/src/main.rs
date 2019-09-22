// Basic Data Types in RUST
// Codembly - Rust Tutorial

// This is the main function
fn main() {
    // The statement below allows the compiler to infer var is of integer data type
    let var = 10;
    println!("Codemly! The integer number is: {} ", var);
    // The statement below specify to the compiler the var is of integer data type
    let _var1:i32 =  10;
    println!("Codemly! The integer number is: {} ", _var1);
    // The statements below specify to the compiler the var is of integer data type
    // In this example _var2 assume the maxium value of i32 so adding 1 generate overflow
    // Uncomment to try overflow at runtime.

    //let mut _var2:i32 =  2147483647;
    //_var2 = _var2 + 1;
    //println!("Codemly! The integer number is: {} ", _var2);

    // Floating point type example printing floating point with one and two decimal digit
    let _var3 = 2.0; // f64 is the default floating point type
    let _var4:f32 = 3.0; //f32
    println!("Codemly! The floating point numbers are: {:.1} and {:.2}", _var3, _var4);
    // Boolean type example printing boolean values
    let _var5 = true;
    let _var6:bool = false; //explicit type annotation
    println!("Codemly! The boolean value are: {} and {}", _var5, _var6);
    // Unicode scalar type example char types
    let _var7 = 'z';
    let _var8:char = 'X'; //explicit type annotation
    println!("Codemly! The chars are: {} and {}", _var7, _var8);
    let _snowman:char = '☃';
    println!("Codemly! The snowman symbol is {}", _snowman);
}
