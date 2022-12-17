fn remove_zero_trail(mut result: u64) -> (u8,u64) {
    dbg!(result);
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
    // TODO:
    // We are still experiencing multiplication overflows
    // Shift to Karatsuba algorithm, (possibly ditch the trailing zeros method)
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
