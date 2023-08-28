use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_to_list_node;

    #[test]
    fn test_reverse_list() {
        assert_eq!(
            Solution::reverse_list(text_to_list_node("1 > 2")),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(1)))
            }))
        );
    }
}
