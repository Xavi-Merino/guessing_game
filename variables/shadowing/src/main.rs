fn main() {
    // this is useful when you want to change/mutate a variable till a determined point, this because if we 
    // create the variable with "mut" it's not mandatory to use a must always, the "let" is necessary to change
    // the variable in this scenario, in the compiler we are creating a new variable called "x".
    let x: i32 = 5;
    println!("the content of your variable x is: {}", x);

    let x: i32 = 6;
    println!("the content of your variable being shadowed by changing its value to: {} ", x);

    let x: i32 = x * 2;
    println!("the content of the same variable is shadowed by multiplying by 2: {}", x);

    println!("-------------------------------------------------------------------------------");

    // also really helpful if you want to change the type of a variable in the time, here the same name of
    // a variable is used to contain a string first and the contain an integer Usize.

    let spaces: &str = "    ";
    println!("you have this space: |{}|", spaces);
    
    let spaces: usize = spaces.len();
    println!("that is {} number of spaces", spaces);

    /* this would be a bad use of "mut" 
    let mut space = "    ";
    space = space.len();
    */

}
