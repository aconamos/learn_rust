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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let (larger, smaller) = setup_smaller_larger();

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn smaller_can_hold_larger() {
        let (larger, smaller) = setup_smaller_larger();

        assert!(!smaller.can_hold(&larger))
    }

    #[test]
    fn larger_can_hold_larger() {
        let (larger, _) = setup_smaller_larger();

        assert!(!larger.can_hold(&larger));
    }

    #[test]
    fn smaller_can_hold_smaller() {
        let (_, smaller) = setup_smaller_larger();

        assert!(!smaller.can_hold(&smaller));
    }

    fn setup_smaller_larger() -> (Rectangle, Rectangle) {
        (
            Rectangle {
                width: 8,
                height: 7,
            },
            Rectangle {
                width: 7,
                height: 6,
            },
        )
    }
}
