struct User {
    // Example on how to create a struct User
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn instantiating_a_user() {
    let user1 = User { // User is intanciated
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1, // The order is not important
    };

    println!("User email: {}", user1.email); // Access to object attribute, pretty normal

    //user1.email = String::from("anotheremail@example.com"); nope, user1 this is immutable!

    let mut user2 = User { // User is intanciated as MUTABLE
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1, // The order is not important
    };

    user2.email = String::from("anotheremail@example.com"); // -> Mutate the thing now


    // Or we can have a build_user function:

    let user3 = build_user(String::from("someone@example.com"),
                           String::from("someusername123"));

    println!("User email: {}", user3.email)
}


fn build_user(email: String, username: String) -> User { // Having parameter names equal to the attribute name
    User {
        email, // allows you to do this crazy stuff
        username, // no need to `username: username,`
        active: true,
        sign_in_count: 1,
    }
}


fn struct_update_wtf() {

    let user1 = build_user(String::from("someone@example.com"),
                           String::from("someusername123"));

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // Can be replaced by this:

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // This creates a new instance and copies the values from user1
    // to fill out user3. Wild.
    // TODO: How this behaves on memory? are just references?

}

fn tuple_structs() {
    // Just tuples with names. NamedTuples in python but with no named
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // used like
    let black = Color(0,0,0);
    let origin = Point(0, 0, 0);

    // Two above have different types, hence they are different.
}

fn unit_structs() {
    // TODO: WTF is this?
}


struct User2 {
    /*
        String was used here instead of &str so the data is owned by the instance
        or valid as long as the entire struct is valid.
     */
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,

    /*
    struct User {
        username: &str, // A Lifetime needs to be specified
        email: &str,
        sign_in_count: u64,
        active: bool,
    }
     */
}
// Check chapter10 for lifetime
