#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub struct LinkedList {
    pub head: Option<Box<ListNode>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            head: None,
        }
    }

    pub fn insert_at_head(&mut self, val: i32) {
        let new_node = Box::new(ListNode {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
}
