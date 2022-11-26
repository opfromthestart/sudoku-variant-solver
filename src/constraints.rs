use std::hash::Hash;
use crate::board::LogicVal::{False, Poss, True};
use crate::board::{Board, Puzzle, LogicVal, Enumerable, Tuple3D};

/// A constraint can only remove a possibility/pencil mark
pub(crate) trait Constraint<T : Eq + Hash + Enumerable, S:Board<T>>{
    /// Remove all illegal pencil marks
    fn apply(&self, board: &mut S) -> bool;

    /// Get all tiles that this rule affects from one tile
    /// Effect of "guessing" True on that tile
    fn affects(&self, board: &S, v: &T)
        -> Vec<T>;
}

pub struct RowUniqueConstraint;

impl Clone for RowUniqueConstraint {
    fn clone(&self) -> Self {
        RowUniqueConstraint
    }

    fn clone_from(&mut self, source: &Self) {}
}

impl<S : Board<Tuple3D<SIZE>>, const SIZE:usize> Constraint<Tuple3D<SIZE>, S> for RowUniqueConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for v in &Tuple3D::<SIZE>::positions() {
            if board.get(v) == True {
                for y_ in 0..SIZE {
                    let to_rem = Tuple3D::from((v.pos.0, y_, v.pos.2));
                    *(board.getm(&to_rem)) = match board.get(&to_rem) {
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
        did
    }

    fn affects(
        &self,
        board: &S,
        v: &Tuple3D<SIZE>
    ) -> Vec<Tuple3D<SIZE>> {
        let mut ret = Vec::new();
        let (x,y,z) = v.pos;
        if board.get(v) != Poss {
            return ret;
        }
        for y_ in 0..SIZE {
            if y == y_ {
                continue;
            }
            let ret_tup = Tuple3D::from((x,y_,z));
            if board.get(&ret_tup) == Poss {
                ret.push(ret_tup);
            }
        }
        ret
    }
}

pub struct RowExistConstraint;

impl Clone for RowExistConstraint {
    fn clone(&self) -> Self {
        Self
    }

    fn clone_from(&mut self, source: &Self) {}
}

impl<S: Board<Tuple3D<SIZE>>, const SIZE: usize> Constraint<Tuple3D<SIZE>, S> for RowExistConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for x in 0..SIZE {
            for z in 0..SIZE {
                let mut poss_count = 0;
                for y in 0..SIZE {
                    if board.get(&Tuple3D::from((x,y,z))) != False {
                        poss_count += 1;
                    }
                }
                if poss_count == 1 {
                    //eprintln!("Row found unique");
                    for y_ in 0..SIZE {
                        let pos = Tuple3D::from((x,y_,z));
                        *(board.getm(&pos)) = match board.get(&pos) {
                            False => {False}
                            True => {True}
                            Poss => {did=true; True}
                        };
                    }
                }
            }
        }
        did
    }

    fn affects(&self, board: &S, v: &Tuple3D<SIZE>) -> Vec<Tuple3D<SIZE>> {
        vec![]
    }
}

pub struct ColUniqueConstraint;

impl Clone for ColUniqueConstraint {
    fn clone(&self) -> Self {
        Self
    }

    fn clone_from(&mut self, source: &Self) where Self: {
    }
}

