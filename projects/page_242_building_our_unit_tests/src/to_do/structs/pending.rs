#[cfg(test)]
mod pending_test {

    use super::Pending;

    #[test]
    fn new() {
        let expected_status = "pending";
        let title = "washing";
        let expected_title = "washing";

        let pending: Pending = Pending::new(title);
        assert_eq!(expected_status, pending.super_struct.status);
        assert_eq!(expected_title, pending.super_struct.title);
    }
}

use super::base::Base;
use super::traits::create::Create;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Pending {
        let base: Base = Base::new(input_title, "pending");

        return Pending { super_struct: base };
    }
}

impl Create for Pending {}
impl Delete for Pending {}
impl Edit for Pending {}
impl Get for Pending {}
