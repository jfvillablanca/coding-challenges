fn factorial (x: usize) -> usize {
    if x == 0 {
        1
    } else {
        x * factorial(x-1)
    }
}

fn binomial_coefficient(n: u8, r: u8) -> usize {
    factorial(n.into()) / (factorial(r.into()) * (factorial((n-r).into())))
}

fn generate_diagonal (base: u8, l: usize) -> Vec<u64> {
    let mut diagonal: Vec<u64> = Vec::new();
    for i in 0 .. l {
        diagonal.push(binomial_coefficient(base+i as u8, base) as u64)
    }
    diagonal
}

fn main() {
    let test = generate_diagonal(1, 10);
    println!("test {:?}", test);
}
