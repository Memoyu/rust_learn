use crate::{array_hash_map::ArrayHashMap, chaining_hash_map::ChainingHashMap};

pub mod array_hash_map;
pub mod chaining_hash_map;
pub mod open_addressing_hash_map;

fn main() {
    println!("Array HashMap");
    let mut map = ArrayHashMap::new();

    map.put(12836, "小哈".to_string());
    map.put(15937, "小啰".to_string());
    map.put(16750, "小算".to_string());
    map.put(13279, "小法".to_string());
    map.put(10583, "小鸭".to_string());

    assert_eq!(Some("小法".to_string()), map.get(13279));
    assert_eq!(Some("小啰".to_string()), map.get(15937));

    map.remove(13279);
    map.remove(10583);

    assert_eq!(vec![12836, 15937, 16750], map.key_set());
    assert_eq!(
        vec!["小哈".to_string(), "小啰".to_string(), "小算".to_string()],
        map.value_set()
    );

    map.print();

    println!("链式地址-HashMap");
    let mut map = ChainingHashMap::new(4);

    map.put(12836, "小哈".to_string());
    map.put(15937, "小啰".to_string());
    map.put(16750, "小算".to_string());
    map.put(13279, "小法".to_string());
    map.put(10583, "小鸭".to_string());

    assert_eq!(Some("小法".to_string()), map.get(13279));
    assert_eq!(Some("小啰".to_string()), map.get(15937));

    map.remove(13279);
    map.remove(10583);

    map.print();

    println!("开放寻址-线性探测-HashMap");
    let mut map = ChainingHashMap::new(4);

    map.put(12836, "小哈".to_string());
    map.put(15937, "小啰".to_string());
    map.put(16750, "小算".to_string());
    map.put(13279, "小法".to_string());
    map.put(10583, "小鸭".to_string());

    assert_eq!(Some("小法".to_string()), map.get(13279));
    assert_eq!(Some("小啰".to_string()), map.get(15937));

    map.remove(13279);
    map.remove(10583);

    map.print();
}
