pub fn p007(){
    // counting the prime numbers using 'n'
    let mut n = 0;
    // 'i' is checked to see if it is a prime number
    for i in 2.. {
        let mut is_prime = true;
        // 'j' iterates over possible prime factors of 'i'(as far as needed)
        // if 'i' isn't a prime number there has to be an prime factor smaller than or equal to the square root of 'i'
        for j in 2..=(i as f32).sqrt() as i32 {
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            n += 1;
        }
        if n == 10001 {
            println!("{}", i);
            break;
        }
    }
}