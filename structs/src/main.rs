struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Book1(String, bool, i32);
struct Book2(String, bool, i32);


fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    let book1 = Book1(String::from("Siddharta"), true, 500);
    let book2 = Book2(String::from("Les Miserables"), false, 1200);
    let Book1(title1, read1, pages1) = book1;
    let Book2(title2, read2, pages2) = book2;

    println!("{} {title1} that has {pages1} pages",
        match read1 {
            true => "I read",
            false => "I did NOT read",
        });
    println!("{} {title2} that has {pages2} pages",
        match read2 {
            true => "I read",
            false => "I did NOT read",
        });

    println!("--------------------------------------------------------");
    println!("USER 1: {:?}", user1.email);

    user1.email = String::from("paconoespaco@haha.com");

    println!("USER 1 new email: {:?}", user1.email);
    println!("Calling the build user shit: {:?}", build_user(String::from("paco@paco.es"), String::from("paco")).email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}


