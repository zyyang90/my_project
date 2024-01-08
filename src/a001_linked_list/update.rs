use crate::a001_linked_list::ListNode;

pub fn update(head: &mut ListNode, position: u16, new_data: i32) {
    // update01(head, position, new_data);
    update02(head, position, new_data);
}

fn update02(head: &mut ListNode, position: u16, new_data: i32) {
    let mut current = Some(Box::new(head.to_owned()));
    let mut pos = 0;

    while let Some(mut node) = current {
        if pos == position {
            node.data = new_data;
            return;
        }
        pos += 1;
        current = node.next;
    }
}

fn update01(head: &mut ListNode, position: u16, new_data: i32) {
    let mut cur = head;
    let mut pos = 0;

    while let Some(node) = cur.next.as_mut() {
        if pos == position {
            cur.data = new_data;
        }

        pos += 1;
        cur = node;
    }

    if pos == position {
        cur.data = new_data;
    }
}

#[cfg(test)]
mod tests {
    use crate::a001_linked_list::mock;
    use crate::a001_linked_list::traverse::traverse;
    use super::*;

    #[test]
    fn add_in_middle() {
        // 1,3,5,7,9 -> 1,3,99,7,9
        let mut list = mock();

        update(&mut list, 2, 99);

        traverse(&list);
    }

    #[test]
    fn add_at_head() {
        let mut list = mock();

        update(&mut list, 0, 99);

        traverse(&list);
    }

    #[test]
    fn add_at_tail() {
        let mut list = mock();

        update(&mut list, 4, 99);

        traverse(&list);
    }

    #[test]
    fn add_out_of_bound() {
        let mut list = mock();

        update(&mut list, 99, 99);

        traverse(&list);
    }
}