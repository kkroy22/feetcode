fn main() {
    use std::io::Write;
    let s = std::io::stdin();
    let w = std::io::stdout();
    let mut bufin = std::io::BufRead::lines(std::io::BufReader::new(s.lock()));
    let mut bufwr = std::io::BufWriter::new(w.lock());

    if let Some(Ok(input)) = bufin.next() {
        let size = input.len();
        let input = input.as_bytes();
        let mut ans = 0;
        let mut open = 0;
        let mut maxclose = i32::MAX;

        (0..size).for_each(|i| {
            if unsafe {
                input.get_unchecked(i) == &b'('
            }{
                open += 1;
            } else {
                open -= 1;
            }

            if maxclose > open {
                maxclose = open;
                ans = 0;
            }

            if maxclose == open {
                ans += 1;
            }
         
        });
        if std::cmp::PartialEq::ne(&open, &0) {
            writeln!(bufwr, "0").ok();
        } else {
            writeln!(bufwr, "{}", ans).ok();
        }
    }
}
