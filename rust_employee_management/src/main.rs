use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    map.insert(String::from("ENG"), Vec::new());
    map.insert(String::from("MATH"), Vec::new());
    map.insert(String::from("BIOL"), Vec::new());

    loop {
        print!("Enter a name: ");
        io::stdout().flush().expect("Unable to flush buffer!!!");
        let mut input_name = String::new();
        io::stdin().read_line(&mut input_name)
                .expect("Invalid Input!!!");
        let input_name = input_name.trim();

        print!("Enter a department: ");
        io::stdout().flush().expect("Unable to flush buffer!!!");
        let mut input_department = String::new();
        io::stdin().read_line(&mut input_department)
                .expect("Invalid Input!!!");
        let input_department = input_department.trim();

        println!("Adding {} to {} department", input_name, input_department);

        if map.contains_key(input_department) {
            if let Some(list) = map.get_mut(input_department) {
                list.push(String::from(input_name));
            } else {
                println!("User does not exists in system");
            }
        } else {
            println!("Department does not exists in system");
        }

        print!("Would you like to continue (YES/OTHER): ");
        io::stdout().flush().expect("Unable to flush buffer!!!");
        let mut input_continue = String::new();
        io::stdin().read_line(&mut input_continue)
                .expect("Invalid Input!!!");
        let input_continue = input_continue.trim();

        if input_continue.eq("YES") {
            continue;
        }
        
        break;
    }

    println!("{:?}", map);
}