impl<S : Board<Tuple3D<SIZE>>, const SIZE:usize> Constraint<Tuple3D<SIZE>, S> for ColUniqueConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for v in &Tuple3D::positions() {
            let (x,y,z) = v.pos;
            if board.get(v) == True {
                for x_ in 0..SIZE {
                    let to_rem = Tuple3D::from((x_, y, z));
                    *(board.getm(&to_rem)) = match board.get(&to_rem) {
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
        did
    }

    fn affects(
        &self,
        board: &S,
        v: &Tuple3D<SIZE>
    ) -> Vec<Tuple3D<SIZE>> {
        let (x,y,z) = v.pos;
        let mut ret = Vec::new();
        if board.get(v) != Poss {
            return ret;
        }
        for x_ in 0..SIZE {
            if x == x_ {
                continue;
            }
            let ret_pos = Tuple3D::from((x_, y, z));
            if board.get(&ret_pos) == Poss {
                ret.push(ret_pos);
            }
        }
        ret
    }
}

pub struct ColExistConstraint;

impl Clone for ColExistConstraint {
    fn clone(&self) -> Self {
        Self
    }

    fn clone_from(&mut self, source: &Self) {}
}

impl<S: Board<Tuple3D<SIZE>>, const SIZE: usize> Constraint<Tuple3D<SIZE>, S> for ColExistConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for y in 0..SIZE {
            for z in 0..SIZE {
                let mut poss_count = 0;
                for x in 0..SIZE {
                    if board.get(&Tuple3D::from((x,y,z))) != False {
                        poss_count += 1;
                    }
                }
                if poss_count == 1 {
                    for x_ in 0..SIZE {
                        let pos = Tuple3D::from((x_,y,z));
                        *(board.getm(&pos)) = match board.get(&pos) {
                            False => {False}
                            True => {True}
                            Poss => {did=true; True}
                        };
                    }
                }
            }
        }
        did
    }

    fn affects(&self, board: &S, v: &Tuple3D<SIZE>) -> Vec<Tuple3D<SIZE>> {
        vec![]
    }
}

pub struct DigitUniqueConstraint;

impl Clone for DigitUniqueConstraint {
    fn clone(&self) -> Self {
        DigitUniqueConstraint
    }

    fn clone_from(&mut self, source: &Self) where Self: {

    }
}

impl<S : Board<Tuple3D<SIZE>>, const SIZE:usize> Constraint<Tuple3D<SIZE>, S> for DigitUniqueConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for v in Tuple3D::<SIZE>::positions() {
            let (x,y,z) = v.pos;
            if board.get(&v) == True {
                for z_ in 0..SIZE {
                    let ret_pos = Tuple3D::from((x, y, z_));
                    *(board.getm(&ret_pos)) = match board.get(&ret_pos) {
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
        did
    }

    fn affects(
        &self,
        board: &S,
        v: &Tuple3D<SIZE>
    ) -> Vec<Tuple3D<SIZE>> {
        let (x,y,z) = v.pos;
        let mut ret = Vec::new();
        if board.get(v) != Poss {
            return ret;
        }
        for z_ in 0..SIZE {
            if z == z_ {
                continue;
            }
            let ret_pos = Tuple3D::from((x, y, z_));
            if board.get(&ret_pos) == Poss {
                ret.push(ret_pos);
            }
        }
        ret
    }
}

pub struct DigitExistConstraint;

impl Clone for DigitExistConstraint {
    fn clone(&self) -> Self {
        Self
    }

    fn clone_from(&mut self, source: &Self) {}
}

impl<S: Board<Tuple3D<SIZE>>, const SIZE: usize> Constraint<Tuple3D<SIZE>, S> for DigitExistConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for x in 0..SIZE {
            for y in 0..SIZE {
                let mut poss_count = 0;
                for z in 0..SIZE {
                    if board.get(&Tuple3D::from((x,y,z))) != False {
                        poss_count += 1;
                    }
                }
                if poss_count == 1 {
                    for z_ in 0..SIZE {
                        let pos = Tuple3D::from((x,y,z_));
                        *(board.getm(&pos)) = match board.get(&pos) {
                            False => {False}
                            True => {True}
                            Poss => {did=true; True}
                        };
                    }
                }
            }
        }
        did
    }

    fn affects(&self, board: &S, v: &Tuple3D<SIZE>) -> Vec<Tuple3D<SIZE>> {
        vec![]
    }
}

pub struct CellConstraint {
    pub(crate) cells: Vec<(usize, usize)>,
}

impl Clone for CellConstraint {
    fn clone(&self) -> Self {
        CellConstraint{ cells : self.cells.clone()}
    }

    fn clone_from(&mut self, source: &Self) where Self: {
        self.cells = source.cells.clone();
    }
}

