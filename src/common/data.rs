
use crate::gstr;

use super::TestStructUser;

#[allow(dead_code, unused)]
pub fn get_sample_users_vector_1() -> Vec<TestStructUser> {
    vec![
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
    ]
}

#[allow(dead_code, unused)]
pub fn get_sample_users_array_1() -> [TestStructUser; 2] {
    [
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
    ]
}
