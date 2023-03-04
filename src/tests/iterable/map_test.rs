use crate::{
    common::{get_sample_users_array_1, get_sample_users_vector_1, TestStructUser},
    map,
};

#[test]
pub fn map_test() {
    let users = get_sample_users_vector_1();

    let result = vec!["Himanshu Jangid".to_string(), "Harsh Agarwal".to_string()];

    let names_only = map(&users, |user, _i| user.name.clone());

    assert_eq!(names_only, result);

    let users = get_sample_users_array_1();

    let names_only = map(&users, |user, _i| user.name.clone());

    assert_eq!(names_only, result);
}
