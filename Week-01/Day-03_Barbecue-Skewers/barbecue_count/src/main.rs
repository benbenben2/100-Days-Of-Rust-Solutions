use std::collections::HashMap;

fn barbecue_count(grill: [&str; 5]) -> [u8; 2] {
    let mut skewers = HashMap::new();

    skewers.insert("vegetarian_skewers", 0);
    skewers.insert("non_vegetarian_skewers", 0);

    for skewer in grill {
        if skewer.contains("x") {
            match skewers.get(&"non_vegetarian_skewers") {
                Some(count) => { skewers.insert("non_vegetarian_skewers", count + 1);},
                None => println!("Error key: non_vegetarian_skewers"),
            }
        } else {
            match skewers.get(&"vegetarian_skewers") {
                Some(count) => { skewers.insert("vegetarian_skewers", count + 1);},
                None => println!("Error key: vegetarian_skewers"),
            }
        }
    }

    [*skewers.get(&"vegetarian_skewers").unwrap(), *skewers.get(&"non_vegetarian_skewers").unwrap()]    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input_1 = [
            "--xo--x--ox--",
            "--xx--x--xx--",
            "--oo--o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ];
        assert_eq!(barbecue_count(input_1), [1, 4]);
    }
    #[test]
    fn example_2() {
        let input_2 = [
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ];
        assert_eq!(barbecue_count(input_2), [2, 3]);
    }
    #[test]
    fn example_3() {
        let input_3 = [
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----"
        ];
        assert_eq!(barbecue_count(input_3), [3, 2]);
    }
}