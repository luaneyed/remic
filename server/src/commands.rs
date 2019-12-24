pub enum Command<'a> {
    Get { key: &'a str },
    Set { key: &'a str, val: &'a str },
}