// this is all for practicing and learning rust

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus 2 doesnt equal 4"))
        }
        // assert_eq!(2 + 2, 4);
    }

/*     #[test]
    fn test_to_fail() {
        panic!("Intentionally failed test"); 
    } */

    #[test]
    fn large_can_hold_small() {
        let larger = Rectangle {
            width: 8, 
            height: 7, 
        }; 
        let smaller = Rectangle {
            width: 4, 
            height: 4, 
        }; 
        assert!(larger.can_hold(&smaller), 
                "custom test message here"); 
    }

    #[test] 
    fn small_cant_hold_large() {
        let larger = Rectangle {
            width: 8, 
            height: 7, 
        };
        let smaller = Rectangle {
            width: 5, 
            height: 1, 
        }; 
        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    fn we_can_add() {
        assert_eq!(4, add_dos(2)); 
    }
}

pub mod camus {
    pub fn do_nothing_at_all() {
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32, 
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_dos(a: i32) -> i32 {
    a + 2
}