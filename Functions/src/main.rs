/*
Length	Signed	Unsigned
8-bit	i8	    u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
128-bit	i128	u128
arch	isize	usize
*/

//////////// Basic Parameters///////////
/*
fn main() {
    println!("Hello, world!");

    another_function(5);
}

fn another_function(x: i32) {  // Must specify type fpr each parameters.
    println!("Another function.{x}");
}
*/
//////////// Multiple Parameters ///////////
/*
fn main() {
    print_labeled_measurement(5, 'h');
    let y = {
        let x = 3;
        x + 1                  // No semicolon for expressions
    };

    println!("The value of y is: {y}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
*/
////////////// Statement vs Expression ///////////////////

/* Statements never return value and end with semicolon.
   Expressions return value and don't end in semicolon. */

/*
fn main() {
    let y = { 
        let x = 3; 
        x + 1 // No Semicolon because it is an expression
    };

    println!("The value of y is: {y}");
}
*/
//////////// Functions with return values /////////////

/* 
- We don't name return values, but must declare type
with (->).
- In rust, return value of the function is snyonmous with
the value of the final expression.
- You can return early by using "return" keyword and specifying
value.
- most functions return the last line implicitly.
*/

/*
fn five() -> i32 { // Return type is specified to i32.
    5  // 5 is the functions return value. Since expression, no semicolon
}

fn main() {
    let x = five(); 

    println!("The value of x is: {x}");
}
*/

fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1  // putting semicolon here will cause an error.
}

/* this is because the function says it will return type i32.
(-> i32) but putting a semicolon turns it into a statement 
which means there is no return value.*/ 