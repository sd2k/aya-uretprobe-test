#![no_std]
#![no_main]

use aya_bpf::{helpers::bpf_probe_read_user, macros::uretprobe, programs::ProbeContext};
use aya_log_ebpf::{error, info};
use aya_uretprobe_test_common::Struct;

#[uretprobe(name = "aya_uretprobe_test")]
pub fn aya_uretprobe_test(ctx: ProbeContext) -> u32 {
    match try_aya_uretprobe_test(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_aya_uretprobe_test(ctx: ProbeContext) -> Result<u32, u32> {
    // This is incorrect: the first arg is not a pointer to `Struct`...
    let s_ptr = ctx.arg::<*const Struct>(0).unwrap();
    let s = unsafe { bpf_probe_read_user(s_ptr) };
    match s {
        Ok(x) => info!(
            &ctx,
            "function foo called by targettest, first arg (address = {}, a = {}, b = {})",
            s_ptr as usize,
            x.a,
            x.b
        ),
        Err(e) => error!(
            &ctx,
            "failed to read pointer from first arg, address {}: {}", s_ptr as usize, e
        ),
    };

    // Instead each field is passed as a separate arg?
    info!(&ctx, "Struct.a is {}", ctx.arg::<usize>(0).unwrap());
    info!(&ctx, "Struct.b is {}", ctx.arg::<usize>(1).unwrap());

    // Similarly with the return value: this isn't actually a pointer, it's just a usize containing
    // the field 'a'...
    let ret_ptr = ctx.ret::<*const usize>().unwrap();
    let ret_a = unsafe { bpf_probe_read_user(ret_ptr) };
    let ret_b = unsafe { bpf_probe_read_user(ret_ptr.add(1)) };
    info!(
        &ctx,
        "what is happening {} {} {}",
        ret_ptr as usize,
        ret_a.unwrap(),
        ret_b.unwrap(),
    );
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
