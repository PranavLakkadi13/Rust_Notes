#[derive(Clone, Debug)]
pub struct Context {
    user_id: i64,
}

// Constructor
impl Context {
    pub fn new(user_id: i64) -> Self {
        Self { user_id }
    }
}

// function on the struct
impl Context {
    pub fn get_user_id(&self) -> i64 {
        self.user_id
    }
}
