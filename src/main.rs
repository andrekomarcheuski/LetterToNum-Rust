const ABC: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

fn letters_to_numbers(word: String) -> Vec<i8> {
    let mut nums: Vec<i8> = Vec::new();
    for letter in word.to_uppercase().chars() {
        if ABC.contains(&letter) {
            for (i, j) in ABC.iter().enumerate() {
                if &letter == j {
                    nums.push((i + 1) as i8);
                }
            }
        }
    }
    nums
}

fn numbers_to_letters(nums: Vec<i8>) -> String {
    let mut word = String::new();
    for num in nums {
        if 0 < num && num <= ABC.len() as i8 {
            for (i, j) in ABC.iter().enumerate() {
                if num == ((i + 1) as i8) {
                    word += &*j.to_string();
                    word += " ";
                }
            }
        }
    }
    word
}

fn main() {
    let word_to_cripto = String::from("hello world");
    let nums_to_cripto = vec![8, 5, 12, 12, 15, 23, 15, 18, 12, 4];
    let nums = letters_to_numbers(word_to_cripto);
    let word = numbers_to_letters(nums_to_cripto);
    println!("{}", word);
    for i in nums {
        print!("{} ", i);
    }
}