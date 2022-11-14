use std::borrow::BorrowMut;
use crate::board::SdkStd::{False, Poss, True};
use crate::constraints::Constraint;

// SudokuStandard
pub enum SdkStd {
    True,
    Poss,
    False,
}

pub struct Board<S>{
    data: Vec<S>,
    constraints: Vec<dyn Constraint<S>>,
    pub size: usize,
}

impl<S> Board<S> {
    pub fn getm(&mut self, x:usize, y:usize, z:usize) -> &mut S {
        (self.data[x*self.size*self.size+y*self.size+z]).borrow_mut()
    }

    pub fn get(&self, x:usize, y:usize, z:usize) -> &S {
        &(self.data[x*self.size*self.size+y*self.size+z])
    }
}

impl Board<SdkStd> {
    fn init(&mut self, size: usize) {
        self.size = size;
        self.data = vec![Poss; size*size*size];
    }

    fn set_trues(&mut self, x: usize, y:usize) {
        let mut can_fill = 0;
        let mut digit = -1;
        for z in 0..self.size {
            let r = self.get(x, y, z);
            if r == Poss {
                can_fill += 1;
                digit = z;
                if can_fill >= 2 {
                    break;
                }
            }
        }
        if can_fill == 1 {
            *(self.getm(x, y, digit)) = True;
        }
        else if can_fill == 0 {
            println!("Not possible");
            panic!();
        }
        can_fill = 0;
        for z in 0..self.size {
            let r = self.get(x, z, y);
            if r == Poss {
                can_fill += 1;
                digit = z;
                if can_fill >= 2 {
                    break;
                }
            }
        }
        if can_fill == 1 {
            *(self.getm(x,  digit, y)) = True;
        }
        else if can_fill == 0 {
            println!("Not possible");
            panic!();
        }
        can_fill = 0;
        for z in 0..self.size {
            let r = self.get(z, y, x);
            if r == Poss {
                can_fill += 1;
                digit = z;
                if can_fill >= 2 {
                    break;
                }
            }
        }
        if can_fill == 1 {
            *(self.getm(digit, y, x)) = True;
        }
        else if can_fill == 0 {
            println!("Not possible");
            panic!();
        }
    }

    fn solve(&mut self) {
        for x in 0..self.size {
            for y in 0..self.size {
                self.set_trues(x,y);
            }
        }
        for con in self.constraints {
            con.apply(self);
        }
    }
}