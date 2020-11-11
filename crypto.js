// cryptoの挙動確認
const key = "this_is_key";

/**
 * @exajmple emit_as_byte() -> 558c6e2f93212d10f8b4ab1ac77031e2ba157471
 */
const emit_as_byte = () => {
  console.log(require("crypto").createHash("sha1").update(key).digest("hex"));
};

/**]
 * @exajmple emit_as_base64() -> VYxuL5MhLRD4tKsax3Ax4roVdHE=
 */
const emit_as_base64 = () => {
  console.log(
    require("crypto").createHash("sha1").update(key).digest("base64")
  );
};

emit_as_byte();
emit_as_base64();
