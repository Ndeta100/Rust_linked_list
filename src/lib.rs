use std::os::macos;

pub type Link = Option<Box<Node>>;
pub struct Node {
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
    pub fn push(&mut self, element: u32) {
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });
        self.head = Some(new_head);
        //Below solution let to the above
        // match self.head {
        //     None => {
        //         self.head = Some(Box::new(Node {
        //             element,
        //             next: None,
        //         }))
        //     }
        //     Some(n) => {
        //         let new_head = Some(Box::new(Node {
        //             element,
        //             next: Some(n),
        //         }));
        //         self.head = new_head;
        //     }
        // }
    }
    pub fn pop(&mut self) -> Option<u32> {
        let old_head = self.head.take();
        match old_head {
            Some(n) => {
                self.head = n.next;
                Some(n.element)
            }
            None => None,
        }
    }
    fn peek(&mut self) -> Option<&u32> {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut list = LinkedList::empty();
        list.push(39);
        list.push(67);
        list.pop();
    }
}
