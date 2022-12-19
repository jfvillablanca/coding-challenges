#[allow(dead_code)]
fn count_digits(mut num: u64) -> (u8, u8) {
    let mut n_digits = 0;
    loop {
        if num <= 0 {
            let small_split = n_digits / 2;
            let big_split = n_digits - small_split;
            break (small_split, big_split);
        }
        num /= 10;
        n_digits += 1;
    }
}

#[allow(dead_code)]
fn multiply_karatsuba(x: u64, y: u64) -> u64 {
    // x = x_1 * 10.pow(m) + x_0
    // y = y_1 * 10.pow(m) + y_0
    // z_2 = x_1 * y_1
    // z_0 = x_0 * y_0
    // z_1 = (x_1 + x_0) * (y_1 + y_0) - z_2 - z_0
    // prod = (z_2 * 10.pow(2m)) + (z_1 * 10.pow(m)) + z_0

    let exponent_split = 1;
    
    let x_1 = x / 10_u64.pow(exponent_split);
    let x_0 = x % 10_u64.pow(exponent_split);
    
    let y_1 = y / 10_u64.pow(exponent_split);
    let y_0 = y % 10_u64.pow(exponent_split);

    let z_2 = x_1 * y_1;
    let z_0 = x_0 * y_0;
    let z_1 = ((x_1 + x_0) * (y_1 + y_0)) - z_2 - z_0;

    (z_2 * 10_u64.pow(2 * exponent_split)) + (z_1 * 10_u64.pow(exponent_split)) + z_0
}

#[allow(dead_code)]
fn remove_zero_trail(mut result: u64) -> (u8,u64) {
    let mut n_zero: u8 = 0;
    loop {
        if result % 10 != 0 {
            break (n_zero, result)
        }
        n_zero += 1;
        result /= 10;
    }
}

fn divide_long (long_dividend: &String, divisor: u64) -> u64 {
    let mut current_div = String::new();
    let mut quotient = String::new();
    let mut digit_place = 0;
    
    loop {
        if digit_place == long_dividend.len() {
            return quotient.parse::<u64>().unwrap();
        }
        current_div.push(long_dividend.as_bytes()[digit_place] as char);
        let current_div_int = current_div.parse::<u64>().unwrap();
        let quotient_digit_int = current_div_int / divisor;

        let current_div_int = current_div_int - (quotient_digit_int * divisor);

        let quotient_digit = std::char::from_digit(quotient_digit_int as u32, 10).unwrap(); 
        quotient.push(quotient_digit);

        current_div = current_div_int.to_string();
        digit_place += 1;
    }
}

#[allow(dead_code)]
fn binomial_coefficient(n: u8, r: u8) -> u64 {
    println!("n: {}, r: {}", n, r);

    let mut binomial_coeff_result: u64 = 1;
    let max_denominator = std::cmp::max(r, n-r);
    let min_denominator = std::cmp::min(r, n-r);

    let mut max_denominator_factor = n;
    let mut min_denominator_factor = min_denominator;

    let mut result_string = binomial_coeff_result.to_string();
    loop {
        dbg!(result_string.clone());
        if max_denominator_factor <= max_denominator {
            break;
        }

        let half_len = result_string.len() / 2;
        let top_half = if half_len == 0 { "0" } else { &result_string[..half_len] };
        let bottom_half = &result_string[half_len..].to_string();

        // binomial_coeff_result *= max_denominator_factor as u64;
        let top_half = &(top_half.parse::<u64>().unwrap() * max_denominator_factor as u64).to_string();
        let bottom_half = &(bottom_half.parse::<u64>().unwrap() * max_denominator_factor as u64).to_string();

        dbg!(top_half, bottom_half, max_denominator);
        println!("---");

        result_string = "".to_string();
        result_string.push_str(top_half);
        result_string.push_str(bottom_half);
        max_denominator_factor -= 1;
    };
    println!("bin_coeff_string {:?}", result_string);

    // For the remaining factors unreduced in the previous loop
    loop {
        if min_denominator_factor <= 1 {
            return binomial_coeff_result;
        }
        binomial_coeff_result /= min_denominator_factor as u64;
        min_denominator_factor -= 1;
    };
}

#[allow(dead_code)]
fn generate_diagonal (base: u8, l: usize) -> Vec<u64> {
    let mut diagonal: Vec<u64> = Vec::new();
    for i in 0 .. l {
        diagonal.push(binomial_coefficient(base+i as u8, base) as u64)
    }
    diagonal
}

fn main() {
    println!("{}",binomial_coefficient(58, 46));
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_small_int() {
    //     // assert_eq!(binomial_coefficient(5, 2), 10);
    //     assert_eq!(binomial_coefficient(10, 2), 45);
    // }

    #[test]
    fn test_mid_int() {
        assert_eq!(binomial_coefficient(50, 10), 10_272_278_170);
        assert_eq!(binomial_coefficient(200, 7), 2_283_896_214_600);
    }
    //
    // #[test]
    // fn test_big_int() {
    //     assert_eq!(binomial_coefficient(83, 69), 2_644_778_698_775_100);
    //     assert_eq!(binomial_coefficient(55, 25), 3_085_851_035_479_212);
    // }

    // #[test]
    // fn test_karatsuba() {
    //     assert_eq!(multiply_karatsuba(8, 9), 72);
    //     assert_eq!(multiply_karatsuba(83, 69), 5_727);
    //     assert_eq!(multiply_karatsuba(12_345_678, 99), 1_222_222_122);
    // }

    // #[test]
    // fn test_count_digits() {
    //     assert_eq!(count_digits(2_644_778_698_775_100_000),(9, 10));
    // }
}
