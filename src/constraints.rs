use crate::board::{Board, SdkStd};
use crate::board::SdkStd::{False, Poss, True};

// A constraint can only remove a possibility/pencil mark
pub trait Constraint<S> {
    // Remove all illegal pencil marks
    fn apply(board: &mut Board<S>);

    // Get all tiles that this rule affects from one tile
    // Effect of "guessing" on that tile
    fn affects(board: &Board<S>, x: usize, y:usize) -> Vec<(usize, usize, usize)>;
}

pub struct RowConstraint;
impl Constraint<SdkStd> for RowConstraint {
    fn apply(board: &mut Board<SdkStd>) {
        for x in 0..board.size {
            for y in 0..board.size {
                for z in 0..board.size {
                    if board.get(x,y,z) == True {
                        for y_ in 0..board.size {
                            *(board.getm(x,y_,z)) = match board.get(x,y_,z) {
                                True => True,
                                _ => False
                            };
                        }
                    }
                }
            }
        }
    }

    fn affects(board: &Board<SdkStd>, x: usize, y: usize) -> Vec<(usize, usize, usize)> {
        let mut ret = Vec::new();
        for z in 0..board.size {
            if board.get(x,y,z) != Poss {
                continue;
            }
            for y_ in 0..board.size {
                if y==y_ {
                    continue;
                }
                if board.get(x, y_, z) == Poss {
                    ret.push((x,y_,z));
                }
            }
        }
        ret
    }
}

pub struct ColConstraint;
impl Constraint<SdkStd> for ColConstraint {
    fn apply(board: &mut Board<SdkStd>) {
        for x in 0..board.size {
            for y in 0..board.size {
                for z in 0..board.size {
                    if board.get(x,y,z) == True {
                        for x_ in 0..board.size {
                            *(board.getm(x_,y,z)) = match board.get(x_,y,z) {
                                True => True,
                                _ => False
                            };
                        }
                    }
                }
            }
        }
    }

    fn affects(board: &Board<SdkStd>, x: usize, y: usize) -> Vec<(usize, usize, usize)> {
        let mut ret = Vec::new();
        for z in 0..board.size {
            if board.get(x,y,z) != Poss {
                continue;
            }
            for x_ in 0..board.size {
                if x==x_ {
                    continue;
                }
                if board.get(x_, y, z) == Poss {
                    ret.push((x_,y,z));
                }
            }
        }
        ret
    }
}