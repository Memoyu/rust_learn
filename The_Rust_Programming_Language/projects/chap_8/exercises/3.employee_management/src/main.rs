use std::collections::HashMap;
use std::io;

fn main() {
    let mut depts = HashMap::new();
    init_depts(&mut depts);
    loop {
        println!(
            "{}",
            String::from(
                "请输入命令:\n\t添加:add {雇员名字} to the {部门}\n\t查询:get {部门 或 all}"
            )
        );

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("非法输入");

        match_cmd(input, &mut depts);
    }
}

fn init_depts(depts: &mut HashMap<String, Vec<String>>) {
    let inputs = vec![
        String::from("Add Sally anny to Engineering"),
        String::from("Add Wade to Engineering"),
        String::from("Add Seth to Engineering"),
        String::from("Add Dave to Engineering"),
        String::from("Add Gilbert to Engineering"),
        String::from("Add Brian to Engineering"),
        String::from("Add Liam to Engineering"),
        String::from("Add Claude to Engineering"),
        String::from("Add Harvey to Engineering"),
        String::from("Add Amir to Sales"),
        String::from("Add Connor to Sales"),
        String::from("Add Kingston to Sales"),
        String::from("Add Harold to Sales"),
        String::from("Add Eli to Sales"),
        String::from("Add Carlos to Sales"),
        String::from("Add Paul to Sales"),
        String::from("Add Ricardo to Sales"),
        String::from("Add Jessie to Manager"),
        String::from("Add Wiley to Manager"),
        String::from("Add Clark to Manager"),
        String::from("Add Marshall to Manager"),
        String::from("Add Marion to Manager"),
        String::from("Add Benjamin to Manager"),
        String::from("Add Terry to Manager"),
        String::from("Add Joe to Manager"),
        String::from("Add Christopher to Manager"),
    ];
    for input in inputs {
        match_cmd(input, depts);
    }
}

fn match_cmd(input: String, depts: &mut HashMap<String, Vec<String>>) {
    // 从索引为3处分隔字符串
    let (op, cmd) = input.trim().split_at(3);
    match op.to_uppercase().as_str() {
        "ADD" => {
            let sp: Vec<&str> = cmd.trim().split(" to ").collect();
            let emps = depts.entry(String::from(sp[1])).or_insert(Vec::new());
            emps.push(String::from(sp[0]));
            println!("添加员工成功:{:?}", depts);
        }
        "GET" => {
            match cmd.trim() {
                "all" => {
                    let mut emps = Vec::new();
                    for values in depts.values() {
                        for value in values {
                            emps.push(value);
                        }
                    }
                    emps.sort();
                    println!("全部员工:{:?}", emps);
                    // let emps = emps.concat();
                    // println!("全部员工:{:?}", emps);
                }
                _ => {
                    let dept = cmd.trim();
                    match depts.get(dept) {
                        Some(emps) => {
                            let mut emps: Vec<&String> = emps.iter().collect();
                            emps.sort();
                            println!("部门:{}下的员工:{:?}", dept, emps)
                        }
                        None => println!("部门不存在"),
                    }
                }
            }
        }
        _ => println!("不支持该操作:{}", op),
    }
}