impl<S : Board<Tuple3D<SIZE>>, const SIZE:usize> Constraint<Tuple3D<SIZE>, S> for CellConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for (x, y) in &self.cells {
            for z in 0..SIZE {
                if board.get(&Tuple3D::from((*x, *y, z))) == True {
                    for (x_, y_) in &self.cells {
                        *(board.getm(&Tuple3D::from((*x_, *y_, z)))) = match board.get(&Tuple3D::from((*x_, *y_, z))) {
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
        board: &S,
        v: &Tuple3D<SIZE>
    ) -> Vec<Tuple3D<SIZE>> {
        let (x,y,z) = v.pos;
        let mut ret = Vec::new();
        if board.get(v) != Poss || !self.cells.contains(&(x, y)) {
            return ret;
        }
        for (x_, y_) in &self.cells {
            if *x_ == x && *y_ == y {
                continue;
            }
            let ret_pos = Tuple3D::from((*x_, *y_, z));
            if board.get(&ret_pos) == Poss {
                ret.push(ret_pos);
            }
        }
        ret
    }
}

pub struct GivenConstraint {
    pub(crate) pos: (usize, usize, usize),
}

impl Clone for GivenConstraint {
    fn clone(&self) -> Self {
        GivenConstraint{pos: self.pos}
    }

    fn clone_from(&mut self, source: &Self) where Self: {
        self.pos = source.pos;
    }
}

impl<S : Board<Tuple3D<SIZE>>, const SIZE:usize> Constraint<Tuple3D<SIZE>, S> for GivenConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let (x, y, z) = self.pos;
        let ret_pos = Tuple3D::from((x, y, z));
        let did = board.get(&ret_pos) == Poss;
        *(board.getm(&ret_pos)) = True;
        did
    }

    fn affects(
        &self,
        _board: &S,
        _v: &Tuple3D<SIZE>
    ) -> Vec<Tuple3D<SIZE>> {
        vec![]
    }
}

pub struct LessThanConstraint {
    pub(crate) lpos: (usize, usize),
    pub(crate) hpos: (usize, usize),
}

impl Clone for LessThanConstraint {
    fn clone(&self) -> Self {
        LessThanConstraint{lpos: self.lpos, hpos: self.hpos}
    }

    fn clone_from(&mut self, source: &Self) where Self: {
        self.lpos = source.lpos;
        self.hpos = source.hpos;
    }
}

impl<S : Board<Tuple3D<SIZE>>, const SIZE:usize> Constraint<Tuple3D<SIZE>, S> for LessThanConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let (xl, yl) = self.lpos;
        let (xh, yh) = self.hpos;
        let mut did = false;
        for zl in 0..SIZE {
            if board.get(&Tuple3D::from((xl, yl, zl))) == Poss {
                let mut to_rem = true;
                for zh in (zl + 1)..SIZE {
                    if board.get(&Tuple3D::from((xh, yh, zh))) != False {
                        to_rem = false;
                        break;
                    }
                }
                if to_rem {
                    did = true;
                    *(board.getm(&Tuple3D::from((xl, yl, zl)))) = False;
                }
            }
            if board.get(&Tuple3D::from((xh, yh, zl))) == Poss {
                let mut to_rem = true;
                for zh in 0..zl {
                    if board.get(&Tuple3D::from((xl, yl, zh))) != False {
                        to_rem = false;
                        break;
                    }
                }
                if to_rem {
                    did = true;
                    *(board.getm(&Tuple3D::from((xh, yh, zl)))) = False;
                }
            }
        }
        did
    }

    fn affects(
        &self,
        board: &S,
        v: &Tuple3D<SIZE>
    ) -> Vec<Tuple3D<SIZE>> {
        let (xl, yl) = self.lpos;
        let (xh, yh) = self.hpos;
        let mut ret = vec![];
        let (x,y,z) = v.pos;
        if board.get(v) != Poss || !((x, y) == self.lpos || (x, y) == self.hpos) {
            return ret;
        }
        if (x, y) == self.lpos {
            for zh in 0..z {
                let ret_pos = Tuple3D::from((xh, yh, zh));
                match board.get(&ret_pos) {
                    Poss => ret.push(ret_pos),
                    _ => {}
                }
            }
        } else {
            for zl in (z + 1)..SIZE {
                let ret_pos = Tuple3D::from((xl, yl, zl));
                match board.get(&ret_pos) {
                    Poss => ret.push(ret_pos),
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
