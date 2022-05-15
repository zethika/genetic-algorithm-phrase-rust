use random_number::random;
const RANDOM_CHARSET: &str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!@#$%^&*()_+-={}[]:\";'\\|,.<>/?`~ ";

pub fn generate_char_vec() -> Vec<char> {
    RANDOM_CHARSET.chars().collect()
}

pub fn generate_random_string(length: u32, vec_charset: &Vec<char>) -> Vec<char> {
    let mut random_result: Vec<char> = Vec::new();
    let mut i: u8;
    for _ in 0..length {
        i = random!(..=(vec_charset.len() - 1) as u8);
        random_result.push(vec_charset[i as usize])
    }
    random_result
}