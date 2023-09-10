fn find_nemo(input: &str) -> String {

    let words: Vec<&str> = input.split(' ').collect();
    // let words = input.split(' ');
    println!("list: {:?}", words);
    println!("list length: {}", words.len());

    for index in 0..words.len() {
        println!("Current word: {}", words[index]);
        if words[index] == "Nemo" {
            return format!("I found Nemo at {}!", index+1).to_string();
        }
    }

    "I can't find Nemo :(".to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn find_nemo_tests(){
        assert_eq!(find_nemo("I am finding Nemo !"), "I found Nemo at 4!");
        assert_eq!(find_nemo("Nemo is me"), "I found Nemo at 1!");
        assert_eq!(find_nemo("I Nemo am"), "I found Nemo at 2!");
    }
    #[test]
    fn case_sensitive(){
        assert_eq!(find_nemo("Is it NeMo ?"), "I can't find Nemo :(");
    }
    #[test]
    fn symbol(){
        assert_eq!(find_nemo("Noooo! Nemo!"), "I can't find Nemo :(");
    }
    #[test]
    fn nemo_and_concat(){
        assert_eq!(find_nemo("Nemo's father"), "I can't find Nemo :(");
    }
    #[test]
    fn first_occurence(){
        assert_eq!(find_nemo("Nemo meets Nemo"), "I found Nemo at 1!");
    }
}