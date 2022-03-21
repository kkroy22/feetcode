pub struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut storage = vec![0; n as usize];
        unsafe {
            *storage.get_unchecked_mut(0) = 1;
        }
        let mut two = 0;
        let mut three = 0;
        let mut five = 0;
        (1..n as usize).for_each(|index| {
            unsafe {
                let two_m = 2 * storage.get_unchecked(two);
                let three_m = 3 * storage.get_unchecked(three);
                let five_m = 5 * storage.get_unchecked(five);

                *storage.get_unchecked_mut(index) = std::cmp::min(std::cmp::min(two_m, three_m), five_m);
                
                if std::cmp::PartialEq::eq(storage.get_unchecked(index), &two_m) {
                    two += 1;
                }
                if std::cmp::PartialEq::eq(storage.get_unchecked(index), &three_m) {
                    three += 1;
                }
                if std::cmp::PartialEq::eq(storage.get_unchecked(index), &five_m) {
                    five += 1;
                }
                
            }
        });
        storage.pop().unwrap()
    }
}