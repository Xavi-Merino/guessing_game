use std::char;

fn main() {
   /* In rust there is two big categories of data types: SCALARS and COMPOUNDS. Scalars represent
      single values: integers, floats, chars, boolean. Compounds represent a group of values*/

   /* *****Know that the underscore before the variable is for a variable that it's created but not used
           anywhere *******/

   let _x: i32 = 5; // x is a integer
   let _x: u32 = 5; // x is a unsigned integer
   let _x: i64 = 5_000_000_000; // x is a long integer and can be written with underscores
   let _y: f64 = 5.0; // y is a float
   let _z: char = 'z'; // z is a char

   /* There are the classic operations like addition, subtraction, multiplication, division and remainder */

   let _sum: i32 = 5 + 10; // sum is 15
   let _difference: f64 = 15.7 - 4.8; // difference is 10.9
   let _product: i32 = 7 * 8; // product is 56
   let _quotient: f64 = 32.4 / 43.4; // quotient is 0.746
   let _remainder: i32 = 37 % 6; // remainder is 1

   /* Booleans are pretty simple, they can be true or false */

   let _t: bool = true;
   let _f: bool = false;

   /* Then you have characters that are defined with single quotes */

   let _character:  char = 'j';
   let _emoji: char = '🫠';


   /* Compounds types can group multiple values into one type. Rust has two primitive compound
      types: tuples and arrays */
   
   let _tuple: (i64, f64, char) = (5_000_000, 5.35, 'j'); // tuple is a tuple with a i64, a f64 and a char 

   // There are two ways to access the values of a tuple, with a dot and the index or with pattern matching (destructuring)
   let tuple: (i32, f64, char) = (5_000_000, 5.35, 'j');
   let x: i32 = tuple.0; // x is 5_000_000
   let y: f64 = tuple.1; // y is 5.35
   let z: char = tuple.2; // z is 'j'

   println!("x is {}, y is {}, z {}", x, y, z);


   //this is destructuring
   let tuple: (i32, f64, char) = (5_000_000, 5.35, 'j');
   let (x, y, z) = tuple; // x is 5_000_000, y is 5.35 and z is 'j'
   println!("x is: {}, y is: {}, z: {}", x, y, z);

   /* Arrays are a collection of multiple values of the SAME TYPE. Arrays in rust have a FIXED LENGTH
      and they are useful when you want your data allocated on the stack rather than the heap */
   
   let array: [i32; 5] = [1, 2, 3, 4, 5]; // array is an array of 5 i32

   // You can access the values of an array with the index
   let x: i32 = array[0]; // x is 1
   let y: i32 = array[1]; // y is 2
   println!("first number in the array is: {}\nsecond number in the array is: {}\nthird number in the array is: {}",
             x, y, array[2]);
   
   




}
