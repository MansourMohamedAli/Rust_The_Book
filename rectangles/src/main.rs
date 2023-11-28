//fn main() {
//    let width = 30;
//	let height = 50;
//	println!("The area of the rectangle is {} square pixels",
//	area(width,height));
//}
//
//fn area(width:u32, height:u32) -> u32
//{
//	width * height
//}

/*
Issue with calculating area this way is it's not clear
in our program that the parameters are related. We can
try using Tuples:
*/

///// Refactoring with Tuples /////
//fn main()
//{
//	let rect1 = (30, 50);
//	println!("The area of the rectangle is {} square pixels",
//	area(rect1));
//}
//
//fn area(dimensions: (u32,u32)) -> u32
//{
//	dimensions.0 * dimensions.1
//}

/*
Both better and worse. referencing dimensions using index
makes it a little unclear. especially if width and height
order matters.
*/

///// Refactoring with Structs /////
//struct Dimensions
//{
//	width: u32,
//	height: u32,
//}
//
//fn area(dimensions:&Dimensions) -> u32
//{
//	dimensions.width * dimensions.height
//}
//
//fn main()
//{
//	let rectangle = Dimensions
//	{
//		width:30,
//		height:50,
//	};
//	println!("Area of rectangle is {}", area(&rectangle));
//}

struct Dimensions
{
	width:i32,
	height:i32,
}
fn main()
{
	
}