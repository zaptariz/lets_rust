use std::io;

#[derive(Debug)]
struct table_1 {
    user_name: String,
    user_email_id: String,
    signin_count: u32,
    active: bool,
}

pub fn structure() {
    // can give a value to user_1 using struct name
    let mut user_1 = table_1 {
        user_name: String::from("zaptariz"),
        user_email_id: String::from("zaptariz@mail.com"),
        signin_count: 1,
        active: true,
    };
    println!(
        "user name : {}\nemail id : {}",
        user_1.user_name, user_1.user_email_id
    );
    // can modify the user_1 values here by dot (.) notation.
    let mut user1 = user_1;
    user1.user_name = String::from("biqaltay");
    // this .. mean balance values are same from new_user function.
    let user2 = table_1 {
        user_name: String::from("suriya"),
        ..user1
        
    };

    println!("user1 is {:?}", user2);
    


    fn new_user(user_name: String, user_email_id: String) -> table_1 {
        return table_1 {
            user_name,
            user_email_id,
            signin_count: 2,
            active: false,
        };
    }
    let auto = new_user(String::from("jalabul"), String::from("email.com"));
    println!("this is auto : {}", auto.signin_count);
}
