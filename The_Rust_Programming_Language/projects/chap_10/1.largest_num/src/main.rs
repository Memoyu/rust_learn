fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // 结构体泛型
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];

    for &n in list.iter() {
        if largest < n {
            largest = n;
        }
    }

    largest
}

// 结构体使用泛型
struct Point<T, U> {
    x: T,
    y: U,
}

// 方法使用泛型
impl<T, U> Point<T, U>
where
    T: Copy,
{
    fn x(&self) -> T {
        self.x
    }
}
