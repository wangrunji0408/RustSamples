extern crate cc;

fn main() {
    // 在构建Rust项目前，编译指定C语言文件
    // 在构建Rust项目后，链接到生成文件中
    cc::Build::new()
        .file("src/main.c")
        .compile("cobj");
}