use lib::models::auth::Signup;

fn main() {
    let signup = Signup {
        username: "username".to_owned(),
        password: "password".to_owned(),
        name: "name".to_owned(),
    };
    println!("gateway");
    println!("{signup:?}");
}
