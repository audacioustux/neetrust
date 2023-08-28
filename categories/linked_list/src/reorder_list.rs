use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut middle = Self::get_middle_node(head);
        middle = Self::reverse_list(middle);
        let mut tail = head;
        while let Some(mut node) = middle {
            middle = node.next.take();
            node.next = tail.as_mut().unwrap().next.take();
            tail.as_mut().unwrap().next = Some(node);
            tail = &mut tail.as_mut().unwrap().next.as_mut().unwrap().next;
        }
    }
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut prev = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
    fn get_middle_node(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut fast, mut slow) = (&head.clone(), head);
        while fast.is_some() {
            fast = &(fast.as_ref().unwrap().next);
            if fast.is_some() {
                fast = &fast.as_ref().unwrap().next;
                slow = &mut (slow.as_mut().unwrap().next);
            }
        }
        slow.as_mut().unwrap().next.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_to_list_node;

    #[test]
    fn test_reorder_list() {
        let mut head = text_to_list_node("1 > 2 > 3 > 4 > 5");
        Solution::reorder_list(&mut head);
        assert_eq!(head, text_to_list_node("1 > 5 > 2 > 4 > 3"));
    }

    #[test]
    fn test_reverse_list() {
        assert_eq!(
            Solution::reverse_list(text_to_list_node("4 > 5")),
            text_to_list_node("5 > 4"),
        );
    }

    #[test]
    fn test_get_middle_node() {
        let mut head = text_to_list_node("1 > 2 > 3 > 4 > 5");
        assert_eq!(
            Solution::get_middle_node(&mut head),
            text_to_list_node("4 > 5"),
        );
    }
}
