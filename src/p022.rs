use std::fs;

pub fn p022(){
    // input is one string
    let input: String = fs::read_to_string("src/022.txt").unwrap();
    // split the string at '\n' and ','
    let input: Vec<&str> = input.split('\n'). map(|x| x.split(',')).flatten().collect();
    // remove '"' from the strings
    let mut input: Vec<String> = input.iter().map(|x| x.chars().filter(|x| *x != '"').collect()).collect();
    input.sort();
    let result: usize = input.iter().enumerate().map(|(i, name)| (i + 1) * score(name)).sum();
    println!("{}", result);
}

// get the score of a name
fn score(name: &str) -> usize {
    let mut sum = 0;
    for c in name.chars() {
        sum += c as usize - 64;
    }
    sum
}