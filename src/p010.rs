pub fn p010(){
    /* 
    get all numbers from 2 to 1_999_999:
        (2..2_000_000)
    filter all prime numbers:
        filter(|x| is_prime(*x))
    calculate the sum of all prime numbers:
        sum::<u64>()
    */
    println!("{}", (2..2_000_000).filter(|x| is_prime(*x)).sum::<u64>());
}

fn is_prime(num: u64) -> bool {
    /*
    'i' iterates over all possible prime factors (and a lot of non prime numbers too :D)
    if 'num' isn't a prime number, there has to be an prime factor smaller than or equal to the square root of 'num'
    */
    for i in 2..=(num as f32).sqrt() as i32 {
        if num as i32 % i == 0 {
            return false;
        }
    }
    true
}