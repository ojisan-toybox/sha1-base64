use crypto::digest::Digest;
use crypto::sha1::Sha1;

fn main() {
    let key = "this_is_key".as_bytes();
    let mut hasher = Sha1::new();
    hasher.input(key);
    let sha1_string = hasher.result_str();
    println!("{}", sha1_string);
}
