
// this is used to create a sub module inside the helpers module
pub mod namehelpers {
    pub fn get_first_last_name(first: &str, last: &str) -> String {
        // the format! macro is used to help format the code and merge things to return a String
        let fullname = format!("{0} {1}", first, last);
        return fullname;
    }
}


pub mod agefunction {
    pub fn get_age_and_add_5(age: u8) -> u8 {
        return age + 5;
    }
}

