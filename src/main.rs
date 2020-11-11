use crypto::digest::Digest;
use crypto::sha1::Sha1;
use base64::{encode};

fn main() {
    emit_as_base64();
   
}

/**
 * @returns emit_as_base64() -> NTU4YzZlMmY5MzIxMmQxMGY4YjRhYjFhYzc3MDMxZTJiYTE1NzQ3MQ==
 */
fn emit_as_base64(){
    let key = "this_is_key".as_bytes();
    let mut hasher = Sha1::new();
    hasher.input(key);
    let sha1_string = hasher.result_str();
    println!("sha1: {}", sha1_string);
    let base64_sha1_string = encode(sha1_string);
    println!("base64: {}", base64_sha1_string);
}