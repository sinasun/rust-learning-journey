use rand::Rng;

fn rand_generator() -> f64 {
    rand::thread_rng().gen_range(0..=10_000_000) as f64 / 10_000_000.0
}
fn main() {
    let mut square = 0;
    let mut circle = 0;

    for _ in 1..10_000_000 {
        let x = rand_generator();
        let y = rand_generator();
        let distance: f64 = (x.powi(2) + y.powi(2)).sqrt();

        if distance < 1.0 {
            circle += 1;
        }
        square += 1;
    }

    let pi: f64 = 4.0 * circle as f64 / square as f64;
    println!("Estimate value of pi is equal to {}", pi)
}
