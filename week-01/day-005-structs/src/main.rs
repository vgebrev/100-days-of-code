struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    print_user(&user1);
    let built_user = build_user(String::from("joe.soap@acme.com"), String::from("joe.soap"));
    print_user(&built_user);

    let user2 = User {
        email: String::from("joe.mama@acme.com"),
        username: String::from("joe.mama"),
        ..built_user
    };
    print_user(&user2);

    let _white = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!(
        "{} ({}) active={}, sign in count={}",
        user.username, user.email, user.active, user.sign_in_count
    );
}
