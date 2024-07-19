use crate::dancing_link_x::DLX;

#[derive(Default, Clone)]
pub struct Node {
    res: u8,
}

impl Node {
    pub fn new(n: u8) -> Self {
        Self {
            res: n,
            ..Default::default()
        }
    }
}

#[derive(Default, Clone)]
pub struct Sudoku {
    pub res: Vec<Vec<Node>>,
    pub dlx: DLX<u8>,
}

impl Sudoku {
    pub fn init(input: [[u8; 9]; 9]) -> Self {
        let mut sudoku = Sudoku {
            res: vec![],
            dlx: DLX::default(),
        };
        for _ in 0..9 {
            sudoku.res.push(vec![]);
        }

        for (i, v) in input.iter().enumerate() {
            let line = sudoku.res.get_mut(i).unwrap();
            for n in v.iter() {
                line.push(Node::new(*n));
            }
        }
        sudoku
    }

    pub fn resolve(&mut self) {
        let mut dlx = self.dlx();
        let mut res = vec![];
        dlx.solve(&mut res);
        res.sort();
        let mut line = 0;
        for (i, r) in res.iter().enumerate() {
            let l = i / 9;
            let c = i % 9;
            let node = self.get_mut(l, c);
            if node.res == 0 {
                node.res = (r - line + 1) as u8;
                line += 9
            } else {
                line += 1
            }
        }
        println!("{:?}", res);
    }

    pub fn get(&self, l: usize, c: usize) -> Node {
        self.res.get(l).unwrap().get(c).unwrap().clone()
    }

    pub fn get_mut(&mut self, l: usize, c: usize) -> &mut Node {
        self.res.get_mut(l).unwrap().get_mut(c).unwrap()
    }

    pub fn dlx(&self) -> DLX<u8> {
        let mut dlx: DLX<u8> = DLX::new(9 * 9 * 4);
        let mut line = 0;
        for l in 0..9 {
            for c in 0..9 {
                let node = self.get(l, c);
                if node.res == 0 {
                    for n in 1..10 {
                        let first_dlx_col = l * 9 + c; // [0, 81) position in sudoku
                        let second_dlx_col = 80 + l * 9 + n; // [81, 162) which num in each line
                        let third_dlx_col = 161 + c * 9 + n; // [162, 243) which num in each col
                        let forth_dlx_col = 242 + (l - l % 3 + c / 3) * 9 + n; // [243, 324) which num in each squa

                        let node1 = dlx.add_node(1, line, first_dlx_col, None, None);
                        let node2 = dlx.add_node(1, line, second_dlx_col, Some(node1), Some(node1));
                        let node3 = dlx.add_node(1, line, third_dlx_col, Some(node2), Some(node1));
                        let _ = dlx.add_node(1, line, forth_dlx_col, Some(node3), Some(node1));
                        line += 1;
                    }
                } else {
                    let n = node.res as usize;
                    let first_dlx_col = l * 9 + c; // [0, 81) position in sudoku
                    let second_dlx_col = 80 + l * 9 + n; // [81, 162) which num in each line
                    let third_dlx_col = 161 + c * 9 + n; // [162, 243) which num in each col
                    let forth_dlx_col = 242 + (l - l % 3 + c / 3) * 9 + n; // [243, 324) which num in each squa

                    let node1 = dlx.add_node(1, line, first_dlx_col, None, None);
                    let node2 = dlx.add_node(1, line, second_dlx_col, Some(node1), Some(node1));
                    let node3 = dlx.add_node(1, line, third_dlx_col, Some(node2), Some(node1));
                    let _ = dlx.add_node(1, line, forth_dlx_col, Some(node3), Some(node1));
                    line += 1;
                }
            }
        }
        dlx
    }

    pub fn output(&self) {
        for (i, l) in self.res.iter().enumerate() {
            if i % 3 == 0 {
                println!("------------------");
            }

            for (j, c) in l.iter().enumerate() {
                if j % 3 == 0 {
                    print!("|");
                }
                print!("{:?}", c.res);
            }
            println!("|");
        }
        println!("------------------");
    }
}
