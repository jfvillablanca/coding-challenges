#[allow(dead_code)]
fn concat(vec: &[u64]) -> u64 {
    let mut acc: u64 = 0;
    for elem in vec.iter() {
        acc *= 10;
        dbg!(acc);
        acc += *elem as u64;
    }
    acc
}

#[allow(dead_code)]
fn multiply (n: u8, long_boi: u64) -> u64 {
    let mut long_vec: Vec<u64> = long_boi.to_string()
                              .chars()
                              .map(|x| x.to_digit(10).unwrap() as u64)
                              .collect();
    let mut carry: u64 = 0;
    for digit in long_vec.iter_mut().rev() {
        let product: u64 = n as u64 * *digit as u64 + carry;
        *digit = product % 10;
        carry = product / 10;
    }
    while carry > 0 {
        long_vec.insert(0, carry % 10);
        carry = carry / 10;
    }
    concat(&long_vec)
}

#[allow(dead_code)]
fn factorial (x: u8, stop: u8) -> u64 {
    if x == stop {
        1
    } else {
        // multiply(x, factorial(x-1, stop))
        let y = x as u64 * factorial(x-1, stop);
        dbg!(y);
        y
    }
}

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
    let mut binomial_coeff_result: u64 = 1;
    let max_denominator = std::cmp::max(r, n-r);
    let min_denominator = std::cmp::min(r, n-r);

    let mut counter = n;
    binomial_coeff_result = loop {
        if counter <= max_denominator {
            break binomial_coeff_result;
        }
        binomial_coeff_result *= counter as u64;
        counter -= 1;
    };

    let mut counter = min_denominator;
    binomial_coeff_result = loop {
        if counter <= 1 {
            break binomial_coeff_result;
        }
        binomial_coeff_result /= counter as u64;
        counter -= 1;
    };
    binomial_coeff_result
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
