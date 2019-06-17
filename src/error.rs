use failure::{Error as FError, Fail};

pub type Result<T, E = FError> = std::result::Result<T, E>;

#[derive(Fail, Debug)]
pub enum CheckError {
    #[fail(display = "the check wasn't a success ({})", _0)]
    UnsuccessfulCheck(u16),

    #[fail(display = "unequal response")]
    UnequalResponse,
}

#[derive(Fail, Debug)]
pub enum StartError {
    #[fail(display = "couldn't get domain list ({})", _0)]
    DomainListError(u16),

    #[fail(display = "launch error: {}", _0)]
    LaunchError(rocket::error::LaunchError),
}
