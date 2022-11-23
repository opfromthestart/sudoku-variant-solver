use crate::board::SdkStd::{False, Poss, True};
use crate::board::{Board, Puzzle, SdkStd};

// A constraint can only remove a possibility/pencil mark
pub trait Constraint<S> {
    // Remove all illegal pencil marks
    fn apply(&self, board: &mut Board<S>) -> bool;

    // Get all tiles that this rule affects from one tile
    // Effect of "guessing" on that tile
    fn affects(&self, board: &Board<S>, x: usize, y: usize, z: usize)
        -> Vec<(usize, usize, usize)>;
}

pub struct RowConstraint;
impl Constraint<SdkStd> for RowConstraint {
    fn apply(&self, board: &mut Board<SdkStd>) -> bool {
        let mut did = false;
        for x in 0..board.size {
            for y in 0..board.size {
                for z in 0..board.size {
                    if board.get(x, y, z) == True {
                        for y_ in 0..board.size {
                            *(board.getm(x, y_, z)) = match board.get(x, y_, z) {
                                True => True,
                                False => False,
                                Poss => {
                                    did = true;
                                    False
                                }
                            };
                        }
                    }
                }
            }
        }
        did
    }

    fn affects(
        &self,
        board: &Board<SdkStd>,
        x: usize,
        y: usize,
        z: usize,
    ) -> Vec<(usize, usize, usize)> {
        let mut ret = Vec::new();
        if board.get(x, y, z) != Poss {
            return ret;
        }
        for y_ in 0..board.size {
            if y == y_ {
                continue;
            }
            if board.get(x, y_, z) == Poss {
                ret.push((x, y_, z));
            }
        }
        ret
    }
}

pub struct ColConstraint;
impl Constraint<SdkStd> for ColConstraint {
    fn apply(&self, board: &mut Board<SdkStd>) -> bool {
        let mut did = false;
        for x in 0..board.size {
            for y in 0..board.size {
                for z in 0..board.size {
                    if board.get(x, y, z) == True {
                        for x_ in 0..board.size {
                            *(board.getm(x_, y, z)) = match board.get(x_, y, z) {
                                True => True,
                                False => False,
                                Poss => {
                                    did = true;
                                    False
                                }
                            };
                        }
                    }
                }
            }
        }
        did
    }

    fn affects(
        &self,
        board: &Board<SdkStd>,
        x: usize,
        y: usize,
        z: usize,
    ) -> Vec<(usize, usize, usize)> {
        let mut ret = Vec::new();
        if board.get(x, y, z) != Poss {
            return ret;
        }
        for x_ in 0..board.size {
            if x == x_ {
                continue;
            }
            if board.get(x_, y, z) == Poss {
                ret.push((x_, y, z));
            }
        }
        ret
    }
}

pub struct DigitConstraint;
impl Constraint<SdkStd> for DigitConstraint {
    fn apply(&self, board: &mut Board<SdkStd>) -> bool {
        let mut did = false;
        for x in 0..board.size {
            for y in 0..board.size {
                for z in 0..board.size {
                    if board.get(x, y, z) == True {
                        for z_ in 0..board.size {
                            *(board.getm(x, y, z_)) = match board.get(x, y, z_) {
                                True => True,
                                False => False,
                                Poss => {
                                    did = true;
                                    False
                                }
                            };
                        }
                    }
                }
            }
        }
        did
    }

    fn affects(
        &self,
        board: &Board<SdkStd>,
        x: usize,
        y: usize,
        z: usize,
    ) -> Vec<(usize, usize, usize)> {
        let mut ret = Vec::new();
        if board.get(x, y, z) != Poss {
            return ret;
        }
        for z_ in 0..board.size {
            if z == z_ {
                continue;
            }
            if board.get(x, y, z_) == Poss {
                ret.push((x, y, z_));
            }
        }
        ret
    }
}

