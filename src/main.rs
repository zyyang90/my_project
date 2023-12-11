use std::thread::current;
use crate::a001_linkedlist::ListNode;

mod a001_linkedlist;

fn main() {
    let mut node1 = ListNode::new(1);
    let mut node2 = ListNode::new(3);
    let mut node3 = ListNode::new(5);
    let mut node4 = ListNode::new(7);
    let node5 = ListNode::new(9);

    node4.next = Some(Box::new(node5));
    node3.next = Some(Box::new(node4));
    node2.next = Some(Box::new(node3));
    node1.next = Some(Box::new(node2));

    let mut head = node1;

    // 遍历: 1, 3, 5, 7, 9
    traverse(&head);
    // 修改: 1, 3, 99, 7, 9
    update(&mut head, 2, 99);
    traverse(&head);
    // 99, 3, 99, 7, 9
    update(&mut head, 0, 99);
    traverse(&head);
    // 99, 3, 99, 7, 9
    update(&mut head, 99, 99);
    traverse(&head);
    // 99, 3, 99, 7, 99
    update(&mut head, 4, 99);
    traverse(&head);

    // // 增加: 99, 3, 99, 100, 7, 99
    // let node = ListNode::new(100);
    // add(&mut head, 2, node);
    // // 100, 99, 3, 99, 100, 7, 99
    // let node = ListNode::new(100);
    // add(&mut head, 0, node);
    // // 100, 99, 3, 99, 100, 7, 99, 100
    // let node = ListNode::new(100);
    // add(&mut head, 7, node);

    // 删除
}

fn add(head: &ListNode, position: u16, node: ListNode) {
    // let mut current = Some(Box::new(head.to_owned()));
    // while let Some(node) = current {
    //     print!("{} ", node.data);
    //
    //     current = node.next;
    // }
}

fn update(head: &ListNode, position: u16, new_data: i32) {
    let mut current = Some(Box::new(head.to_owned()));
    let mut pos = 0;

    while let Some(mut node) = current {
        if pos == position {
            node.data = new_data;
        }

        pos += 1;
        current = node.next;
    }
}

fn traverse(head: &ListNode) {
    let mut current = Some(Box::new(head.to_owned()));

    while let Some(node) = current {
        print!("{} ", node.data);
        current = node.next;
    }
    println!();
}