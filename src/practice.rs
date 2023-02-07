// // Rust Documentation   
// #![allow(dead_code)] 
// #[derive(Debug)]
// // #![allow(unreachable_code)] 
// // Structure 
// struct Person {
//     name: String,
//     age: u8,
// }
// struct Unit;
// struct Pair(i32, f32);
// struct Point {
//     x: f32,
//     y: f32,
// } 
// struct Rectangle {  
//     top_left: Point,
//     bottom_right: Point,
// } 
// struct Answer<T, U>{   
//     x: T,
//     y: U,
// } 
// //Enum 
// enum Direction{
//     Up,
//     // Down,
//     // Left,
//     // Right 
// }
// // Function  
// fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
//     if rhs == 0 {
//         return false;
//     }
//     lhs % rhs == 0 
// }
// fn fizzbuzz(n: u32) -> () {      
//     let boolean  = is_divisible_by(n, 2);
//     match boolean {    
//         true  =>println!("{} is Even Number",n), 
//         false =>println!("{} is a ODD Number",n),     
//     }
// }  
// fn fizzbuzz_to(n: u32) {
//     for n in 1..=n {
//         fizzbuzz(n);
//     }
// } 
// //
// fn res(){
//     a();
// }
// fn a(){
//     panic!("MARKET CRASH!!"); 
// } 
// // MAIN FUNCTION 
// fn main(){ 
//     println!("Hello World!"); 
// // tuples 
//     let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true); 
//     println!("long tuple first value: {}", long_tuple.0);
//     println!("long tuple second value: {}", long_tuple.1);         

//     let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
//     println!("tuple of tuples: {:?}", tuple_of_tuples);


// // Array & Slices 
//      let xs: [i32; 5] = [1, 2, 3, 4, 5]; 
//     let sxs   =&xs[2..4];

//     println!("first element of the array: {}", xs[0]);
//     println!("second element of the array: {}", xs[1]);

//     println!("number of elements in array: {}", xs.len());
//     println!("number of elements in array: {:?}", sxs); 

// // Structure 
//     let name = String::from("Suman Saurabh");
//     let age = 21;  
//     let peter = Person { name, age };

//     println!("{:?}", peter);

//     let point: Point = Point { x: 10.3, y: 0.4 };

//     println!("point coordinates: ({}, {})", point.x, point.y);

//     let bottom_right = Point { x: 5.2, ..point };

//     println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

//     let Point { x: left_edge, y: top_edge } = point;

//     let _rectangle = Rectangle { 
//         top_left: Point { x: left_edge, y: top_edge },
//         bottom_right: bottom_right,
//     };

//     let _unit = Unit;

//     let pair = Pair(1, 0.1); 

//     println!("pair contains {:?} and {:?}", pair.0, pair.1);

//     let Pair(integer, decimal) = pair;

//     println!("pair contains {:?} and {:?}", integer, decimal); 


// // Enum type - you tube dcode
//     let player_direction: Direction = Direction::Up; 
//     match player_direction {   
//         Direction::Up => println!("We are headinig up"),  
//     //  Direction::Down => println("We are headinig up");
//     //  Direction::Left => println("We are headinig up");
//     //  Direction::Right=> println("We are headinig up"); 
//         }  
// // Variable Bindings - mutability, Scope and Shadowing, Declare first, Freezing 
//     let an_integer = 1u32; 
//     let a_boolean = true;
//     let unit = ();

//     let copied_integer = an_integer;

//     println!("An integer: {:?}", copied_integer);
//     println!("A boolean: {:?}", a_boolean);
//     println!("Meet the unit value: {:?}", unit);

//     let _unused_variable = 3u32;

//     let _noisy_unused_variabe  = 2u32;      


// // Casting    
//     let decimal = 65.4321_f32; 
//     let integer = decimal as u8;
//     let character = integer as char; 
//     println!("Casting: {} -> {} -> {}", decimal, integer, character); 
//     println!("1000 as a u16 is: {}", 1000 as u16);  
//     println!("  -1 as a u8 is : {}", (-1i8) as u8); 
//     println!("1000 mod 256 is : {}", 1000 % 256);
//     println!(" 128 as a i16 is: {}", 128 as i16);    
//     println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
//     println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
//     println!("   nan as u8 is : {}", f32::NAN as u8);
// // Literals 
//     let x = 1u8; 
//     let y = 2u32;
//     let z = 3f32; 
//     let i = 1;
//     let f = 1.0;

