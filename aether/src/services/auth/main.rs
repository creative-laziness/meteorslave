use lib::models::auth::Signin;

fn main() {
    let signin = Signin {
        username: "username".to_owned(),
        password: "password".to_owned(),
    };

    println!("auth");
    println!("{signin:?}");
}
