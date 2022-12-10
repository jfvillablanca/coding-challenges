fn factorial (x: usize) -> usize {
    if x == 0 {
        1
    } else {
        let z = x * factorial(x-1);
        println!("x {x} z {z}");
        z
    }
}

fn binomial_coefficient(n: u8, r: u8) -> usize {
    factorial(n.into()) / (factorial(r.into()) * (factorial((n-r).into())))
}

// fn generate_diagonal (base: u8, l: usize) -> Vec<u64> {
//     todo!()
// }

fn main() {
    // generate_diagonal()
}
