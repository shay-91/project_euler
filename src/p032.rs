use std::collections::HashSet;

pub fn p032() {
    /*
    a * b = c with |a| + |b| + |c| = 9
    this is only possible with |a| + |b| = 5:
    using 4 digits for a and b:
    9 * 999 < 9999 (max 8 digits) and 99 * 99 < 9999 (max 8 digits)
    => |a| + |b| > 4
    using 6 digits for a and b:
    1 * 10000 = 10000 (min 11 digits) and 10 * 1000 = 10000 (min 11 digits) and ...
    => |a| + |b| < 6
    |a| + |b| = 5 can be achieved with |a| = 1 and |b| = 4 or |a| = 2 and |b| = 3 (that's all we need since * is commutative)
    1) pick all permutations of 5 digits from [9,8,7,6,5,4,3,2,1]
    2) each permutation has two possibilities to be divided into a and b:
        - a consisting of the first digit and b consisting of the last four digits
        - a consisting of the first two digits and b consisting of the last three digits
    3) check if a * b is exactly using the reamining digits from [9,8,7,6,5,4,3,2,1]
    */
    let mut sum: HashSet<u32> = HashSet::new();
    let nine_digits = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    for i in 0..nine_digits.len() {
        let number = nine_digits[i];
        let mut eight_digits = nine_digits.clone();
        eight_digits.remove(i);
        for j in 0..eight_digits.len() {
            let number = number * 10 + eight_digits[j];
            let mut seven_digits = eight_digits.clone();
            seven_digits.remove(j);
            for j in 0..seven_digits.len() {
                let number = number * 10 + seven_digits[j];
                let mut six_digits = seven_digits.clone();
                six_digits.remove(j);
                for j in 0..six_digits.len() {
                    let number = number * 10 + six_digits[j];
                    let mut five_digits = six_digits.clone();
                    five_digits.remove(j);
                    for j in 0..five_digits.len() {
                        let number = number * 10 + five_digits[j];
                        let mut four_digits = five_digits.clone();
                        four_digits.remove(j);
                        let product = number / 10000 * (number % 10000);
                        if is_pandigital(product, four_digits.clone()){
                            sum.insert(product);
                        }
                        let product = number / 1000 * (number % 1000);
                        if is_pandigital(product, four_digits){
                            sum.insert(product);
                        }
                    }
                }
            }
        }
    }
    println!("{}", sum.iter().sum::<u32>());
}

fn is_pandigital(mut n: u32, mut digits: Vec<u32>) -> bool { 
    for _i in 0..4 {
        let digit = n % 10;
        n /= 10;
        if !digits.contains(&digit) {
            return false;
        }
        digits.retain(|&x| x != digit);
    }
    // n can initially have more than 4 digits (in this case it s not pandigital)
    n == 0
}
