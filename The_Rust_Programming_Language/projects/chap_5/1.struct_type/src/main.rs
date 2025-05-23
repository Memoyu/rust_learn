fn main() {
    let user = User {
        username: String::from("Tony"),
        email: String::from("tony@qq.com"),
        active: true,
        sign_in_count: 1,
    }

}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
