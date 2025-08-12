// simple brute force

pub fn p030() {
    let mut sum = 0;
    /*
    the lower limit is because you need at least two digits to get a sum.
    the upper limit is a (lucky) guess. the numbers range from 4150 to 194_979
    */
    for i in 10..1_000_000 {
        if digit_fifth_powers(i) {
            sum += i;
        }
    }
    println!("{}", sum);
}

fn digit_fifth_powers(n: i32) -> bool{
    let mut remnant = n;
    let mut digits = n;
    while digits > 0 {
        let digit = digits % 10;
        digits /= 10;
        remnant -= digit.pow(5);
        if remnant < 0 {
            return false;
        }
    }
    remnant == 0
}