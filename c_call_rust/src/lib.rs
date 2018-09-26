// 不使用std库
#![no_std]

// 使用 #[lang = "..."]
// https://doc.rust-lang.org/beta/unstable-book/language-features/lang-items.html
#![feature(lang_items)]

// 导入C语言的函数
extern {
    fn putchar(c: u8);
}

// 包装外部不安全函数
fn puts(s: &str) {
    for c in s.chars() {
        unsafe { putchar(c as u8); }
    }
}

// 导出给C语言的函数
#[no_mangle]    // 禁止在符号表中改名
pub extern "C"  // 符合C标准调用约定
fn func(x: i32) -> i32 {
    puts("Hello C! I'm Rust!\n");
    x * x
}

// 以下是由于不使用std，必须实现的语言相关项目

// 不知道干啥的
#[lang = "eh_personality"]
extern fn eh_personality() {
}

// panic处理程序
#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    puts("PANIC!\n");
    loop { }
}