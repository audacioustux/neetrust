use crate::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }
            carry = sum / 10;
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            tail = &mut tail.as_mut().unwrap().next;
        }

        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_to_list_node;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(
            Solution::add_two_numbers(
                text_to_list_node("2 > 4 > 3"),
                text_to_list_node("5 > 6 > 4")
            ),
            text_to_list_node("7 > 0 > 8"),
        );
        assert_eq!(
            Solution::add_two_numbers(text_to_list_node("0"), text_to_list_node("0")),
            text_to_list_node("0"),
        );
        assert_eq!(
            Solution::add_two_numbers(
                text_to_list_node("9 > 9 > 9 > 9 > 9 > 9 > 9"),
                text_to_list_node("9 > 9 > 9 > 9")
            ),
            text_to_list_node("8 > 9 > 9 > 9 > 0 > 0 > 0 > 1"),
        );
    }
}
