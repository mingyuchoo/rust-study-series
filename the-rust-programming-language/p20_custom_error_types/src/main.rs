use thiserror::Error;

#[derive(Debug, Error)]
enum ProgramError {
    #[error("signup error")]
    SignUp(#[from] SignUpError),

    #[error("login error")]
    Login(#[from] LoginError),

    #[error("auth error")]
    Auth(#[from] AuthError),

    #[error("create error")]
    Create(#[from] CreateError),

    #[error("select error")]
    Select(#[from] SelectError),

    #[error("update error")]
    Update(#[from] UpdateError),

    #[error("auth error")]
    Delete(#[from] DeleteError),

    #[error("math error")]
    Math(#[from] MathError),
}

#[derive(Debug, Error)]
enum SignUpError {
    #[error("invalid information")]
    InvalidInformation,
}

fn do_signup(name: &str) -> Result<(), SignUpError> {
    match name {
        | "david" => Ok(()),
        | _ => Err(SignUpError::InvalidInformation),
    }
}

#[derive(Debug, Error)]
enum LoginError {
    #[error("invalid id")]
    InvalidId,

    #[error("invalid password")]
    InvalidPassword,
}
fn do_login(email: &str,
            password: &str)
            -> Result<(), LoginError> {
    match email {
        | "david@email.com" => match password {
            | "password" => Ok(()),
            | _ => Err(LoginError::InvalidPassword),
        },
        | _ => Err(LoginError::InvalidId),
    }
}

#[derive(Debug, Error)]
enum AuthError {
    #[error("unauthenticated")]
    Unauthenticated,

    #[error("unauthorized")]
    Unauthorized,
}
fn check_auth(auth: &str) -> Result<(), AuthError> {
    match auth {
        | "admin" => Ok(()),
        | _ => Err(AuthError::Unauthorized),
    }
}

#[derive(Debug, Error)]
enum SelectError {
    #[error("data not found")]
    NotFound,
}

// TODO: create a function here

#[derive(Debug, Error)]
enum CreateError {
    #[error("can not create")]
    CanNotCreate,
}
// TODO: create a function here

#[derive(Debug, Error)]
enum UpdateError {
    #[error("can not update")]
    CanNotUpdate,
}
// TODO: create a function here

#[derive(Debug, Error)]
enum DeleteError {
    #[error("can not delete")]
    CanNotDelete,
}

#[derive(Debug, Error)]
enum MathError {
    #[error("divide by zero error")]
    DivideByZero,
}
// TODO: create a function here

fn run(choice: i32) -> Result<(), ProgramError> {
    match choice {
        | 1 => do_signup("john")?,
        | 2 => do_login("john@email.com", "root")?,
        | 3 => do_login("david@email.com", "root")?,
        | 4 => check_auth("user")?,
        | _ => (),
    };
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", run(1));
    println!("{:?}", run(2));
    println!("{:?}", run(3));
    println!("{:?}", run(4));

    Ok(())
}
