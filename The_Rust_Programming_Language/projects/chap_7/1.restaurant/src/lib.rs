mod back_of_house;
mod front_of_house;

use crate::front_of_house::hosting;

use std::collections::HashMap;
// 使用as关键字处理同名引入
// use std::fmt::Result;
// use std::io::Result as IoResult;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // 黑麦面包作为夏季早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // 修改面包类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 以下代码无法编译通过，无法设置私有seasonal_fruit字段的值
    // meal.seasonal_fruit = String::from("Blueberries");

    // 使用枚举，枚举所有变体(内部项)的公共性随枚举的公共属性决定，与结构体不同，结构体需要每个属性进行标注pub
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // 使用use导入作用域，简化调用
    hosting::add_to_waitlist();

    // 导入标准库HashMap
    let mut map = HashMap::new();
    map.insert(1, 2);
}
