use std::collections::HashMap;

fn main() {
    let mut list: [i32; 4] = [5, 4, 3, 2];

    println!("Target List: {:?}", list);

    list.sort();

    if list.len() == 0 {
        println!("List has no median");
    } else if list.len () % 2 == 0 {
        println!("Median: {}", list[(list.len() / 2) - 1]);
    } else {
        println!("Median: {}", list[list.len() / 2]);
    }

    let mut map = HashMap::new();
    let mut max: Option<i32> = None;

    for (_element, index) in list.iter().enumerate() {
        let current_amount = map.entry(*index).or_insert(0);
        *current_amount += 1;
        
        // NOTE: checking if the max value is this index
        match max {
            Some(i) => {
                if i < *current_amount {
                    max = Some(*current_amount);
                }
            },
            None => {
                max = Some(*current_amount);
            }
        }
    }

    if let Some(i) = max {
        println!("Highest Frequency Number: {}", i);
       for (key, value) in &map {
         if i == *value {
            println!("Mode Number: {}", *key);
         }
       }
    } else {
        println!("List of numbers has no mode");
    }

    println!("{:?}", map);
}
