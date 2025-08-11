pub fn p023() {
    // get vector of all possible abundant summands
    let mut amicable_numbers: Vec<i32> =  vec![];
    // 12 is the smallest abundant number (see problem)
    for i in 12..28123 {
        if sum_of_divisors(i) > i {
            amicable_numbers.push(i);
        }
    }
    // get every number smaller than 28124 which is the sum of two abundant numvers
    let mut is_sum: Vec<bool> = (1..28124).map(|_| false).collect();
    for i in &amicable_numbers {
        for j in &amicable_numbers {
            if i + j > 28123 {
                break;
            }
            is_sum[(i + j - 1) as usize] = true;
        }
    }
    println!(
        "{}",
        is_sum
            .iter()
            .enumerate()
            .map(|(index, b)| if !*b { index + 1 } else { 0 })
            .sum::<usize>()
    );
}

// for n > 2
fn sum_of_divisors(n: i32) -> i32 {
    // every number has the proper divisor 1 (excluding 1)
    let mut sum = 1;
    // reduce cost of iterating by using:
    // if a | c
    // => there exists b with: a * b = c
    // => b | c
    // and if a < sqrt(b) than b > sqrt(b)
    for i in 2..=(n as f32).sqrt() as i32 {
        if n % i == 0 {
            sum += i + n / i;
        }
    }
    // is sqrt(n) is a proper divisor of n?
    if ((n as f32).sqrt() as i32).pow(2) == n {
        sum -= (n as f32).sqrt() as i32;
    }
    sum
}
