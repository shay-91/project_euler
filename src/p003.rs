pub fn p003() {
    /*
    'num' contains 600851475143. in the process it is divided by every found prime factor
    'factor' contains the largest prime factor found so far (except the starting value)
    'i' is checked to see if it is a prime factor of num
    if 'factor' is greater than or equal to num, the largest prime factor was found
    */
    let mut num: u64 = 600851475143;
    let mut factor: u64 = 1;
    let mut i = 2;
    loop {
        if num % i == 0 {
            factor = i;
            num /= factor;
        } else {
            i += 1;
        }
        if factor >= num {
            break;
        }
    }
    println!("{}", factor);
}
