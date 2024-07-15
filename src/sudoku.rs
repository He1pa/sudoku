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
        let dlx = self.dlx();
        dlx.print()
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

    pub fn get_line(&self, l: usize) -> Vec<Node> {
        self.res.get(l).unwrap().clone()
    }

    pub fn get_col(&self, c: usize) -> Vec<Node> {
        let mut col = vec![];
        for i in 0..9 {
            col.push(self.get(i, c))
        }
        col
    }

    pub fn get_squa(&self, i: usize) -> Vec<Node> {
        let l = i - i % 3;
        let c = 3 * (i % 3);
        let mut squa = vec![];
        for i in 0..3 {
            for j in 0..3 {
                squa.push(self.get(l + i, c + j))
            }
        }
        squa
    }

    pub fn get(&self, l: usize, c: usize) -> Node {
        self.res.get(l).unwrap().get(c).unwrap().clone()
    }

    pub fn get_mut(&mut self, l: usize, c: usize) -> &mut Node {
        self.res.get_mut(l).unwrap().get_mut(c).unwrap()
    }

    pub fn set(&mut self, l: usize, c: usize, n: u8) {
        self.res.get_mut(l).unwrap().get_mut(c).unwrap().res = n;
    }

    pub fn check(&self) -> bool {
        for i in 0..9 {
            let line = self.get_line(i);
            let mut l_sum = 0;
            line.iter().for_each(|node| l_sum += node.res);

            let col = self.get_col(i);
            let mut c_sum = 0;
            col.iter().for_each(|node| c_sum += node.res);

            let squa = self.get_squa(i);
            let mut squa_sum = 0;
            squa.iter().for_each(|node| squa_sum += node.res);

            if l_sum != 45 || c_sum != 45 || squa_sum != 45 {
                return false;
            }
        }
        true
    }

    pub fn dlx(&self) -> DLX<u8> {
        let mut dlx_len = 0;

        for l in 0..9 {
            for c in 0..9 {
                if self.get(l, c).res == 0 {
                    dlx_len += 9;
                } else {
                    dlx_len += 1;
                }
            }
        }

        let mut dlx: DLX<u8> = DLX::new(dlx_len, 9 * 9 * 4);
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
                        dlx.add_node(1, line, first_dlx_col);
                        dlx.add_node(1, line, second_dlx_col);
                        dlx.add_node(1, line, third_dlx_col);
                        dlx.add_node(1, line, forth_dlx_col);
                        line += 1;
                    }
                } else {
                    let n = node.res as usize;
                    let first_dlx_col = l * 9 + c; // [0, 81) position in sudoku
                    let second_dlx_col = 80 + l * 9 + n; // [81, 162) which num in each line
                    let third_dlx_col = 161 + c * 9 + n; // [162, 243) which num in each col
                    let forth_dlx_col = 242 + (l - l % 3 + c / 3) * 9 + n; // [243, 324) which num in each squa
                    dlx.add_node(1, line, first_dlx_col);
                    dlx.add_node(1, line, second_dlx_col);
                    dlx.add_node(1, line, third_dlx_col);
                    dlx.add_node(1, line, forth_dlx_col);
                    line += 1;
                }

            }
        }
        dlx
    }
}
