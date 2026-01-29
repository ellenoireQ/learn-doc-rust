/// Implementing from https://doc.rust-lang.org/book/ch10-01-syntax.html
pub struct SocialMedia<'a> {
    pub heading: &'a str,
    pub creator: &'a str,
    pub body: &'a str,
    pub created_at: &'a str,
}

impl<'a> SocialMedia<'a> {
    pub fn new(heading: &'a str, creator: &'a str, body: &'a str, created_at: &'a str) -> Self {
        Self {
            heading: heading,
            creator: creator,
            body: body,
            created_at: created_at,
        }
    }
}

pub fn lifetime() {
    let heading = String::from("Hello World");
    let creator = String::from("John Doe");
    let body = String::from("Learning Rust lifetimes");
    let created_at = String::from("2026-01-29");

    let post = SocialMedia::new(&heading, &creator, &body, &created_at);

    println!(
        "Heading: {}\n Creator: {}\n Body: {}\n Created At: {}",
        post.heading, post.creator, post.body, post.created_at
    );
}
