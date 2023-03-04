use crate::{common::{TestStructUser, get_sample_users_vector_1, get_sample_users_array_1}, every};

#[test]
pub fn every_test() {
    let users = get_sample_users_vector_1();

    let is_everyone_above_10 = every(&users, |user, _index| user.age > 10);
    let is_everyone_above_20 = every(&users, |user, _index| user.age > 20);
 
    assert_eq!(is_everyone_above_10, true);
    assert_eq!(is_everyone_above_20, false);

    let users = get_sample_users_array_1();

    let is_everyone_above_10 = every(&users, |user, _index| user.age > 10);
    let is_everyone_above_20 = every(&users, |user, _index| user.age > 20);
 
    assert_eq!(is_everyone_above_10, true);
    assert_eq!(is_everyone_above_20, false);

}
