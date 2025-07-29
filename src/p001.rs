pub fn p001() {
    /*
    iterate over all numbers between 1 and 999:
        (1..1000)
    filter all numbers divisible by 3 or 5:
        filter(|x| x % 3 == 0 || x % 5 == 0)
    calculate the sum (as 32bits unsigend integer):
        sum::<u32>()
    */
    println!(
        "{}",
        (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum::<u32>()
    );
}
