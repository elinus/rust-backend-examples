#![allow(dead_code)]
#[allow(unused_variables)]

fn foo<T>(item: T) {}

struct List {
    list: Option<Box<List>>,
}

impl List {
    fn new() -> Self {
        List {
            list: Some(Box::new(List { list: None })),
        }
    }
}

struct ListNode {
    value: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> ListNode {
        ListNode {
            value: val,
            next: Some(Box::new(ListNode { value: 0, next: None })),
        }
    }
}

fn main() {
    println!("Box!");
    let x: i32 = 0;
    foo(x);
    foo(x);

    let my_box = Box::new(1);
    let my_integer = *my_box;
    println!("{:?}", my_box);
    println!("{:?}", my_integer);
    foo(my_box.clone());
    foo(my_box);

    println!("\n\nExperiment linked-list!");
    let mut list = ListNode::new(1);
    list.next = ListNode::new(2);
    list.next.next = ListNode::new(3);

    let mut curr = list; 
    while curr.is_some() {
        print!("{}", curr.value);
        curr = curr.next;
    }
}
