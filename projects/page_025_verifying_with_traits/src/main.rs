#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn succeed_some_function_given_run() {
        let expected = ();
        let actual = some_function();

        assert_eq!(expected, actual);
    }
}

trait CanCreate {
    fn create(&self) {
        println!("user is creating");
    }
}

trait CanDelete {
    fn delete(&self) {
        println!("user is deleting");
    }
}

trait CanEdit {
    fn edit(&self) {
        println!("user is editing");
    }
}

struct AdminUser {
    name: String,
    password: String,
}

impl CanCreate for AdminUser {}
impl CanDelete for AdminUser {}
impl CanEdit for AdminUser {}

fn delete<T: CanDelete>(user: T) -> () {
    user.delete();
}

struct BaseUser {
    name: String,
    password: String,
}

struct GeneralUser {
    super_struct: BaseUser,
    team: String,
}

impl GeneralUser {
    fn new(name: String, password: String, team: String) -> GeneralUser {
        return GeneralUser {
            super_struct: BaseUser { name, password },
            team: team,
        };
    }
}

impl CanEdit for GeneralUser {}

impl CanCreate for GeneralUser {
    fn create(&self) -> () {
        println!(
            "{} is creating under a {} team",
            self.super_struct.name, self.team
        );
    }
}

fn some_function() {}

fn main() {
    some_function();
}
