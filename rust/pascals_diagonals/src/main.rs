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

fn binomial_coefficient(n: u8, r: u8) -> u64 {
    println!("n: {}, r: {}", n, r);

    let mut binomial_coeff_result: u64 = 1;
    let max_denominator = std::cmp::max(r, n-r);
    let min_denominator = std::cmp::min(r, n-r);

    let mut counter = n;
    let mut zero_count = 0;
    let mut temp_zero_container;
    binomial_coeff_result = loop {
        if counter <= max_denominator {
            break binomial_coeff_result;
        }
        (temp_zero_container, binomial_coeff_result) = remove_zero_trail(binomial_coeff_result);
        zero_count += temp_zero_container;

        binomial_coeff_result *= counter as u64;
        counter -= 1;
    };

    let mut counter = min_denominator;
    binomial_coeff_result = loop {
        if counter <= 1 {
            break binomial_coeff_result;
        }
        if zero_count > 0 {
            binomial_coeff_result *= 10;
            zero_count -= 1;
        }
        binomial_coeff_result /= counter as u64;
        counter -= 1;
    };
    binomial_coeff_result * 10_u64.pow(zero_count as u32)
}

fn generate_diagonal (base: u8, l: usize) -> Vec<u64> {
    let mut diagonal: Vec<u64> = Vec::new();
    for i in 0 .. l {
        diagonal.push(binomial_coefficient(base+i as u8, base) as u64)
    }
    diagonal
}

fn main() {
    let test = generate_diagonal(100, 5);
    println!("test {:?}", test);
}
