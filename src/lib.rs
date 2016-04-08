use std::vec::Vec;


pub struct SegmentTree {
    array: Vec<i32>,
    f: fn(i32, i32) -> i32,
}

impl SegmentTree {
    pub fn new(arr : &[i32], f: fn(i32, i32) -> i32) -> SegmentTree {
        let mut tree_size = 1;
        while tree_size < arr.len() { tree_size *= 2};
        let mut tree = Vec::with_capacity(tree_size * 2);
        for _ in 0..tree_size {
            tree.push(0);
        }
        for x in arr.iter() {
            tree.push(*x);
        }
        // add zeros to make 2^n sized array
        // alternatively check if node has children
        for _ in (arr.len() +tree_size) .. (tree_size*2) {
            tree.push(0);
        }
        for i in (1..tree_size).rev() {
            tree[i] = f(tree[2*i], tree[2*i +1]);
        }
        SegmentTree {array: tree, f: f}
    }

    pub fn range(&self, a: usize, b: usize) -> Result<i32, &str> {
        let n =  self.array.len() / 2; // size of the input array, index of first leaf
        let (mut a, mut b) = (a + n, b + n);
        if b < a || b >= self.array.len() { // a < n is guaranteed by type usize
            return Err("invalid range");
        }

        let mut uninitialised = true;
        let mut acc = 0;
        while a <= b {
            if a%2 == 1 {
                if uninitialised {
                    acc = *self.array.get(a).unwrap();
                    uninitialised = false;
                } else {
                    acc = (self.f)(acc, *self.array.get(a).unwrap());
                }
            }
            if b%2 == 0 {
                if uninitialised {
                    acc = *self.array.get(b).unwrap();
                    uninitialised = false;
                } else {
                    acc = (self.f)(acc, *self.array.get(b).unwrap());
                }
            }
            a = (a+1) / 2;
            b = (b-1) / 2;
        }
        Ok(acc)
    }

    pub fn print(&self) {
        let mut magic = 2;
        for (i, val) in self.array.iter().enumerate() {
            if i == 0 {continue};
            if i == magic {
                magic *= 2;
                println!("");
            }
            let padding = (self.array.len()*2)/magic;
            print!("{1:^0$}", padding,  val);
        }
        println!("");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Add;
    use std::ops::Mul;

    #[test]
    fn entire_arr_with_add() {
        let vector : &[i32] = &vec!(1,2,3,4);
        let tree = SegmentTree::new(vector, i32::add);
        assert_eq!(tree.range(0, 3), Ok(10i32));

    }

    #[test]
    fn entire_arr_with_mul() {
        let vector : &[i32] = &vec!(5,2,3,4);
        let tree = SegmentTree::new(vector, i32::mul);
        assert_eq!(tree.range(0,3), Ok(120i32));
    }

    #[test]
    fn it_works_with_add() {
        let vector : &[i32] = &vec!(7,2,11,-2,8,-1,2);
        let tree = SegmentTree::new(vector, i32::add);
        assert_eq!(tree.range(3,6), Ok(7i32));
    }

    #[test]
    fn it_works_with_mul() {
        let vector : &[i32] = &vec!(-1,2,3,4);
        let tree = SegmentTree::new(vector, i32::mul);
        assert_eq!(tree.range(1,2), Ok(6i32));
    }

    #[test]
    fn err_if_too_big_range() {
        let vector : &[i32] = &vec!(7,2,5,9,3,1);
        let tree = SegmentTree::new(vector, i32::add);
        assert_eq!(tree.range(3, 8), Err("invalid range"));
    }

    #[test]
    fn err_if_inverted_range() {
        let vector : &[i32] = &vec!(7,2,5,9,3,1);
        let tree = SegmentTree::new(vector, i32::add);
        assert_eq!(tree.range(4,1), Err("invalid range"));
    }
}
