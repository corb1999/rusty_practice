// lib.rs file code below...

//lib.rs
#[test]
fn iterator_demo() {
    let v1 = vec![1, 2, 3]; 
    let mut v1_iter = v1.iter(); 

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

//lib.rs
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3]; 
    let v1_iter = v1.iter(); 
    let total: i32 = v1_iter.sum(); 

    assert_eq!(total, 6); 
}

//lib.rs
#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32, 
    style: String, 
}

fn shoes_in_sizes(arg_shoes: Vec<Shoe>, arg_shoe_size: u32) -> Vec<Shoe> {
    arg_shoes.into_iter().filter(|s| s.size == arg_shoe_size).collect()
}

//lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10, 
                style: String::from("sneaker"), 
            }, 
            Shoe {
                size: 13, 
                style: String::from("sandal"), 
            }, 
            Shoe {
                size: 10, 
                style: String::from("boot"), 
            }, 
        ]; 

        let in_my_size = shoes_in_sizes(shoes, 10); 

        assert_eq!(in_my_size, 
            vec![
            Shoe {
                size: 10, 
                style: String::from("sneaker"), 
            }, 
            Shoe {
                size: 10, 
                style: String::from("boot"), 
            }, 
        ]
        )
    }
}