
fn get_pi(precision: usize) {
    let pi = 3.141592;

    println!("{:.*}", precision, pi);
}

fn main() {
    get_pi(2);
    get_pi(3);
}