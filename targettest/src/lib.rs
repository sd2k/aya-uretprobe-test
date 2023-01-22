use aya_uretprobe_test_common::Struct;

#[no_mangle]
#[inline(never)]
pub fn foo(s: Struct) -> Struct {
    s
}
