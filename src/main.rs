fn main() {
    use std::io::Write;
    let s = std::io::stdin();
    let w = std::io::stdout();
    let mut bufin = std::io::BufRead::lines(std::io::BufReader::new(s.lock()));
    let mut bufwr = std::io::BufWriter::new(w.lock());

    if let Some(Ok(_n)) = bufin.next() {
        if let Some(Ok(ele_list)) = bufin.next() {
            let mut ll = std::collections::LinkedList::new();
            ele_list
                .trim()
                .split_whitespace()
                .for_each(|e| ll.push_back(e.parse::<i32>().unwrap()));
            let mut dis = false;
            let mut rev: Vec<i32> = Vec::new();
            for val in ll {
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
            }

            (0..rev.len()).for_each(|_| {
                write!(bufwr, "{:?} ", rev.pop().unwrap()).ok();
            });
        }
    }
}
