/////// Basic If Expression///////
/* EXPRESSION because it returns a value */
/*
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
*/
////////////////////////////////////////////////////////
/* Condition must be bool! So "if number"
doesn't work*/
////////////////////////////////////////////////////////



/////// Using if in a let Statement ///////
/*
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
*/
/* Whole value of let depends ons which block
executes */




/////// Mismatching types in If blocks////



/*
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" }; 

    println!("The value of number is: {number}");
}
*/
////////////////////////////////////////////////////////
/* Above code won't work becase one arm is an int and
 the other a is a string. Must both be the same types 
 The compiler needs to know the type of number at compile
 time. can't do that if the type is determined at run time.*/ 
 ////////////////////////////////////////////////////////





 //////////////// Loops  //////////////////
 /* 
Rust has three kinds of loops:
 - loop
 - while
 - for
 
 loop executes block forever until explicitely told
 to stop
 */
 /*
 fn main() {
    loop {
        println!("again!");
    }
}
*/
/* 
- Use "break" to get out.
- "Continue" for next iteration.
*/
/*
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
*/
/* Once counter is equal to 10. breaks and use 
semiclon to end statement that assigns to result.
*/
//////////// Loop Labels ///////////
/* Break or continue apply to the inner most loop
by default. but you can optionally specify "loop label":
*/
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // will break inner loop.
            }
            if count == 2 {
                break 'counting_up; // specifiying whcih loop to break out of.
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

////////// While Loop ////////////
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

////////// For loop ////////////////////
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}