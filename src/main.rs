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

    /* Boolian 
    let t = true;
    let f: bool = false;
    println!("This statement is both {t} and {f}!");
    */

    /* Character 
    let h = 'H';
    let e: char = 'i';
    let happi_cat = '😸';
    println!("{h}{e}{happi_cat}");
    */

    /* Tuple 
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred}|{six_point_four}|{one}");
    */

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = ["Monday", "Tuesday", "Wednesday"];
    let num = arr[1];
    let day = arr2[1];
    println!("{day} is the {num}nd day of the week")
}