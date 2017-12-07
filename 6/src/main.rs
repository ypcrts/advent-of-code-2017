// use std::ops::Range;
use std::fmt;
use std::string::String;
use std::cell::RefCell;
use std::rc::Rc;
use std::hash::{self, Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
type BanksVec = std::vec::Vec<u8>;

#[derive(Debug,PartialEq,Eq)]
struct Banks {
    cell: Rc<RefCell<BanksVec>>,
    size: usize
}
impl Banks {
    pub fn new(a: &[u8]) -> Banks {
        let size = a.len();
        let mut b: BanksVec = Vec::with_capacity(size);
        for x in a.iter() {
            b.push(*x);
        }
        let s = Banks {
            cell: Rc::new(RefCell::new(b)),
            size: size
        };
        s
    }

    fn realloc(&mut self) -> u64 {
        let mut i: u64 = 0;
        let mut hist = HashMap::new();
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hist.insert(hasher.finish(), i);
        println!("{}", self);
        loop {
            i += 1;
            self.realloc_single_cycle();
            println!("{}", self);
            let mut hasher = DefaultHasher::new();
            self.hash(&mut hasher);
            let ret = hist.insert(hasher.finish(), i);
            if ret.is_some() {
                return i
            }
            match ret {
                None    => continue,
                Some(x) => return x
            }
        }
    }

    fn realloc_single_cycle(&mut self) {
        let largest_index = self.largest_block_index();
        assert_ne!(self.cell.borrow()[largest_index], 0);

        let mut v = self.cell.borrow_mut();
        let mut b = v[largest_index];
        v[largest_index] = 0;
        let mut i = largest_index;
        while b > 0 {
            i = ( i + 1 ) % self.size;
            b -=  1;
            let this = v.get_mut(i).unwrap();
            *this += 1;
        }

        return;
    }

    /// Return the lowest index of the highest value
    fn largest_block_index(&self) -> usize {
        self.cell.borrow()
            .iter()
            .enumerate()
            .rev()
            .max_by(|&(_,a), &(_,b)| a.cmp(b))
            .unwrap() .0
    }

    

}
impl Hash for Banks {
    #[inline]
    /// make it use the vector hash, so i can type less
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.cell.borrow().hash(state)
    }
}

impl fmt::Display for Banks {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let c = self.cell.borrow();
        let mut s = String::new();
        s.push_str("[ ");
        for i in c.iter()  {
            s.push_str(&format!("{:2}", i));
            s.push_str(" ");
        }
        s.push_str("]");
        try!(fmt.write_str(&s));
        return Ok(());
    }
}


#[cfg(not(test))]
fn main() {
    part2();
}

fn part1() {
    let mut banks: Banks = Banks::new(&[5, 1, 10, 0, 1, 7, 13, 14, 3, 12, 8, 10, 7, 12, 0, 6]);
    let ret = banks.realloc();
    println!("{}", ret);
}
fn part2() {
    let mut banks = Banks::new(&[ 1, 1, 14, 13, 12, 11, 10,  9,  8,  7,  7,  5,  5,  3,  3,  0 ]);
    let ret = banks.realloc();
    println!("{}", ret);
}


#[cfg(test)]
mod test {
    use super::Banks;

    #[test]
    fn test_size() {
        let banks = Banks::new(&[0, 2, 7, 0]);
        let n = banks.size;
        assert_eq!(n, 4);
    }
    #[test]
    fn test_size_larger() {
        let banks = Banks::new(&[0, 2, 7, 0, 1, 1, 1, 1, 1, 1, 1]);
        let n = banks.size;
        assert_eq!(n, 11);
    }

    #[test]
    fn test_largest_block_index() {
        let banks = Banks::new(&[9, 2, 12, 0, 12, 12, 12, 11, 0]);
        assert_eq!(banks.largest_block_index(), 2);
    }

    #[test]
    fn test_realloc_single_cycle() {
        let mut banks = Banks::new(&[0, 2, 7, 0]);
        banks.realloc_single_cycle();
        assert_eq!(banks, Banks::new(&[2, 4, 1, 2]));;
        banks.realloc_single_cycle();
        assert_eq!(banks, Banks::new(&[3, 1, 2, 3]));;
        banks.realloc_single_cycle();
        assert_eq!(banks, Banks::new(&[0, 2, 3, 4]));;
        banks.realloc_single_cycle();
        assert_eq!(banks, Banks::new(&[1, 3, 4, 1]));;
        banks.realloc_single_cycle();
        assert_eq!(banks, Banks::new(&[2, 4, 1, 2]));;
    }

    #[test]
    fn test_realloc() {
        let mut banks = Banks::new(&[0, 2, 7, 0]);
        let ret = banks.realloc();
        assert_eq!(ret, 5);

    }
}
