use std::collections::HashMap;
use itertools::iproduct;

/*
cannot return [], return [""] instead
*/

fn combine_vec(array_1: Vec<String>, array_2: Vec<String>) -> Vec<String> {
    let product: Vec<String> = iproduct!(array_1, array_2)
        .map(|(a, b)| format!("{}{}", a, b))
        .collect();
    println!("product: {:?}", product);
    product
}

fn get_possible_combinations(input: &str) -> Vec<String> {
    // validate input
    if input.len() > 4  || input.len() == 0 {
        println!("Error, input is too long (4 characters maximum).");
        return vec!["".to_string()]; // equivalent to ["".to_string()].to_vec()
    }    
    let accepted_char = vec!['2', '3', '4', '5', '6', '7', '8', '9'];
    for digit in input.chars() {
        if !accepted_char.contains(&digit) {
            println!("Error, input digit out of scope (from 2 to 9).");
            return vec!["".to_string()]; // equivalent to ["".to_string()].to_vec()
        }
    }

    let number_letters_map = HashMap::from([
        ('2', ["a".to_string(), "b".to_string(), "c".to_string()].to_vec()),
        ('3', ["d".to_string(), "e".to_string(), "f".to_string()].to_vec()),
        ('4', ["g".to_string(), "h".to_string(), "i".to_string()].to_vec()),
        ('5', ["j".to_string(), "k".to_string(), "l".to_string()].to_vec()),
        ('6', ["m".to_string(), "n".to_string(), "o".to_string()].to_vec()),
        ('7', ["p".to_string(), "q".to_string(), "r".to_string(), "s".to_string()].to_vec()),
        ('8', ["t".to_string(), "u".to_string(), "v".to_string()].to_vec()),
        ('9', ["w".to_string(), "x".to_string(), "y".to_string(), "z".to_string()].to_vec()),
    ]);

    let mut letters_1 = ["".to_string()].to_vec();
    let mut letters_2 = ["".to_string()].to_vec();

    let first_char = input.chars().next().unwrap();
    match number_letters_map.get(&first_char) {
        Some(value) => letters_1 = value.to_vec(),
        None => println!("Empty input"),
    }

    let mut product = letters_1.clone();
    for i in 1..=input.len()-1 {
        let next_char = input.chars().nth(i).unwrap();
        match number_letters_map.get(&next_char) {
            Some(value) => letters_2 = value.to_vec(),
            None => println!("Reached end of input"),
        }
        product = combine_vec(letters_1.clone(), letters_2.clone());
        letters_1 = product.clone();
    }

    product
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(get_possible_combinations("23"), ["ad","ae","af","bd","be","bf","cd","ce","cf"]);
    }

    #[test]
    fn example_2() {
        assert_eq!(get_possible_combinations(""), [""]);
    }

    #[test]
    fn example_3() {
        assert_eq!(get_possible_combinations("2"), ["a","b","c"]);
    }

    #[test]
    fn example_4() {
        assert_eq!(get_possible_combinations("12345"), [""]);
    }

    #[test]
    fn example_5() {
        assert_eq!(get_possible_combinations("1"), [""]);
    }

}