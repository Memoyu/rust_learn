fn main() {
    // 不安全的能力：
    // 解引用裸指针。
    // 调用不安全的函数或方法。
    // 访问或修改可变的静态变量。
    // 实现不安全trait。

    // 可以在安全代码中创建裸指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // 但却不能通过解引用裸指针来读取其指向的数据
    // 需要在unsafe块中操作
    unsafe {
        println!("r1 is:{}", *r1);
        println!("r2 is:{}", *r2);
    }
}
