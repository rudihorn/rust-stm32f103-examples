
#[lang = "panic_fmt"]
#[no_mangle]  
#[allow(private_no_mangle_fns)]                                                                   
pub extern fn rust_begin_unwind(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop { }                                         
}


#[lang = "eh_personality"]
#[no_mangle]
#[allow(private_no_mangle_fns)]                                                                   
pub extern fn rust_eh_unwind_resume() {
} 

