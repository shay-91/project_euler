pub fn p004(){
    // get a vector of all products of two 3-digit numbers:
    let mut products: Vec<i32> = (100..=999).map(|x| (x..=999).map(move |y| x * y)).flatten().collect();
    // sort the vector from largest to smallest element:
    products.sort();
    products.reverse();
    // get the first number that is a palindrome:
    println!("{}", products.into_iter().find(|x| is_palindrome(*x)).unwrap());
}

fn is_palindrome(num: i32) -> bool {
    let chars: Vec<char> = num.to_string().chars().collect();
    let mut reversed = chars.clone();
    reversed.reverse();
    chars == reversed
}