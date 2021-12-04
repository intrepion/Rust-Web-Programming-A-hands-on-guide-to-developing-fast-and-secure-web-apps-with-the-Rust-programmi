#[cfg(test)]
mod done_test {

    use super::Done;

    #[test]
    fn new() {
        let expected_status = "done";
        let title = "excel date";
        let expected_title = "excel date";

        let done: Done = Done::new(title);
        assert_eq!(expected_status, done.super_struct.status);
        assert_eq!(expected_title, done.super_struct.title);
    }
}

use super::base::Base;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Done {
        let base: Base = Base::new(input_title, "done");

        return Done { super_struct: base };
    }
}

impl Delete for Done {}
impl Edit for Done {}
impl Get for Done {}
