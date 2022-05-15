use rand::seq::SliceRandom;
use std::io;

fn main() {
    loop {
        println!{"Enter password length"}
        let mut password_length = String::new();
        io::stdin()
            .read_line(&mut password_length)
            .expect("Failed to read");
        println!("Requested length: {}", password_length);

        let password_length: u16 = match password_length.trim().parse() {
            Ok(num) => num,
            Err(error) => panic!{"Error {}", error},
        };

        let abc = vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H",
            "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y",
            "Z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "@", "#", "_", "%", "/", "[",
            "]", "{", "}", "¨~", "(", ")", "-",
        ];

        let mut choosen_char: String;
        let mut password_vector: Vec<String> = Vec::new();
        while password_vector.len() < password_length as usize {
            let mut rng = rand::thread_rng();
            choosen_char = abc.choose(&mut rng).unwrap().to_string();
            password_vector.push(choosen_char);
        }
        let s = String::from_iter(password_vector);
        println! {"Password is: {}", s};
    }
}
