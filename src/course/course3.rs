struct Aser {
    name: String,
    age: i32,
}

impl Aser {
    fn get_tag(&self) -> String {
        format!("{}_{}", self.name, self.age)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let a = Aser {
            name: "aaa".to_string(),
            age: 1,
        };
        assert_eq!("aaa", a.name);
        assert_eq!(1, a.age);

        let b = Aser { age: 2, ..a };
        assert_eq!("aaa_2", b.get_tag());

        let Aser { age, .. } = b;
        assert_eq!(2, age);
    }
}
