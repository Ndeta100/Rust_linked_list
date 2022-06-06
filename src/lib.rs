struct Node {
    element: u32,
    next: Link,
}

struct LinkedList {
    head: Link,
}
enum Link {
    Empty,
    NonEmpty(Box<Node>),
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let list = Link::NonEmpty(Box::new(Node {
            element: 103,
            next: Link::Empty,
        }));
    }
}
