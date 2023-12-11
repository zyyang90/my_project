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