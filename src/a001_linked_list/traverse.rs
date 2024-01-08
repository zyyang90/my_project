use crate::a001_linked_list::ListNode;

pub fn traverse(head: &ListNode) {
    traverse01(head);

    // traverse02(head);
}

/// 遍历： cur 从 head 开始，到最后一个 node 结束
fn traverse01(head: &ListNode) {
    let mut current = head;
    while let Some(node) = current.next.as_ref() {
        print!("{} ", current.data);
        current = node;
    }
    println!("{}", current.data);
}

/// 遍历，cur 从 head 开始，到 None 结束
fn traverse02(head: &ListNode) {
    let mut current = Some(Box::new(head.to_owned()));

    while let Some(node) = current {
        print!("{} ", node.data);
        current = node.next;
    }
    println!();
}

#[cfg(test)]
mod tests {
    use crate::a001_linked_list::mock;
    use super::*;

    #[test]
    fn test_traverse01() {
        let head = mock();
        traverse01(&head);
    }

    #[test]
    fn test_traverse02() {
        let head = mock();
        traverse02(&head);
    }
}
