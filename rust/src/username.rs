struct user {
username : String,
email : String,
sign_in_count : u64,
active : bool,
}
let user1 = user {
email : String::from("someone@example.com"),
username : String::from("someusername123"),
active : true,
sign_in_count : 1,
};
