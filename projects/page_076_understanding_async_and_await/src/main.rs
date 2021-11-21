#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_do_something_given_run() {
        let expected = 2;
        let actual = block_on(do_something(1));

        assert_eq!(expected, actual);
    }
}

use futures::executor::block_on;
use std::{thread, time};

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

fn main() {
    let now = time::Instant::now();
    let future_one = do_something(1);
    let outcome = block_on(future_one);

    println!("time elapsed {:?}", now.elapsed());
    println!("here is the outcome: {}", outcome);

    let future_two = async { return do_something(2).await };

    let future_two = block_on(future_two);
    println!("here is the outcome: {:?}", future_two);
}
