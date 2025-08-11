pub fn p021() {
    // 'sum' keeps track of the sum of the amicable numbers
    let mut sum = 0;
    // checking every number from 3 to 9999 (excluding 1 amd 2)
    for i in 3..10000{
        // get the sum of proper divisors: d(i)
        let d = sum_of_divisors(i);
        // check amicable property
        if d != i && sum_of_divisors(d) == i {
            sum += i;
        }
    }
    println!("{}", sum);
}

// for n > 2
fn sum_of_divisors(n: i32) -> i32{
    // every number has the proper divisor 1 (excluding 1)
    let mut sum = 1;
    // reduce cost of iterating by using:
    // if a | c 
    // => exist b with: a * b = c 
    // => b | c
    // and if a < sqrt(b) than b > sqrt(b)
    for i in 2..(n as f32).sqrt() as i32 {
        if n % i == 0 {
            sum += i + n / i;
        }
    }
    // is sqrt(n) is a proper divisor of n?
    if ((n as f32).sqrt() as i32).pow(2) == n {
        sum += (n as f32).sqrt() as i32;
    }
    sum
}