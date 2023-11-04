pub mod p1;

fn str_to_snafu(string: &str) -> i64 {
    let mut num = 0;
    let mut base = 1;
    for ch in string.as_bytes().iter().rev() {
        num += base
            * match ch {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => unreachable!(),
            };
        base *= 5;
    }
    num
}

fn int_to_snafu(mut num: i64) -> String {
    if num == 0 {
        return String::from("0");
    }

    let mut base = 1;
    let mut range = 0;
    loop {
        let min = base - range;
        let max = (2 * base) + range;
        if min <= num && num <= max {
            break;
        }
        range += 2 * base;
        base *= 5;
    }

    let mut output = String::new();
    while num != 0 {
        if num > 0 {
            if num > base + range {
                output.push('2');
                num -= 2 * base;
            } else if num > range {
                output.push('1');
                num -= base;
            } else {
                output.push('0');
            }
        } else {
            if num < -base - range {
                output.push('=');
                num += 2 * base;
            } else if num < -range {
                output.push('-');
                num += base;
            } else {
                output.push('0');
            }
        }
        base /= 5;
        range -= 2 * base;
    }

    while base != 0 {
        output.push('0');
        base /= 5;
    }

    output
}
