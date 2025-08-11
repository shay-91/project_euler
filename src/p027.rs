pub fn p027() {
    let mut max_a = 0;
    let mut max_b = 0;
    let mut max_n = 0;
    // n^2 + na + b => b has to be prime (otherwise n^2 + na + b is no prime for n = 0)
    let possbile_bs: Vec<i32> = (2..1000).filter(|x| is_prime(x)).collect();
    // iterate over every possible combination of as and bs
    for a in -999..=999 {
        for b in &possbile_bs {
            // start with n = 1 since n^2 + na + b is true for n = 0 because b is prime
            for n in 1.. {
                let num = n * n + n * a + b;
                // n < 2 avoids negative numbers, 0 and 1 as input for is_prime()
                if num < 2 || !is_prime(&num){
                    // update the maximum
                    if n > max_n {
                        max_n = n;
                        max_a = a;
                        max_b = *b;
                    }
                    break;
                }
            }
        }
    }
    println!("{}", max_a * max_b);
}

fn is_prime(n: &i32) -> bool  {
    // if 'n' isn't a prime number there has to be an prime factor smaller than or equal to the square root of 'n'
    for i in 2..=(*n as f32).sqrt() as i32 {
        if n % i == 0 {
            return false
        }
    }
    true
}