use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // 使用match匹配操作Result
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 细化错误处理，文件不存在时，则创建文件，创建失败则再panic
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(ec) => panic!("Tried to create file but there was a problem:{:?}", ec),
            },
            other_error => panic!("There was a problem opening the file:{:?}", other_error),
        },
    };

    // 使用闭包的方式
    // 使用if + unwrap_or_else, 省去了match, 代码看起来更整洁
    let f = File::open("hello1.text").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello1.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem:{:?}", error);
            })
        } else {
            panic!("There was a problem opening the file:{:?}", error);
        }
    });

    // 传播错误, 调用函数时，会传播函数处理产生的错误, 由调用方决定该如何处理
    readname_from_file();
}

// 传播错误
fn readname_from_file() -> Result<String, io::Error> {
    // 分步
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    // 链式
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    // 更简短的方式, 从文件中读取字符串是比较常见的操作, 所rust提供了封装方法
    // fs::read_to_string("hello.txt")

    Ok(s)
}
