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

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main()
{
	let x = build_user("yerr".to_string(),"yerr@yerr".to_string());
	println!("{}", x.username)
}

fn build_user(email: String, username: String) -> User
{
	User
	{
		active:true,
		//username: username,
		username,
		//email: email,
		email,
		sign_in_count: 1,
	}
}

/*
if the parameter names and the struct field names are exactly
the same, you can use shorthand for for field.
*/