pub struct CellConstraint {
    pub(crate) cells: Vec<(usize, usize)>,
}
impl Constraint<SdkStd> for CellConstraint {
    fn apply(&self, board: &mut Board<SdkStd>) -> bool {
        let mut did = false;
        for (x, y) in &self.cells {
            for z in 0..board.size {
                if board.get(*x, *y, z) == True {
                    for (x_, y_) in &self.cells {
                        *(board.getm(*x_, *y_, z)) = match board.get(*x_, *y_, z) {
                            True => True,
                            False => False,
                            Poss => {
                                did = true;
                                False
                            }
                        };
                    }
                }
            }
        }
        did
    }

    fn affects(
        &self,
        board: &Board<SdkStd>,
        x: usize,
        y: usize,
        z: usize,
    ) -> Vec<(usize, usize, usize)> {
        let mut ret = Vec::new();
        if board.get(x, y, z) != Poss || !self.cells.contains(&(x, y)) {
            return ret;
        }
        for (x_, y_) in &self.cells {
            if *x_ == x && *y_ == y {
                continue;
            }
            if board.get(*x_, *y_, z) == Poss {
                ret.push((*x_, *y_, z));
            }
        }
        ret
    }
}

pub struct GivenConstraint {
    pub(crate) pos: (usize, usize, usize),
}
impl Constraint<SdkStd> for GivenConstraint {
    fn apply(&self, board: &mut Board<SdkStd>) -> bool {
        let (x, y, z) = self.pos;
        let did = board.get(x, y, z) == Poss;
        *(board.getm(x, y, z)) = True;
        did
    }

    fn affects(
        &self,
        _board: &Board<SdkStd>,
        _x: usize,
        _y: usize,
        _z: usize,
    ) -> Vec<(usize, usize, usize)> {
        vec![]
    }
}

pub struct LessThanConstraint {
    pub(crate) lpos: (usize, usize),
    pub(crate) hpos: (usize, usize),
}
impl Constraint<SdkStd> for LessThanConstraint {
    fn apply(&self, board: &mut Board<SdkStd>) -> bool {
        let (xl, yl) = self.lpos;
        let (xh, yh) = self.hpos;
        let mut did = false;
        for zl in 0..board.size {
            if board.get(xl, yl, zl) == Poss {
                let mut to_rem = true;
                for zh in (zl + 1)..board.size {
                    if board.get(xh, yh, zh) != False {
                        to_rem = false;
                        break;
                    }
                }
                if to_rem {
                    did = true;
                    *(board.getm(xl, yl, zl)) = False;
                }
            }
            if board.get(xh, yh, zl) == Poss {
                let mut to_rem = true;
                for zh in 0..zl {
                    if board.get(xl, yl, zh) != False {
                        to_rem = false;
                        break;
                    }
                }
                if to_rem {
                    did = true;
                    *(board.getm(xh, yh, zl)) = False;
                }
            }
        }
        did
    }

    fn affects(
        &self,
        board: &Board<SdkStd>,
        x: usize,
        y: usize,
        z: usize,
    ) -> Vec<(usize, usize, usize)> {
        let (xl, yl) = self.lpos;
        let (xh, yh) = self.hpos;
        let mut ret = vec![];
        if board.get(x, y, z) != Poss || !((x, y) == self.lpos || (x, y) == self.hpos) {
            return ret;
        }
        if (x, y) == self.lpos {
            for zh in 0..z {
                match board.get(xh, yh, zh) {
                    Poss => ret.push((xh, yh, zh)),
                    _ => {}
                }
            }
        } else {
            for zl in (z + 1)..board.size {
                match board.get(xl, yl, zl) {
                    Poss => ret.push((xl, yl, zl)),
                    _ => {}
                }
            }
        }
        ret
    }
}

pub fn thermo_constraint(cells: Vec<(usize, usize)>) -> Vec<LessThanConstraint> {
    let mut ret = vec![];
    for i in 0..(cells.len() - 1) {
        ret.push(LessThanConstraint {
            lpos: cells[i],
            hpos: cells[i + 1],
        });
    }
    ret
}
