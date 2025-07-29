pub fn p002() {
    /*
    create a vector of all fibonacci terms whose values do not exceed 4_000_000
    strating with the first two terms: 
        vec![1, 2]
    each iteration of the following loop adds the next term of the fibonacci sequence
    if the value is greater than 4_000_000, the loop terminates
    */
    let mut fib: Vec<u32> = vec![1, 2];
    loop {
        let next_term: u32 = fib[fib.len() - 1] + fib[fib.len() - 2];
        if next_term > 4_000_000 {
            break;
        } else {
            fib.push(next_term);
        }
    }
    /* 
    filter all even fibonacci terms:
        fib.iter().filter(|x| *x % 2 == 0)
    calculate the sum:
        sum::<u32>()
    */
    println!("{}", fib.iter().filter(|x| *x % 2 == 0).sum::<u32>());
}
