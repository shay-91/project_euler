pub fn p019() {
    // 'count' is the number of sundays, which fell on the firth of a month
    let mut count = 0;
    // 'days' since 31.12.1899
    // 01.01.1900 is a monday so if 'days' % 7 == 0 than the day is a sunday
    let mut days = 1;
    // iterating from 1900 to 2000
    for i in 0..=100 {
        // iterating from january (1) to december (12)
        for j in 1..=12 {
            // relevant range is 1901 to 2000 (excluding the year 1900)
            if i > 0 && days % 7 == 0 {
                count += 1;
            }
            days += match j {
                // months with 30 days
                4 | 6 | 9 | 11 => 30,
                // february has 28 or 29 days (leap year)
                2 => {
                    let year = 1900 + i;
                    if year % 400 == 0 || (year % 100 != 0 && year % 4 == 0) {
                        29
                    } else {
                        28
                    }
                }
                // all other months have 31 days
                _ => 31,
            };
        }
    }
    println!("{}", count);
}
