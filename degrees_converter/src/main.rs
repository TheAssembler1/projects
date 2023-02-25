use std::io;

const STD_RATIO_F: f64 = 5_f64/ 9_f64;
const STD_RATIO_C: f64 = 9_f64/ 5_f64;

fn f_to_c(f: f64) -> f64 {
    (f - 32_f64) * STD_RATIO_F
}

fn c_to_f(c: f64) -> f64 {
    (c * STD_RATIO_C) + 32_f64
}

fn main() {
    println!("What is your starting unit (F/C)?");
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("failed to read line");
    let unit = unit.trim();

    println!("What is your starting number (int)?");
    let mut base_number = String::new();
    io::stdin()
        .read_line(&mut base_number)
        .expect("failed to read line");
    let base_number: i32 = base_number.trim().parse().expect("failed to parse number");

    println!("{base_number}{unit}");

    if unit == "F" {
        println!("{base_number}{unit}={}C", f_to_c(base_number as f64));
    } else if  unit == "C" {
        println!("{base_number}{unit}={}F", c_to_f(base_number as f64));
    } else {
        println!("Invalid Unit");
    }
}
