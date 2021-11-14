#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_get_highest_given_1_2() {

        let expected = &2;
        let actual = get_highest(&1, &2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeed_get_highest_given_2_1() {

        let expected = &2;
        let actual = get_highest(&2, &1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeed_get_highest_given_1_1() {

        let expected = &1;
        let actual = get_highest(&1, &1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeed_filter_given_1_2() {
        let expected = &0;
        let actual = filter(&1, &2);

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeed_filter_given_2_1() {
        let expected = &2;
        let actual = filter(&2, &1);

        assert_eq!(expected, actual);
    }

    #[test]
    fn succeed_filter_given_1_1() {
        let expected = &1;
        let actual = filter(&1, &1);

        assert_eq!(expected, actual);
    }

}

fn get_highest<'a>(first_number: &'a i8, second_number: &'a i8) -> &'a i8 {

    if first_number > second_number {

        first_number
    } else {

        second_number
    }
}

fn filter<'a, 'b>(first_number: &'a i8, second_number: &'b i8) -> &'a i8 {

    if first_number < second_number {

        &0
    } else {

        first_number
    }
}

fn main() {

    let one: i8 = 1;
    let outcome: &i8;
    let outcome2: &i8;
    {

        let two: i8 = 2;
        outcome = get_highest(&one, &two);
        println!("{}", outcome);
        outcome2 = filter(&one, &two);
        println!("{}", outcome2);
    }
}
