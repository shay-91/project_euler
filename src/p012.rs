pub fn p012(){
    // calculate every triangle number untill there is one with more than 500 divisors
    let mut triangle = 0;
    for i in 1.. {
        triangle += i;
        let n = divisors(triangle);
        if n > 500 {
            println!("{}", triangle);
            break;
        }
    }
}

fn divisors(num: i32) -> i32{
    let mut n = 0;
    // if the square root of 'num' is a divisor of 'num' there is one more divisor
    if num % (num as f32).sqrt() as i32 == 0 {
        n += 1;
    }
    /* 
    every divisor smaller than the square root delivers a second divisor:
    if x divides z, there is an y with: z = x * y
    since x is smaller than the sqaure root of z, y has to be bigger than the square root of z 
    */
    for i in 1..(num as f32).sqrt() as i32 {
        if num % i == 0 {
            n += 2;
        }
    }
    n
}