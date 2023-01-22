use aya_uretprobe_test_common::Struct;

use targettest::foo;

fn main() {
    let s = Struct { a: 1234, b: 5678 };
    let resp = foo(s);
    println!("{:?}", resp);
}
