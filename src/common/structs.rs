#[allow(unused)]
#[derive(Debug, Clone)]
pub struct TestStructUser {
    pub name: String,
    pub age: i32,
}

impl PartialEq<TestStructUser> for TestStructUser {
    fn eq(&self, other: &TestStructUser) -> bool {
        self.age == other.age && self.name == other.name
    }
}
