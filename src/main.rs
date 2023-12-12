fn main() {
    /* Mut variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    */
    /* Variable shadowing
    let x = 5;
    let x = x+1;
    {
        let x = x*2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    */

    /* Numeric operations
    // addition
    let sum: i8 = 5+10;
    // subtraction
    let difference = 95.5-4.3;
    // multiplication
    let product = 4*30;
    // division
    let quotient = 56.7/32.2;
    let truncated = -5/3;
    // remainder
    let remainder = 43%5;
    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Product: {product}");
    println!("Quotient: {quotient}");
    println!("Truncated: {truncated}");
    println!("Remainder: {remainder}");
    */

    let t = true;
    let f: bool = false;
    println!("This statement is both {t} and {f}!");
}