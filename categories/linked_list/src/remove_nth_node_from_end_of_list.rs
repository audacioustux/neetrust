use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        use std::iter::successors;
        let count = successors(head.as_ref(), |last| last.next.as_ref()).count();
        let mut head = head;

        let n = n as usize;
        if count == n {
            return head.unwrap().next;
        }

        let mut prev = &mut head;
        for _ in 0..(count - n - 1) {
            prev = &mut prev.as_mut().unwrap().next;
        }
        prev.as_mut().unwrap().next = prev.as_mut().unwrap().next.as_mut().unwrap().next.take();
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_to_list_node;

    #[test]
    fn test_remove_nth_from_end() {
        assert_eq!(
            Solution::remove_nth_from_end(text_to_list_node("1 > 2 > 3 > 4 > 5"), 2),
            text_to_list_node("1 > 2 > 3 > 5"),
        );
        assert_eq!(
            Solution::remove_nth_from_end(text_to_list_node("1 > 2 > 3 > 4 > 5"), 5),
            text_to_list_node("2 > 3 > 4 > 5"),
        );
    }
}
