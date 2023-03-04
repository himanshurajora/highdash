# Highdash

#### The lodash alternative in Rust. 

You have decided switch to Rust from JavaScript but are missing some of the cool libraries such as `lodash`. This library tries to implement Lodash in rust and gives the experience as close as possible to lodash. Currently It is in development and has only implemented a few array methods. I am constantly working on it and adding new features. Hopefully this library will serve its purpose.


> Installation

`cargo add highdash`

> Example

```rust
use highdash::{gstr, map};

#[allow(unused)]
#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}
fn main() {
    let user = User {
        name: gstr("Himanshu"),
        age: 20,
    };

    // It works for both arrays and vectors
    let users1 = vec![user];

    let names_only = map::<User, String>(&users1, |user, _i| return format!("{_i}: {}", user.name));

    print!("{:?}", names_only);
}

// Output: ["0: Himanshu"]

```

> Currently, Open For Contribution


Thanks for visiting!

Made by Vedik Dev: [Himanshu Jangid](https://github.com/himanshurajora)

