use macros::AutoDeref;

#[allow(unused)]
#[derive(Debug, AutoDeref)]
#[deref(field = "inner")]
pub struct RespBulkString {
    inner: String,
    nothing: (),
}

fn main() {
    let s = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
    };
    println!("{:?}", s);
}
