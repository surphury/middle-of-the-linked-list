#[cfg(test)]
use crate::{middle_node, ListNode};

#[test]
fn test_1() {
    let node = Some(Box::new(ListNode::from(&vec![1, 2, 3, 4, 5])));
    let result = middle_node(node);

    assert_eq!(vec![3, 4, 5], ListNode::to_vector(&result));
}

#[test]
fn test_2() {
    let node = Some(Box::new(ListNode::from(&vec![1, 2, 3, 4, 5, 6])));
    let result = middle_node(node);

    assert_eq!(vec![4, 5, 6], ListNode::to_vector(&result));
}

#[test]
fn test_3() {
    let node = Some(Box::new(ListNode::from(&vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ])));
    let result = middle_node(node);

    assert_eq!(vec![6, 7, 8, 9, 10], ListNode::to_vector(&result));
}
#[test]
fn test_4() {
    let node = Some(Box::new(ListNode::from(&vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
    ])));
    let result = middle_node(node);

    assert_eq!(vec![6, 7, 8, 9, 10, 11], ListNode::to_vector(&result));
}
