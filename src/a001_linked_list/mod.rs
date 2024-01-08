pub mod traverse;
mod update;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub data: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub(crate) fn new(data: i32) -> Self {
        ListNode {
            data,
            next: None,
        }
    }
}

pub fn mock() -> ListNode {
    let mut node1 = ListNode::new(1);
    let mut node2 = ListNode::new(3);
    let mut node3 = ListNode::new(5);
    let mut node4 = ListNode::new(7);
    let node5 = ListNode::new(9);

    node4.next = Some(Box::new(node5));
    node3.next = Some(Box::new(node4));
    node2.next = Some(Box::new(node3));
    node1.next = Some(Box::new(node2));

    node1
}