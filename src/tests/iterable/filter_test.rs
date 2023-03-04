use crate::{
    common::{get_sample_users_array_1, get_sample_users_vector_1, TestStructUser},
    map, filter,
};

#[test]
pub fn map_test() {
    let users = get_sample_users_vector_1();

    let result = TestStructUser {
        name: "Himanshu Jangid".to_string(),
        age: 20,
    };

    let filtered = filter(&users, |user, _i| user.age == 20);

    assert_eq!(filtered[0], result);

    let users = get_sample_users_array_1();

    let filtered = filter(&users, |user, _i| user.age == 20);

    assert_eq!(filtered[0], result);
}
