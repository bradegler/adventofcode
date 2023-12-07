pub mod tree {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Clone)]
    pub struct BinaryTreeNode {
        val: i32,
        left: Option<BinaryTreeNodeRef>,
        right: Option<BinaryTreeNodeRef>,
    }

    type BinaryTreeNodeRef = Rc<RefCell<BinaryTreeNode>>;

    impl BinaryTreeNode {
        pub fn tree_sum(root: BinaryTreeNodeRef) -> i32 {
            let mut sum = 0i32;
            // We'll use a `vec` as a
            // stack LIFO data structure.
            // Start by adding the root node
            // to the stack.
            let mut stack = vec![root];

            while !stack.is_empty() {
                // current points to top most
                // item in the stack
                let current: Rc<RefCell<BinaryTreeNode>> = stack.pop().unwrap();
                sum += current.borrow().val;

                // if there is a right node,
                // then push it on top of the stack
                if let Some(right) = &current.borrow().right {
                    stack.push(right.to_owned());
                };
                // if there is a left node,
                // then push it on top of the stack
                if let Some(left) = &current.borrow().left {
                    stack.push(left.to_owned());
                };
            }
            sum
        }
    }
}
