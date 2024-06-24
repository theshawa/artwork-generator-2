#[derive(Debug, PartialEq, Eq)]
pub enum DnaTreeNodeColor {
    Red,
    Black,
}

#[derive(Debug)]
pub struct DnaTreeNode {
    pub value: u64,
    pub color: DnaTreeNodeColor,
    pub left: Option<Box<DnaTreeNode>>,
    pub right: Option<Box<DnaTreeNode>>,
}

impl DnaTreeNode {
    fn new(value: u64) -> Self {
        DnaTreeNode {
            value,
            color: DnaTreeNodeColor::Red,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct DnaTree {
    root: Option<Box<DnaTreeNode>>,
}

impl DnaTree {
    pub fn new() -> Self {
        DnaTree { root: None }
    }

    pub fn search(&self, value: u64) -> bool {
        self.search_node(&self.root, value)
    }

    fn search_node(&self, node: &Option<Box<DnaTreeNode>>, value: u64) -> bool {
        match node {
            Some(n) => {
                if n.value == value {
                    true
                } else if n.value > value {
                    self.search_node(&n.left, value)
                } else {
                    self.search_node(&n.right, value)
                }
            }
            None => false,
        }
    }

    pub fn insert(&mut self, value: u64) {
        let new_node = Box::new(DnaTreeNode::new(value));
        self.root = Self::insert_node(self.root.take(), new_node);
        if let Some(ref mut root_node) = self.root {
            root_node.color = DnaTreeNodeColor::Black;
        }
    }

    fn insert_node(
        node: Option<Box<DnaTreeNode>>,
        new_node: Box<DnaTreeNode>,
    ) -> Option<Box<DnaTreeNode>> {
        match node {
            Some(mut current) => {
                if new_node.value < current.value {
                    current.left = Self::insert_node(current.left.take(), new_node);
                } else if new_node.value > current.value {
                    current.right = Self::insert_node(current.right.take(), new_node);
                }
                Some(Self::balance(current))
            }
            None => Some(new_node),
        }
    }

    fn balance(mut node: Box<DnaTreeNode>) -> Box<DnaTreeNode> {
        if Self::is_red(&node.right) && !Self::is_red(&node.left) {
            node = Self::rotate_left(node);
        }
        if Self::is_red(&node.left) && Self::is_red(&node.left.as_ref().unwrap().left) {
            node = Self::rotate_right(node);
        }
        if Self::is_red(&node.left) && Self::is_red(&node.right) {
            Self::flip_colors(&mut node);
        }
        node
    }

    fn is_red(node: &Option<Box<DnaTreeNode>>) -> bool {
        match node {
            Some(n) => n.color == DnaTreeNodeColor::Red,
            None => false,
        }
    }

    fn rotate_left(mut node: Box<DnaTreeNode>) -> Box<DnaTreeNode> {
        let mut new_root = node.right.take().unwrap();
        node.right = new_root.left.take();
        new_root.left = Some(node);
        match new_root.left.as_ref().unwrap().color {
            DnaTreeNodeColor::Red => new_root.color = DnaTreeNodeColor::Red,
            DnaTreeNodeColor::Black => new_root.color = DnaTreeNodeColor::Black,
        }
        new_root.left.as_mut().unwrap().color = DnaTreeNodeColor::Red;
        new_root
    }

    fn rotate_right(mut node: Box<DnaTreeNode>) -> Box<DnaTreeNode> {
        let mut new_root = node.left.take().unwrap();
        node.left = new_root.right.take();
        new_root.right = Some(node);
        match new_root.right.as_ref().unwrap().color {
            DnaTreeNodeColor::Red => new_root.color = DnaTreeNodeColor::Red,
            DnaTreeNodeColor::Black => new_root.color = DnaTreeNodeColor::Black,
        }
        new_root.right.as_mut().unwrap().color = DnaTreeNodeColor::Red;
        new_root
    }

    fn flip_colors(node: &mut Box<DnaTreeNode>) {
        node.color = DnaTreeNodeColor::Red;
        if let Some(ref mut left_child) = node.left {
            left_child.color = DnaTreeNodeColor::Black;
        }
        if let Some(ref mut right_child) = node.right {
            right_child.color = DnaTreeNodeColor::Black;
        }
    }

    pub fn print_inorder(&self) {
        self.inorder_traversal(&self.root);
    }

    fn inorder_traversal(&self, node: &Option<Box<DnaTreeNode>>) {
        if let Some(n) = node {
            self.inorder_traversal(&n.left);
            println!("{} ({:?})", n.value, n.color);
            self.inorder_traversal(&n.right);
        }
    }
}
