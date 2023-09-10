use std::collections::HashMap;

fn sock_pairs(socks_pile: &str) -> u8 {
    println!("input {}", socks_pile);

    let mut socket_count: HashMap<char, u8> = HashMap::new();
    
    for socket in socks_pile.chars() {
        println!("found socket {}", socket);
        
        match socket_count.get(&socket) {
            Some(count) => {  socket_count.insert(socket, count + 1);},
            None => { socket_count.insert(socket, 1);},
        }
    }
    println!("counted sockets: {:?}", socket_count);

    let mut sockets_pair = 0;
    for val in socket_count.values(){
        sockets_pair += val / 2;
    }
    sockets_pair
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1(){
        assert_eq!(sock_pairs("AA"), 1);
    }

    #[test]
    fn example_2(){
        assert_eq!(sock_pairs("ABABC"), 2);
    }

    #[test]
    fn example_3(){
        assert_eq!(sock_pairs("CABBACCC"), 4);
    }
}