//     println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
//     println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
//     println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
//     println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
//     println!("size of `f` in bytes: {}", std::mem::size_of_val(&f)); 

// // inference
//     let _elem = 5u8;
//     let vec: Vec<u8> = Vec::new(); 
//     // vec.push(elem);
//     println!("{:?}", vec);

// //Aliasing 
//     type NanoSecond = u64;
//     type Inch = u64;
//     type U64 = u64;

//     let nanoseconds: NanoSecond = 5 as U64;
//     let inches: Inch = 2 as U64;

//     println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);

// //Flow of Control - if/else, Nesting and labels
//     let mut count = 0u32;
//     println!("Let's count until infinity!");
//     loop {
//         count += 1;
//         if count == 3 {
//             println!("three");
//             continue;
//         }
//         println!("{}", count);
//         if count == 5 {
//             println!("OK, that's enough");
//             break;
//         }
//     }
//     //Nesting and labels 
//     'outer: loop {
//         println!("Entered the outer loop");
//         'inner: loop {
//             println!("Entered the inner loop");
//             break 'outer;
//         }
//         // println!("This point will never be reached");
//     }
//     println!("Exited the outer loop");
// //for and range 
// for n in 1..21 { 
//         if n % 15 == 0 {
//             println!("{} is divisible by 15", n); 
//         } else if n % 3 == 0 {
//             println!("{} is divisible by 3",n);
//         } else if n % 5 == 0 {
//             println!("{} is divisible by 5",n); 
//         } else {
//             println!("{}", n);
//         }
//     } 
// //Function     
// fizzbuzz_to(10);     

// // You tube - Traversy media| Rustlang 
// print::run();               

// // Error Handling
// // panic!("Crash and Burn !");    
// // res();  
// // enum Result<T, E>{
// //     OK(T);
// //     Err(E); 
// // 
// // ans();    // Doubt    
// // Generic
// let p5: Answer<i32, f64> = Answer{x:50, y:20.0};   
// println!("{}, {}",p5.x , p5.y);      
// // Traits       
// // const PI : f32 =  3.14;   

// // trait Shape{
// //     fn new(length: f32, width: f32)-> Self;
// //     fn area(&self)->f32;
// // }
// // struct Rectangles {length: f32, width: f32}
// // struct Circle {length: f32, width: f32}

// // impl Shape for Rectangles{
// //     fn new(length: f32, width: f32) -> Rectangles{
// //         return Rectangles{length, width};  
// //     }
// //     fn area(&self) -> f32{
// //         return self.length * self.width; 
// //     }
// // }  
// // impl Shape for Circle{  
// //     fn new(length: f32, width: f32) -> Circle{ 
// //         return Circle{length, width};  
// //     }
// //     fn area(&self) -> f32{
// //         return (self.length/2.0).powf(2.0) * PI;   
// //     }
// // }

// // let rec: Rectangles = <dyn Shape as Trait::new>::(length:10.0, width: 10.0);              // 404 
// // let circ: Circle = Shape::new{length: 10.0, width: 10.0}; 

// // println!("Rectangle Area : {}", rec.area()); 
// // println!("Circle Area: {}", circ.area()); 

// // Iterator    
// let var = vec![1,2,3,4,5];
// let mut ans = var.iter();
// println!("{:?}", ans);  
// for val in ans{
//     println!("Vector Number is : {}", val); 
// }
// // println!("{:?}", ans.next()); 

// // closure  



// } // Main Function Closing Bracket 

// //Generic 
// // strut Answer<T, U>{  
// //     x: T;
// //     y: U;
// // }


// // Error handling check   
// // std::num::ParseIntError; 
// // fn ans()  -> Result<(), ParseIntError> { 
// //     let number_str = "10"; 
// //     let number = match number_str.parse::<i32>() {
// //         Ok(number)  => number,
// //         Err(e) => return Err(e),
// //     };
// //     println!("{}", number);
// //     Ok(())
// // } 
// mod print;  