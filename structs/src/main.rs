////// Defining and Instantiating Structs ////////
/*
Define name and types of piece of data called "fields".
*/

//struct User {
//    active: bool,
//    username: String,
//    email: String,
//    sign_in_count: u64,
//}



//fn main() {
//    let mut mo = User
//	{
//		active: true,
//		username: String::from("momo"),
//		email: String::from("mo@mo"),
//		sign_in_count: 2,
//	};
//	
//	mo.username = String::from("TEST");
//	println!("{}",mo.username);
//}

/*
The whole instance turns mutable. Rust donen't let us turn 
part of an instance mutable.
*/

//struct User {
//    active: bool,
//    username: String,
//    email: String,
//    sign_in_count: u64,
//}
//
//fn build_user(email: String, username: String) -> User
//{
//	User
//	{
//		active:true,
//		//username: username, // Don't need to do this.
//		username,             // Do this instead
//		//email: email,       // Don't need to do this.
//		email,                // Do this instead
//		sign_in_count: 1,
//	}
//}
//
//fn main()
//{
//	let user1 = build_user("mohamed@email.com".to_string(),"mmansour".to_string());
//	println!("{}", user1.username);
//	let user2 = User
//	{
//		//username:String::from("jimmy"),
//		..user1
//	};
//	println!("{}", user2.username);
//	println!("{}", user1.username);
//}



/*
if the parameter names and the struct field names are exactly
the same, you can use shorthand for for field.
*/

///// Creating instances from other instances //////
/*
See "user2" above. The "..user1" has to come last to fill
remaining fields.
*/


//struct User {
//    active: bool,
//    username: String,
//    email: String,
//    sign_in_count: u64,
//}
//
//
//fn main() {
//    let mut user1 = User {
//        active: true,
//        username: String::from("someusername123"),
//        email: String::from("someone@example.com"),
//        sign_in_count: 1,
//    };
//
//    user1.email = String::from("anotheremail@example.com");
//    let user2 = User {
//        email: String::from("another@example.com"),
//        ..user1
//    };
//	println!("{}",user1.email)
//	
//}

/*
using the = for user2 means we moved user1 to user2 means we can't use
the user1 fields we used to intialize unit 2. if we explitely set email
for user2 like above, we can still use user1.email. We can't use anything
else tho.
*/

///// Tuple Structs /////
/*
Can set struct that is similar to tuple that contain a tuple of types
but no names. for example:
*/

//struct Color(i32,i32,i32);
//struct Point(i32,i32,i32);
//
//fn main()
//{
//	let black = Color(255,255,255);
//	let random = Point(32,43,11);
//	println!("{}", black.0);
//	println!("{}", random.1);
//}

/*
Can access element the same way as traditional struct
by using "." then the index you want to access.
*/

///// Printing Structs Directly/////
/*
In the example below, to println!(rect1), we would have to
put #[derive(debug)]. println! can't handle a struct normally.
the {:?} means we want to output format in debug.
*/
//#[derive(Debug)]
//struct Rectangle {
//    width: u32,
//    height: u32,
//}
//
//fn main() {
//    let rect1 = Rectangle {
//        width: 30,
//        height: 50,
//    };
//
//    println!("rect1 is {:?}", rect1);
//}

/*
Alternativly, use dbg! macro
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}