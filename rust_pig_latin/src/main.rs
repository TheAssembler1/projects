const VOWELS: [char; 10] = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];

fn main() {
    let word = String::from("apple");


    println!("{}", convert_to_pig_latin(&word));
}

fn convert_to_pig_latin(word: &String) -> String {
    let first_char = word.chars().nth(0).unwrap();
    let new_word = &word[1..];

    let mut result = String::from(new_word);
    let mut vowel = false;

    for letter in VOWELS {
        if letter == first_char {
            vowel = true;
            break;
        }
    }

    if vowel {
        result.push('h');
    } else {
        result.push(first_char);
    }

    result.push_str("ay");
    result
}
