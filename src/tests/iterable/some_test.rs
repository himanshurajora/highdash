use crate::{common::{TestStructUser, get_sample_users_vector_1, get_sample_users_array_1}, some};

#[test]
pub fn every_test() {
    let users = get_sample_users_vector_1();

    let is_everyone_above_20 = some(&users, |user, _index| user.age > 20);
    let is_everyone_above_30 = some(&users, |user, _index| user.age > 30);
 
    assert_eq!(is_everyone_above_20, true);
    assert_eq!(is_everyone_above_30, false);

    let users = get_sample_users_array_1();

    let is_everyone_above_20 = some(&users, |user, _index| user.age > 20);
    let is_everyone_above_30 = some(&users, |user, _index| user.age > 30);
 
    assert_eq!(is_everyone_above_20, true);
    assert_eq!(is_everyone_above_30, false);

}
