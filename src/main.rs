use rand::{thread_rng, Rng};

fn main() {
    const MESSAGES: &[&str] = &["Yes", "No", "Maybe", "Probably not", "Not a chance"];

    let mut rng = thread_rng();

    let n = rng.gen_range(0..MESSAGES.len());

    println!("{}", MESSAGES[n]);
}
