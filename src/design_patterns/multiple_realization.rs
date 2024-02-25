#![allow(dead_code)]

// 泛型系统可为一个类型实现多个不同泛型的特征
// 这里实现多个 AsRef

struct Person {
    pub age: u32,
    pub name: String,
}

trait Profession {
    fn describe(&self) -> String;
}

struct Citizen {
    person: Person,
    profession: Box<dyn Profession>,
}

impl AsRef<Person> for Citizen {
    fn as_ref(&self) -> &Person {
        &self.person
    }
}

impl AsRef<Box<dyn Profession>> for Citizen {
    fn as_ref(&self) -> &Box<dyn Profession> {
        &self.profession
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn print_person_info(person: &Person) {
        println!("person: name is {}, age is {}", person.name, person.age)
    }

    fn print_profession_describe(p: &Box<dyn Profession>) {
        println!("profession: describe is {}", p.describe());
    }

    struct Teacher();

    impl Profession for Teacher {
        fn describe(&self) -> String {
            "this is a teacher".into()
        }
    }

    #[test]
    fn test() {
        let alice = Citizen {
            person: Person {
                age: 56,
                name: "Alice".into(),
            },
            profession: Box::new(Teacher()),
        };

        print_person_info(alice.as_ref());
        print_profession_describe(alice.as_ref());
    }
}
