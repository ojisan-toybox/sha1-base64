use crypto::digest::Digest;
use crypto::sha1::Sha1;
use base64::{encode};

fn main() {
    let key = "this_is_key".as_bytes();
    let mut hasher = Sha1::new();
    hasher.input(key);
    let sha1_string = hasher.result_str();
    let base64_sha1_string = encode(sha1_string);
    println!("{}", base64_sha1_string);
}
