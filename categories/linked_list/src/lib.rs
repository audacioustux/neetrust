pub mod merge_two_sorted_lists;
pub mod reorder_list;
pub mod reverse_linked_list;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn text_to_list_node(text: &str) -> Option<Box<ListNode>> {
    text.split('>')
        .rev()
        .map(|val| val.trim().parse::<i32>().unwrap())
        .fold(None, |head, val| {
            let mut node = ListNode::new(val);
            node.next = head;
            Some(Box::new(node))
        })
}

#[test]
fn test_text_to_list_node() {
    assert_eq!(
        text_to_list_node("1 > 2 > 3 > 4 > 5"),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }))
    );
}
