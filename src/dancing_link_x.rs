use std::{collections::HashMap, fmt::Debug};

type NodeId = u32;

#[derive(Default, Clone, Debug)]
pub struct Node<T>
where
    T: Default + Clone,
{
    pub node: T,
    pub id: NodeId,
    pub left: NodeId,
    pub right: NodeId,
    pub up: NodeId,
    pub down: NodeId,
    pub line: usize,
    pub col: usize,
}

impl<T: Default + Clone> Node<T> {
    pub fn new(node: T, id: NodeId, line: usize, col: usize) -> Self {
        Node {
            node,
            id,
            left: id,
            right: id,
            up: id,
            down: id,
            line,
            col,
        }
    }
}

#[derive(Default, Clone)]
pub struct DLX<T>
where
    T: Default + Clone,
{
    pub nodes: HashMap<NodeId, Node<T>>,
    pub root: NodeId,
    pub id: NodeId,
    pub cols: Vec<NodeId>,
    pub lines: Vec<NodeId>,
}

impl<T: Default + Clone + Debug> DLX<T> {
    pub fn new(line_size: usize, col_size: usize) -> Self {
        let mut dlx = DLX {
            nodes: HashMap::new(),
            root: 0,
            id: 0,
            cols: vec![],
            lines: vec![],
        };
        let id = dlx.get_id();
        dlx.nodes.insert(0, Node::new(T::default(), id, 0, 0));
        for _ in 0..col_size {
            dlx.append_empty_col()
        }
        for _ in 0..line_size {
            dlx.append_empty_line()
        }
        dlx
    }

    pub fn get_id(&mut self) -> NodeId {
        let id = self.id;
        self.id += 1;
        id
    }

    fn alloc_node(&mut self, node: T, line: usize, col: usize) -> NodeId {
        let id = self.get_id();
        let node = Node::new(node, id, line, col);
        self.nodes.insert(id, node);
        id
    }

    pub fn add_node(&mut self, node: T, line: usize, col: usize) -> NodeId {
        let id = self.alloc_node(node, line, col);
        let line_head_id = self.get_line(line);
        let line_last_node_id = self.get_node(line_head_id).left;

        let col_head_id = self.get_line(col);
        let col_last_node_id = self.get_node(col_head_id).up;

        self.set_node(line_head_id, None, None, Some(id), None);
        self.set_node(line_last_node_id, None, None, None, Some(id));

        self.set_node(col_head_id, Some(id), None, None, None);
        self.set_node(col_last_node_id, None, Some(id), None, None);

        self.set_node(
            id,
            Some(col_last_node_id),
            Some(col_head_id),
            Some(line_last_node_id),
            Some(line_head_id),
        );
        id
    }

    pub fn append_empty_col(&mut self) {
        let root = self.root.clone();
        let col_head_id = self.alloc_node(T::default(), 0, self.cols.len());
        let last_col_id = self.get_node(root).left;
        self.set_node(last_col_id, None, None, None, Some(col_head_id));
        self.set_node(col_head_id, None, None, Some(last_col_id), Some(root));
        self.set_node(root, None, None, Some(col_head_id), None);

        self.cols.push(col_head_id);
    }

    pub fn append_empty_line(&mut self) {
        let root = self.root.clone();
        let line_head_id = self.alloc_node(T::default(), self.lines.len(), 0);

        let last_line_id = self.get_node(root).up;

        self.set_node(last_line_id, None, Some(line_head_id), None, None);
        self.set_node(line_head_id, Some(last_line_id), Some(root), None, None);
        self.set_node(root, Some(last_line_id), Some(root), None, None);

        self.lines.push(line_head_id);
    }

    pub fn get_node(&self, id: NodeId) -> &Node<T> {
        self.nodes.get(&id).unwrap()
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> &mut Node<T> {
        self.nodes.get_mut(&id).unwrap()
    }

    pub fn set_node(
        &mut self,
        id: NodeId,
        up: Option<NodeId>,
        down: Option<NodeId>,
        left: Option<NodeId>,
        right: Option<NodeId>,
    ) {
        let node = self.get_node_mut(id);
        if let Some(up) = up {
            node.up = up
        }

        if let Some(down) = down {
            node.down = down
        }

        if let Some(left) = left {
            node.left = left
        }

        if let Some(right) = right {
            node.right = right
        }
    }

    pub fn get_col(&self, col: usize) -> NodeId {
        *self.cols.get(col).unwrap()
    }

    pub fn get_line(&self, line: usize) -> NodeId {
        *self.lines.get(line).unwrap()
    }

    pub fn print(&self) {
        print!("root    <-> ");
        for col in &self.cols {
            let node = self.get_node(*col);
            print!(" col({:?}) <->", node.col);
        }
        println!();
        for i in 0..self.lines.len() {
            self.print_line(i);
        }
    }
    pub fn print_line(&self, line: usize) {
        print!("line({line}) <-> ");
        let head_node = self.get_node(self.get_line(line));
        let mut right_node_id = head_node.right;
        while right_node_id != head_node.id {
            let right_node = self.get_node(right_node_id);
            print!("node({}, {}) <-> ", right_node.line, right_node.col);
            right_node_id = right_node.right;
        }
        println!();
    }
}
