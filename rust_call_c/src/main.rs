// 导入C语言函数
extern {
    fn func(x: i32) -> i32;
}

fn main() {
    let x = unsafe { func(2) };
    println!("func(2) = {}", x);
}
