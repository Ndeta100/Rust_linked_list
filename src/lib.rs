type Link = Option<Box<Node>>;
struct Node {
    element: u32,
    next: Link,
}

pub struct LinkedList {
    head: Link,
}
impl LinkedList {
    fn empty() -> LinkedList {
        LinkedList { head: None }
    }
    fn push(&mut self, element: u32) {
        match self.head {
            None => {
                self.head = Some(Box::new(Node {
                    element,
                    next: None,
                }))
            }
            Some(n) => {
                let new_head = Some(Box::new(Node {
                    element,
                    next: Some(n),
                }));
                self.head = new_head;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut list = LinkedList::empty();
        list.push(39);
    }
}
