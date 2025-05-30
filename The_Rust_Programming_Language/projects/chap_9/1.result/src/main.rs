use std::fs::File;
use std::io::ErrorKind;

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
    // 省去了match, 代码看起来更整洁
    let f = File::open("hello1.text").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello1.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem:{:?}", error);
            })
        } else {
            panic!("There was a problem opening the file:{:?}", error);
        }
    });
}
