//////////// References and Borrowing //////////////
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
The issue is we have to return the String to the colling function so
we can still use String after the call.
*/

//fn main() {
//    let s1 = String::from("hello");
//
//    let len = calculate_length(&s1);
//
//    println!("The length of '{}' is {}.", s1, len);
//}
//
//fn calculate_length(s: &String) -> usize { // s is a reference to a String
//    s.len()
//} // Here, s goes out of scope. But because it does not have ownership of what
//  // it refers to, it is not dropped.
//}


/*
This code won't work!!!
*/

//fn main() {
//    let s = String::from("hello");
//
//    change(&s);
//}
//
//fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

/*
Have to make the reference mutable:
*/

//fn main() {
//    let s = String::from("hello");
//
//    change(&mut s); // Passing mutable
//}
//
//fn change(some_string: &mut String) { // mutable in function signature
//    some_string.push_str(", world");
//}

/*
One BIG restriction:
- You can only have one mutable reference to a value.
The following code will fail:

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
	
This prevents a "data race" to occur. A data race
is when these three behaviors occur:
- Two ore more pointers access data at the same time.
- At least one of the pointers is being used to write to the data
- There is no mechanicm being used to synchronize access to the data.

Use brackets to create new scope for multiple mutable references. Just
can't do it at the same time.
*/

//fn main()
//{
//    let mut s = String::from("hello");
//
//    {
//        let r1 = &mut s;
//    } // r1 goes out of scope here, so we can make a new reference with no problems.
//
//    let r2 = &mut s;
//}

/*
When combinging mutable and immutable references, This code will error:
*/

//fn main()
//{
//    let mut s = String::from("hello");
//
//    let r1 = &s; // no problem
//    let r2 = &s; // no problem
//    let r3 = &mut s; // BIG PROBLEM
//
//    println!("{}, {}, and {}", r1, r2, r3);
//}

/*
Above code won't work because rust doesn't want r1 and r2's value
to suddenly change from under them. Do this instead:
*/

//fn main()
//{
//    let mut s = String::from("hello");
//
//    let r1 = &s; // no problem
//    let r2 = &s; // no problem
//    println!("{} and {}", r1, r2);
//    // variables r1 and r2 will not be used after this point
//
//    let r3 = &mut s; // no problem
//    println!("{}", r3);
//
//}

/*
The compiler can tell that that the immutable reference is no longer
being used before the end of the scope.
*/

////////////// Dangling References /////////////////
/*
Rust guarantees there are no dangling references.
*/

//fn main() {
//    let reference_to_nothing = dangle();
//}
//
//fn dangle() -> &String {
//    let s = String::from("hello");
//
//    &s
//}
/*
in Dangle, creating variable s then returning the a reference
to it back to main won't work. the memory is deallocated at the end
of dangle.

For this to work, you would have to return "s" directly (drop the &).
*/

