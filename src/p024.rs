pub fn p024() {
    // the smallest permutation is the permutation 0 => searching for the  permutation 999_999
    let mut n = 1_000_000 - 1;
    // 'digits' contains the remaining digits (from small to big)
    let mut digits: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    // 'result' is used to calculate the solution (every iteration adds one digit to 'result')
    let mut result = 0;
    for i in (0..=9).rev() {
        /*
            example:
            first digit:
                Considering 0 as the first digit of the permutation.
                There are 9! = 362_880 permutations to order the reamining 9 digits (from 123456789 to 987654321)
                since we are looking for the permuation 999_999 and 999_999 is bigger than 362_880 the first digit can't be 0
                following this logic the first digit has to be 2 (3rd digit in the 'digits' vector). permuations beggining with 2 are between permutation 725_760 and 1_088_640
                remaining digits: [0, 1, 3, 4, 5, 6, 7, 8, 9]
            second digit:
                Considering 0 as the second digit of the permutation.
                There are 8! = 40_320 permutations to order the reamining 8 digits (from 13456789 to 98765431)
                since we are looking for the permuation 999_999 - 725_760 and 274_239 is bigger than 40_320 the first digit can't be 0
                following this logic the second digit has to be 7 (7th digit in the 'digits' vector). permuations beggining with 27 are between permutation 967_680 and 1_008_000
                remaing digits: [0, 1, 3, 4, 5, 6, 8, 9]
            and so on...
        */
        let x = n / fac(i) as usize;
        // 'n' are the remaining permutations untill you reach the result
        n -= x * fac(i);
        // update result
        result *= 10;
        result += digits[x];
        // remove the added digit
        digits.remove(x);
    }
    println!("{}", result);
}

fn fac(n: usize) -> usize {
    if n <= 1 { 1 } else { n * fac(n - 1) }
}
