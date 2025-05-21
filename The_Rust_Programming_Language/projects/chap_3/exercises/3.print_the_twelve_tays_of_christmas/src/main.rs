fn main() {
    // 打印圣诞颂歌The Twelve Days of Christmas 的歌词
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for (day_ind, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas, My true love sent to me:", day);

        // 取得本次gift下标数组，并反转
        for gi in (0..=day_ind).rev() {
            // 最后一个gift需要使用and连接
            if day_ind != 0 && gi == 0 {
                print!("And ");
            }
            println!("{}", gifts[gi]);
        }

        print!("\n");
    }
}
