////////// Ownership Rules ////////// 

/* 
- Each value in rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
*/

/////////// Strings ///////////////
/*
String literals:
- are hard coded into the code and
are known at run time. Meaning, you don't have to
dynamically allocate it in the heap.
- string literals are immutable.

Not every string can be known at run time. such
as user inputs.

To dynamically allocate string to the heap.
do the following:
*/

//fn main()
//{
//	let s = String::from("hello");
//}


/*
String can be mutated:
*/

//fn main()
//{
//	let mut s = String::from("hello");
//	s.push_str(", world!");
//	println!("{}",s);
//}

///////// Memory and Allocation ////////////
/*
Memory is automatically returned once the variable
the owns it goes out of scope.
*/

//fn main()
//{
//    {
//        let s = String::from("hello"); // s is valid from this point forward
//
//        // do stuff with s
//    }                                  // this scope is now over, and s is no
//                                       // longer valid
//
//}


//fn main()
//{
//	// bind 5 to x, copy value in x and bind it to y:
//	let x = 5;
//	let y = x;
//	
//	/* in the case for dynamically allocated string,
//	we copy the pointer,length, and capacity to the
//	stack. The data on the heap that the pointer refers
//	to is NOT copied.*/
//    let s1 = String::from("hello");
//    let s2 = s1;
//	
//}

/* When s2 and s1 both go out of scope, they will both try
to free the same memory AKA "double free" which would cause
undefined behavior in c++.

To ensure this doesn't happen in Rust. After the line
"let s2 = s1;", Rust considers s1 no longer valid so it
doesn't have to free s1 when it goes out of scope.
*/

//fn main()
//{
//	let s1 = String::from("hello");
//	let s2 = s1;
//	println!("{}",s1);
//}

/*
Shallow copy: copying pointer,length and capacity.
Deep copy: copies actual data.

We say s1 was "moved" to s2 meaning only s2 is valid.
A move is like a shallow copy which also invalidates
the first variable.
*/

/////////////// Clone //////////////////
/* If you do want to make a "deep copy" aka copy the heap data
and not just the stack data, you can use the "clone" method.

*/
//fn main()
//{
//    let s1 = String::from("hello");
//    let s2 = s1.clone();
//
//    println!("s1 = {}, s2 = {}", s1, s2);
//	
//}
/*
Expensive
*/

//////// Stack only data ///////////
//fn main()
//{
//	let x = 5;
//	let y = x;
//	println!("x ={}, y = {}", x, y);
//}
/*
The example with ints works because the size is known
at compile time so they are stored in the stack. calling
method clone wouldn't do anything different from usual shallow
copy.

Rust has a "copy" annotation that can be placed on types stored
in the stack. If a type implements the copy trait, variables that
use it do not move, but are trivially copied. making them available
after assignment to another variable.

Rust won't let us implement "copy" if the type or any of it's parts
has implemented the "drop" trait. Generally simple scalar values can
implement "copy" such asL

- All integers, such as u32.
- boolean
- All floating types
- Char type
- Tuples if they contain types that implement copy.
*/


///////////// Ownership and Functions //////////////////
/*
Similar to assigning values to variables.

Passing a variable to a function will "move" or "copy"
just as an assignment does.

*/

//fn main() {
//    let s = String::from("hello");  // s comes into scope
//
//    takes_ownership(s);             // s's value moves into the function...
//                                    // ... and so is no longer valid here
//
//    let x = 5;                      // x comes into scope
//
//    makes_copy(x);                  // x would move into the function,
//                                    // but i32 is Copy, so it's okay to still
//                                    // use x afterward
//
//} // Here, x goes out of scope, then s. But because s's value was moved, nothing
//  // special happens.
//
//fn takes_ownership(some_string: String) { // some_string comes into scope
//    println!("{}", some_string);
//} // Here, some_string goes out of scope and `drop` is called. The backing
//  // memory is freed.
//
//fn makes_copy(some_integer: i32) { // some_integer comes into scope
//    println!("{}", some_integer);
//} // Here, some_integer goes out of scope. Nothing special happens.

//fn main() {
//    let s1 = gives_ownership();         // gives_ownership moves its return
//                                        // value into s1
//
//    let s2 = String::from("hello");     // s2 comes into scope
//
//    let s3 = takes_and_gives_back(s2);  // s2 is moved into
//                                        // takes_and_gives_back, which also
//                                        // moves its return value into s3
//} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//  // happens. s1 goes out of scope and is dropped.
//
//fn gives_ownership() -> String {             // gives_ownership will move its
//                                             // return value into the function
//                                             // that calls it
//
//    let some_string = String::from("yours"); // some_string comes into scope
//
//    some_string                              // some_string is returned and
//                                             // moves out to the calling
//                                             // function
//}
//
//// This function takes a String and returns one
//fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
//                                                      // scope
//
//    a_string  // a_string is returned and moves out to the calling function
//}

//fn main() {
//    let s1 = String::from("hello");
//
//    let (s2, len) = calculate_length(s1);
//
//    println!("The length of '{}' is {}.", s2, len);
//}
//
//fn calculate_length(s: String) -> (String, usize) {
//    let length = s.len(); // len() returns the length of a String
//
//    (s, length)
//}

/*
The above is length, there is a easier way called "references"
*/