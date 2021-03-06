use std::fmt;

extern crate rustc_serialize;

#[derive(RustcDecodable, RustcEncodable)]
pub struct UserModel {
    pub id: u64,
    pub nickname: String,
    pub mobile: String,
    pub email: String,
    //status: i8,
    //ctime: String,
    //mtime: String,
}

impl UserModel {
    pub fn create_empty() -> UserModel {
        UserModel {
            id: 0,
            nickname: "".to_string(),
            mobile:   "".to_string(),
            email:    "".to_string(),
        }
    }

    pub fn create(id: u64, nickname: String, mobile: String, email: String) -> UserModel {
        UserModel {
            id: id,
            nickname: nickname,
            mobile:   mobile,
            email:    email,
        }
    }
}

impl fmt::Display for UserModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf: String = "".to_string();
        buf = buf + &format!("id: {:?}, ", self.id);
        buf = buf + &format!("nickname: \"{}\", ", self.nickname);
        buf = buf + &format!("mobile: \"{}\", ", self.mobile);
        buf = buf + &format!("email: \"{}\"", self.email);

        write!(f, "UserModel:{{{}}}", buf)
    }
}
