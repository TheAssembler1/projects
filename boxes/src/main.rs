use std::ops::Deref;

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmarkPointer with data '{}'", self.data);
    }
}

fn main() {
   let c = CustomSmartPointer {
        data: String::from("my stuff")
   };
   let d = CustomSmartPointer {
        data: String::from("other stuff")
   };

   println!("CustomSmartPointers Created.");
}

fn hello(name: &str) {
    println!("Hello, {name}");
}
