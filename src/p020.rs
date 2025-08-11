use num_bigint::BigInt;

pub fn p020() {
    // BigInt does the job :)
    let mut product = BigInt::from(1);
    for i in 2..=100 {
        product *= BigInt::from(i);
    }
    // get the sum of all digits
    let result: u32 = product
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum();
    println!("{}", result);
}