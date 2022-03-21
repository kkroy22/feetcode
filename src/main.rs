use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

macro_rules! cast {
    ($x:expr, $t:ident) => {
        $x.unwrap().trim().parse::<$t>().unwrap()
    };

    ($x:expr) => {
        $x.unwrap().unwrap()
    };
}

fn main() {
    let s = stdin();
    let w = stdout();
    let mut bufin = BufReader::new(s.lock()).lines();
    let mut bufwr = BufWriter::new(w.lock());

    let mut t = cast!(bufin.next().unwrap(), usize);

    while t > 0 {
        let mut c = cast!(bufin.next().unwrap(), usize);
        let mut a = 0;
        let mut b = 0;
        let mut i = 0;
        while c > 0 {
            let z = 2usize.pow(i);
            if c == 1 {
                a += z;
            } else if c % 2 == 0 {
                a += z;
                b += z;
            } else {
                b += z;
            }
            c /= 2;
            i += 1;
        }
        writeln!(bufwr, "{}", a * b).ok();
        t -= 1;
    }
}