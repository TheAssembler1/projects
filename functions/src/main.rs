fn main() {
    let x = another_function(5); 
    println!("another_function{x}");
}

fn another_function(x: i32) -> i32 {
   x + 1 
}
