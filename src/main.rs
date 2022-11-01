mod test;

/* Definition for singly-linked list. */

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn to_vector(list: &Option<Box<ListNode>>) -> Vec<i32> {
        let mut pointer = list;

        let mut vector = Vec::new();

        loop {
            if let Some(item) = pointer {
                vector.push(item.val);
                pointer = &item.next;
            } else {
                break;
            }
        }

        vector
    }

    pub fn from(collection: &Vec<i32>) -> ListNode {
        let collection = collection.iter().rev().collect::<Vec<&i32>>();

        let mut list = ListNode {
            val: *collection[0],
            next: None,
        };

        for i in 1..collection.len() {
            let current = collection[i];

            let new_node = ListNode {
                val: *current,
                next: Some(Box::new(list)),
            };

            list = new_node;
        }

        list
    }
}

fn main() {}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut pointer, mut pointer_2) = (&head, &head);

    while let Some(ListNode {
        next: Some(node), ..
    }) = pointer_2.as_deref()
    {
        pointer = &pointer.as_ref().unwrap().next;
        pointer_2 = &node.next;
    }

    pointer.clone()
}
