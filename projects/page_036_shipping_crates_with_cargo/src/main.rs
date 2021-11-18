#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_generate_float_given_rng() {
        let expected = true;

        let mut rng: ThreadRng = rand::thread_rng();
        let number = generate_float(&mut rng);

        let actual = number >= 0.0 && number <= 10.0;

        assert_eq!(expected, actual);
    }
}

use rand::prelude::*;

fn generate_float(generator: &mut ThreadRng) -> f64 {
    let placeholder: f64 = generator.gen();

    return placeholder * 10.0;
}

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let random_number = generate_float(&mut rng);
    println!("{}", random_number);
}
