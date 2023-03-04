#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::{every, gstr, some};
    struct TestStructUser {
        name: String,
        age: i32,
    }
    // add tests in future
    #[test]
    fn test_every_and_some() {
        let users = vec![
            TestStructUser {
                name: gstr("Himanshu Jangid"),
                age: 20,
            },
            {
                TestStructUser {
                    name: gstr("Harsh Agarwal"),
                    age: 21,
                }
            },
        ];

        let users = vec![
            TestStructUser {
                name: gstr("Himanshu Jangid"),
                age: 20,
            },
            {
                TestStructUser {
                    name: gstr("Harsh Agarwal"),
                    age: 21,
                }
            },
        ];

        let is_everyone_above_10 = every(&users, |user, _index| user.age > 10);
        let is_everyone_above_20 = every(&users, |user, _index| user.age > 20);
        let are_some_above_20 = some(&users, |user, _index| user.age > 20);
        let are_some_above_30 = some(&users, |user, _index| user.age > 30);
        assert_eq!(is_everyone_above_10, true);
        assert_eq!(is_everyone_above_20, false);
        assert_eq!(are_some_above_20, true);
        assert_eq!(are_some_above_30, false);
    }
    
}
