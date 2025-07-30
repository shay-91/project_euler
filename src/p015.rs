use num_bigint::BigInt;

pub fn p015() {
    /*
    u need 40 steps. 20 of them are to the right and 20 of them are down
    => 40 choose 20
    using bigint because numbers like 40! are quite large
    */
    println!(
        "{}",
        fac(BigInt::from(40)) / (fac(BigInt::from(20)) * fac(BigInt::from(40 - 20)))
    );
}

fn fac(num: BigInt) -> BigInt {
    if num == BigInt::from(1) {
        return BigInt::from(1);
    } else {
        return num.clone() * fac(num - BigInt::from(1));
    }
}
