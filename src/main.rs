#[derive(Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut new_node = None;
        for &v in vec.iter().rev() {
            let mut node = ListNode::new(v);
            node.next = new_node;
            new_node = Some(Box::new(node));
        }
        new_node
    }
}

// 24 18 2 3 5 7 9 12 6
fn main() {
    use std::io::Write;
    let s = std::io::stdin();
    let w = std::io::stdout();
    let mut bufin = std::io::BufRead::lines(std::io::BufReader::new(s.lock()));
    let mut bufwr = std::io::BufWriter::new(w.lock());

    if let Some(Ok(n)) = bufin.next() {
        let mut n = n.parse::<usize>().unwrap();
        if let Some(Ok(ele_list)) = bufin.next() {
            let ele_list = ele_list
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let root = ListNode::to_list(ele_list);

            if let Some(root) = root {
                let mut dis = false;
                let mut rev: Vec<i32> = Vec::new();
                let mut next = root;

                while n > 0 {
                    let val = next.clone().val;
                    if val % 2 != 0 {
                        if !dis {
                            (0..rev.len()).for_each(|_| {
                                write!(bufwr, "{:?} ", rev.pop().unwrap()).ok();
                            });
                        }
                        write!(bufwr, "{:?} ", val).ok();
                        dis = true;
                    } else {
                        rev.push(val);
                        dis = false;
                    }

                    let current = next.clone().next;
                    if let Some(c) = current {
                        next = c;
                    }
                    n -= 1;
                }
                (0..rev.len()).for_each(|_| {
                    write!(bufwr, "{:?} ", rev.pop().unwrap()).ok();
                });
            }
        }
    }
}
