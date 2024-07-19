use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

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
    T: Default + Clone + Eq,
{
    pub nodes: HashMap<NodeId, Node<T>>,
    pub root: NodeId,
    pub id: NodeId,
    pub cols: Vec<NodeId>,
    // pub lines: Vec<NodeId>,
}

impl<T: Default + Clone + Debug + Eq> DLX<T> {
    pub fn new(col_size: usize) -> Self {
        let mut dlx = DLX {
            nodes: HashMap::new(),
            root: 0,
            id: 0,
            cols: vec![],
            // lines: vec![],
        };
        let id = dlx.get_id();
        dlx.nodes.insert(0, Node::new(T::default(), id, 0, 0));
        for _ in 0..col_size {
            dlx.append_col_head()
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

    pub fn add_node(
        &mut self,
        node: T,
        line: usize,
        col: usize,
        left_node_id: Option<NodeId>,
        right_node_id: Option<NodeId>,
    ) -> NodeId {
        let id = self.alloc_node(node, line, col);

        let col_head_id = self.get_col(col);

        let col_last_node_id = self.get_node(col_head_id).up;

        self.set_node(col_head_id, Some(id), None, None, None);
        self.set_node(col_last_node_id, None, Some(id), None, None);

        if let Some(left_node_id) = left_node_id {
            self.set_node(left_node_id, None, None, None, Some(id));
        }

        if let Some(right_node_id) = right_node_id {
            self.set_node(right_node_id, None, None, Some(id), None);
        }

        self.set_node(
            id,
            Some(col_last_node_id),
            Some(col_head_id),
            left_node_id,
            right_node_id,
        );
        id
    }

    pub fn append_col_head(&mut self) {
        let root = self.root.clone();
        let col_head_id = self.alloc_node(T::default(), 0, self.cols.len());
        let last_col_id = self.get_node(root).left;

        self.set_node(last_col_id, None, None, None, Some(col_head_id));
        self.set_node(col_head_id, None, None, Some(last_col_id), Some(root));
        self.set_node(root, None, None, Some(col_head_id), None);

        self.cols.push(col_head_id);
    }

    pub fn get_node(&self, id: NodeId) -> Node<T> {
        self.nodes.get(&id).unwrap().clone()
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

    pub fn col_len(&self, col: usize) -> u32 {
        let mut len = 0;
        let col_head_node_id = self.cols.get(col).unwrap().clone();
        let col_head_node = self.get_node(col_head_node_id);
        let mut current_node_id = col_head_node.down;
        while current_node_id != col_head_node_id {
            len += 1;
            current_node_id = self.get_node(current_node_id).down;
        }
        len
    }

    pub fn remove_node(&mut self, id: NodeId) {
        let node = self.get_node(id);
        self.set_node(node.down, Some(node.up), None, None, None);
        self.set_node(node.up, None, Some(node.down), None, None);
        self.set_node(node.left, None, None, None, Some(node.right));
        self.set_node(node.right, None, None, Some(node.left), None);
    }

    pub fn recover_node(&mut self, id: NodeId) {
        let node = self.get_node(id);
        self.set_node(node.down, Some(id), None, None, None);
        self.set_node(node.up, None, Some(id), None, None);
        self.set_node(node.left, None, None, None, Some(id));
        self.set_node(node.right, None, None, Some(id), None);
    }

    pub fn solve(&mut self, res: &mut Vec<usize>) -> bool {
        // (1) 查找头结点Head元素右边相邻的结点，如果Head的right域指向自己，则算法结束，表明找到了合法的方案。否则，继续执行步骤(2)。
        // (2) 找长度最短的列 c，查找列c的所有节点，选择其中一个节点，记作r，将结点r所在的行加入答案中。如果节点c所在列没有节点，此时说明当前方案不可行，需要回溯到上一阶段重新选择结点。否则，执行步骤(3)。
        // (3) 遍历r所在行的全部节点(不包括r)，对于每一个节点s，执行步骤(4)、(5)。
        // (4) 对于节点s，遍历s所在列的每一个节点t(不包括s)，删除t所在行的全部节点，包括结点t本身。
        // (5) 删除节点s和s所在列的头结点。
        // (6) 对节点r执行步骤(4)、(5)。
        // (7) 继续从步骤(1)开始执行，直到算法结束。

        // (1) 查找头结点Head元素右边相邻的结点，如果Head的right域指向自己，则算法结束，表明找到了合法的方案。否则，继续执行步骤(2)。
        if self.get_node(self.root).right == self.root {
            return true;
        }

        // (2) 找长度最短的列 c，查找列c的所有节点，选择其中一个节点，记作r，将结点r所在的行加入答案中。如果节点c所在列没有节点，此时说明当前方案不可行，需要回溯到上一阶段重新选择结点。否则，执行步骤(3)。
        let mut min_len = 9 * 9 * 9;
        let mut min_len_col = 9 * 9 * 4;
        let mut col_head_id = self.get_node(self.root).right;
        while col_head_id != self.root {
            // println!("{:?}", col_head_id);
            let col_head_node = self.get_node(col_head_id);
            let col_len = self.col_len(col_head_node.col);
            if col_len < min_len {
                min_len = col_len;
                min_len_col = col_head_node.col;
            }
            col_head_id = col_head_node.right;
        }
        if min_len == 9 * 9 * 9 {
            return true;
        }
        // 最短一列的头节点
        let min_len_col_head_node = self.get_node(self.cols[min_len_col]);
        let mut r_id = min_len_col_head_node.down;

        // 如果列c没有节点，此时说明当前方案不可行，需要回溯到上一阶段重新选择结点。否则，执行步骤(3)。
        if r_id == min_len_col_head_node.id {
            return false;
        }

        // 查找列c的所有节点
        while r_id != min_len_col_head_node.id {
            let r = self.get_node(r_id);
            let mut remove_node = HashSet::new();
            // 选择其中一个节点，记作r，将结点r所在的行加入答案中。
            res.push(r.line);

            // (3) 遍历r所在行的全部节点(不包括r)，对于每一个节点s，执行步骤(4)、(5)。
            let mut s_id = r.right;
            while s_id != r.id {
                // (4) 对于节点s，遍历s所在列的每一个节点t(不包括s)，删除t所在行的全部节点，包括结点t本身。
                let s = self.get_node(s_id);
                let mut t_id = s.down;
                while t_id != s_id {
                    let t: Node<T> = self.get_node(t_id);
                    if t_id == self.cols[t.col] {
                        t_id = t.down;
                        continue;
                    }
                    remove_node.insert(t_id);
                    let t = self.get_node(t_id);
                    let mut node_in_t_line = t.right;
                    while node_in_t_line != t_id {
                        remove_node.insert(node_in_t_line);
                        node_in_t_line = self.get_node(node_in_t_line).right;
                    }
                    t_id = self.get_node(t_id).down;
                }

                // (5) 删除节点s和s所在列的头结点。
                remove_node.insert(s_id);
                remove_node.insert(self.cols[s.col]);

                s_id = self.get_node(s_id).right;
            }

            for node in &remove_node {
                self.remove_node(*node);
            }

            // (6) 对节点r执行步骤(4)、(5)。 删除 r 所在列
            {
                let s_id = r.id;
                let s = self.get_node(s_id);
                let mut t_id = s.down;
                while t_id != s_id {
                    let t: Node<T> = self.get_node(t_id);
                    if t_id == self.cols[t.col] {
                        t_id = t.down;
                        continue;
                    }
                    self.remove_node(t_id);
                    let t = self.get_node(t_id);
                    let mut node_in_t_line = t.right;
                    while node_in_t_line != t_id {
                        self.remove_node(node_in_t_line);
                        node_in_t_line = self.get_node(node_in_t_line).right;
                    }
                    t_id = self.get_node(t_id).down;
                }

                // (5) 删除节点s和s所在列的头结点。直接删除，不需要恢复
                self.remove_node(s_id);
                self.remove_node(self.cols[s.col]);
            }

            if self.solve(res) {
                return true;
            } else {
                // 需要回溯到上一阶段重新选择结点
                res.pop();
                for node in remove_node {
                    self.recover_node(node);
                }
                r_id = self.get_node(r_id).down;
            }
        }

        false
    }
}
