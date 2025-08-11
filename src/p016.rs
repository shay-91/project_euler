use num_bigint::BigInt;

pub fn p016() {
    // BigInt does the job :)
    let product = BigInt::from(2).pow(1000);
    // get the sum of all digits
    let result: u32 = product
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .sum();
    println!("{}", result);
}
