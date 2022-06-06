struct Node {
    element: u32,
    next: Link,
}

struct LinkedList {
    head: Link,
}
type Link = Option<Box<Node>>;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let list = Link::Some(Box::new(Node {
            element: 103,
            next: Link::None,
        }));
    }
}
