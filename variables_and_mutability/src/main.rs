//fn main() {
//    let mut x= 5; // Have to put mut x to allow x = 6 work below.
//	println!("The value of x is: {x}");
//	x = 6;
//	println!("The value of x is: {x}");
//}


///////////////// Constants /////////////////////
//fn main() {
//	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//	println!("three hours in seconds {THREE_HOURS_IN_SECONDS}");
//}
/////////////////////////////////////////////////

///////////////// Shadowing /////////////////////
/* You can declare a variable with the same name as
a previous variable AKA "Shadowing"
*/
/*
fn main() {
	let x = 5;
	let x = x + 1;
	
	{
		let x = x * 2;
		println!("The Value of inner scope is {x}")
	}
	
	println!("The value of x is {x}")
}
*/


fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
	println!("_{spaces}");
}





/////////////////////////////////////////////////