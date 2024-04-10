#[derive(Debug)]
pub struct Signup {
    pub username: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug)]
pub struct Signin {
    pub username: String,
    pub password: String,
}

pub struct UserInfo {
    pub username: String,
    pub name: String,
}

pub struct SignupDto {
    pub request: Signup,
    pub response: UserInfo,
}

pub struct SigninDto {
    pub request: Signin,
    pub response: String,
}
