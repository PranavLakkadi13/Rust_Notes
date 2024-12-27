
// here i am publically exporting the Reult Type to handle the error of our custom error 
pub type Result<T> = core::result::Result<T, Error>;

// giving my own type of error enum 
#[derive(Debug)]
pub enum Error {
    LoginFail,
}


// this is the good practise the handle the error and using it to display on the screen during the debug 
// this is basically implementing the parent Display Module to handle the error for my particular type --> 
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt , "{self:?}")
    }
}

// similar to the above making the standard crate handle the custom Error type --> 
impl std::error::Error for Error {}