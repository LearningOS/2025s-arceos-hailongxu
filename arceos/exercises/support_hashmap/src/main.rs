#![no_std]
#![no_main]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

#[cfg(feature = "axstd")]
use std::collections::HashMap;

#[cfg_attr(feature = "axstd", no_mangle)]
#[cfg(feature = "axstd")]
fn main() {
    println!("Running memory tests...");
    test_hashmap();
    println!("Memory tests run OK!");
}

#[cfg(feature = "axstd")]
fn test_hashmap() {
    const N: u32 = 50_000;
    let mut m = HashMap::new();
    for value in 0..N {
        let key = format!("key_{value}");
        m.insert(key, value);
    }
    for (k, v) in m.iter() {
        if let Some(k) = k.strip_prefix("key_") {
            assert_eq!(k.parse::<u32>().unwrap(), *v);
        }
    }
    println!("test_hashmap() OK!");
}
