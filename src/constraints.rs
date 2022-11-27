use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use crate::board::LogicVal::{False, Poss, True};
use crate::board::{Board, Puzzle, LogicVal, Enumerable, Tuple3D, PosOnOff, TFBoard};

/// A constraint can only remove a possibility/pencil mark
pub(crate) trait Constraint<T : Eq + Hash + Enumerable, S:Board<T>> : Debug{
    /// Remove all illegal pencil marks
    fn apply(&self, board: &mut S) -> bool;

    /// Get all tiles that this rule affects from one tile
    /// Effect of "guessing" True on that tile
    fn affects(&self, board: &S, v: &T)
        -> Vec<T>;
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

impl<S : Board<PosOnOff<SIZE>>, const SIZE:usize> Constraint<PosOnOff<SIZE>, S> for DigitUniqueConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for v in PosOnOff::<SIZE>::positions() {
            let (x,y,z) = v.pos;
            if board.get(&v) == True {
                for z_ in [true, false] {
                    let ret_pos = PosOnOff::from((x, y, z_));
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
        v: &PosOnOff<SIZE>
    ) -> Vec<PosOnOff<SIZE>> {
        let (x,y,z) = v.pos;
        let mut ret = Vec::new();
        if board.get(v) != Poss {
            return ret;
        }
        for z_ in [true, false] {
            if z == z_ {
                continue;
            }
            let ret_pos = PosOnOff::from((x, y, z_));
            if board.get(&ret_pos) == Poss {
                ret.push(ret_pos);
            }
        }
        ret
    }
}

#[derive(Debug)]
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

impl<S: Board<PosOnOff<SIZE>>, const SIZE: usize> Constraint<PosOnOff<SIZE>, S> for DigitExistConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for x in 0..SIZE {
            for y in 0..SIZE {
                let mut poss_count = 0;
                for z in [true, false] {
                    if board.get(&PosOnOff::from((x,y,z))) != False {
                        poss_count += 1;
                    }
                }
                if poss_count == 1 {
                    for z_ in [true, false] {
                        let pos = PosOnOff::from((x,y,z_));
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

    fn affects(&self, board: &S, v: &PosOnOff<SIZE>) -> Vec<PosOnOff<SIZE>> {
        vec![]
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct GivenConstraint<T : Clone> {
    pub(crate) pos: T,
}

impl<T : Clone> Clone for GivenConstraint<T> {
    fn clone(&self) -> Self {
        GivenConstraint{pos: self.pos.clone()}
    }

    fn clone_from(&mut self, source: &Self) where Self: {
        self.pos = source.pos.clone();
    }
}

impl<T: Eq + Clone + Hash + Enumerable + Debug, S : Board<T>> Constraint<T, S> for GivenConstraint<T> {
    fn apply(&self, board: &mut S) -> bool {
        let did = board.get(&self.pos) == Poss;
        *(board.getm(&self.pos)) = True;
        did
    }

    fn affects(
        &self,
        _board: &S,
        _v: &T
    ) -> Vec<T> {
        vec![]
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct OhHiRow3Constraint;

impl Clone for OhHiRow3Constraint {
    fn clone(&self) -> Self {
        Self
    }

    fn clone_from(&mut self, source: &Self) {}
}

impl<S : Board<PosOnOff<SIZE>>, const SIZE:usize> Constraint<PosOnOff<SIZE>, S> for OhHiRow3Constraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for x in 0..SIZE {
            for y in 1..SIZE {
                for z in [true, false] {
                    let pos1 = PosOnOff::from((x, y, z));
                    let pos2 = PosOnOff::from((x, y - 1, z));
                    if board.get(&pos1) == True {
                        if board.get(&pos2) == True {
                            if y > 1 {
                                let clear_pos = PosOnOff::from((x, y - 2, z));
                                *(board.getm(&clear_pos)) = match board.get(&clear_pos) {
                                    Poss => {did = true; False}
                                    _ => {False}
                                };
                            }
                            if y < SIZE - 1 {
                                let clear_pos = PosOnOff::from((x, y + 1, z));
                                *(board.getm(&clear_pos)) = match board.get(&clear_pos) {
                                    Poss => {did = true; False}
                                    _ => {False}
                                };
                            }
                        }
                        if y > 1 {
                            let pos3 = PosOnOff::from((x, y - 2, z));
                            if board.get(&pos3) == True {
                                *(board.getm(&pos2)) = match board.get(&pos2) {
                                    Poss => {did = true; False}
                                    _ => {False}
                                };
                            }
                        }
                    }
                }
            }
        }
        did
    }

    fn affects(&self, board: &S, v: &PosOnOff<SIZE>) -> Vec<PosOnOff<SIZE>> {
        let mut ret = vec![];
        if board.get(&v) != Poss {
            return ret;
        }
        let (x,y,z) = v.pos;
        if y==0 || y==SIZE-1 {
            return ret;
        }
        if board.get(&PosOnOff::from((x,y-1,z))) == True {
            let clear_pos = PosOnOff::from((x, y + 1, z));
            ret.push(clear_pos);
        }
        else if board.get(&PosOnOff::from((x,y+1,z))) == True {
            let clear_pos = PosOnOff::from((x, y - 1, z));
            ret.push(clear_pos);
        }
        if y>1 {
            if board.get(&PosOnOff::from((x, y-2, z))) == True {
                let clear_pos = PosOnOff::from((x , y-1, z));
                ret.push(clear_pos);
            }
            if board.get(&PosOnOff::from((x, y-1, z))) == True {
                let clear_pos = PosOnOff::from((x, y-2, z));
                ret.push(clear_pos);
            }
        }
        if y< SIZE-2 {
            if board.get(&PosOnOff::from((x, y+2, z))) == True {
                let clear_pos = PosOnOff::from((x , y+1, z));
                ret.push(clear_pos);
            }
            if board.get(&PosOnOff::from((x, y+1, z))) == True {
                let clear_pos = PosOnOff::from((x, y+2, z));
                ret.push(clear_pos);
            }
        }
        ret
    }
}

#[derive(Debug)]
pub struct OhHiCol3Constraint;

impl Clone for OhHiCol3Constraint {
    fn clone(&self) -> Self {
        Self
    }

    fn clone_from(&mut self, source: &Self) {}
}

impl<S : Board<PosOnOff<SIZE>>, const SIZE:usize> Constraint<PosOnOff<SIZE>, S> for OhHiCol3Constraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for x in 1..SIZE {
            for y in 0..SIZE {
                for z in [true, false] {
                    let pos1 = PosOnOff::from((x, y, z));
                    let pos2 = PosOnOff::from((x-1, y, z));
                    if board.get(&pos1) == True {
                        if board.get(&pos2) == True {
                            if x > 1 {
                                let clear_pos = PosOnOff::from((x - 2, y, z));
                                *(board.getm(&clear_pos)) = match board.get(&clear_pos) {
                                    Poss => {did = true; False}
                                    _ => {False}
                                };
                            }
                            if x < SIZE - 1 {
                                let clear_pos = PosOnOff::from((x + 1, y, z));
                                *(board.getm(&clear_pos)) = match board.get(&clear_pos) {
                                    Poss => {did = true; False}
                                    _ => {False}
                                };
                            }
                        }
                        if x > 1 {
                            let pos3 = PosOnOff::from((x-2, y, z));
                            if board.get(&pos3) == True {
                                *(board.getm(&pos2)) = match board.get(&pos2) {
                                    Poss => {did = true; False}
                                    _ => {False}
                                };
                            }
                        }
                    }
                }
            }
        }
        did
    }

    fn affects(&self, board: &S, v: &PosOnOff<SIZE>) -> Vec<PosOnOff<SIZE>> {
        let mut ret = vec![];
        if board.get(&v) != Poss {
            return ret;
        }
        let (x,y,z) = v.pos;
        if x==0 || x==SIZE-1 {
            return ret;
        }
        if board.get(&PosOnOff::from((x-1,y,z))) == True {
            let clear_pos = PosOnOff::from((x+1, y, z));
            ret.push(clear_pos);
        }
        else if board.get(&PosOnOff::from((x+1,y,z))) == True {
            let clear_pos = PosOnOff::from((x-1, y , z));
            ret.push(clear_pos);
        }
        if x>1 {
            if board.get(&PosOnOff::from((x - 2, y, z))) == True {
                let clear_pos = PosOnOff::from((x - 1, y, z));
                ret.push(clear_pos);
            }
            if board.get(&PosOnOff::from((x - 1, y, z))) == True {
                let clear_pos = PosOnOff::from((x - 2, y, z));
                ret.push(clear_pos);
            }
        }
        if x< SIZE-2 {
            if board.get(&PosOnOff::from((x + 2, y, z))) == True {
                let clear_pos = PosOnOff::from((x + 1, y, z));
                ret.push(clear_pos);
            }
            if board.get(&PosOnOff::from((x + 1, y, z))) == True {
                let clear_pos = PosOnOff::from((x + 2, y, z));
                ret.push(clear_pos);
            }
        }
        ret
    }
}

#[derive(Debug)]
pub struct OhHiRowBalancedConstraint;

impl Clone for OhHiRowBalancedConstraint {
    fn clone(&self) -> Self {
        Self
    }

    fn clone_from(&mut self, source: &Self) {}
}

impl<S : Board<PosOnOff<SIZE>>, const SIZE:usize> Constraint<PosOnOff<SIZE>, S> for OhHiRowBalancedConstraint {
    fn apply(&self, board: &mut S) -> bool {
        assert_eq!(SIZE%2, 0, "SIZE must be even");
        let mut did = false;
        for x in 0..SIZE {
            for z in [true, false] {
                let mut count = 0;
                for y in 0..SIZE {
                    let pos1 = PosOnOff::from((x, y, z));
                    if board.get(&pos1) == True {
                        count += 1;
                    }
                }
                if count == SIZE/2 {
                    for y in 0..SIZE {
                        let pos = PosOnOff::from((x,y,z));
                        *(board.getm(&pos)) = match board.get(&pos) {
                            True => {True}
                            False => {False}
                            Poss => {did = true; False}
                        };
                    }
                }
            }
        }
        did
    }

    fn affects(&self, board: &S, v: &PosOnOff<SIZE>) -> Vec<PosOnOff<SIZE>> {
        assert_eq!(SIZE%2, 0, "SIZE must be even");
        let mut ret = vec![];
        if board.get(&v) != Poss {
            return ret;
        }
        let (x,y,z) = v.pos;
        let mut count = 0;
        for y_ in 0..SIZE {
            let pos = PosOnOff::from((x,y_,z));
            if board.get(&pos) == True {
                count += 1;
            }
        }
        if count == (SIZE/2 -1) {
            for y_ in 0..SIZE {
                if y_==y {
                    continue;
                }
                let pos = PosOnOff::from((x,y_,z));
                if board.get(&pos) == Poss {
                    ret.push(pos);
                }
            }
        }
        ret
    }
}

#[derive(Debug)]
pub struct OhHiColBalancedConstraint;

impl Clone for OhHiColBalancedConstraint {
    fn clone(&self) -> Self {
        Self
    }

    fn clone_from(&mut self, source: &Self) {}
}

impl<S : Board<PosOnOff<SIZE>>, const SIZE:usize> Constraint<PosOnOff<SIZE>, S> for OhHiColBalancedConstraint {
    fn apply(&self, board: &mut S) -> bool {
        assert_eq!(SIZE%2, 0, "SIZE must be even");
        let mut did = false;
        for y in 0..SIZE {
            for z in [true, false] {
                let mut count = 0;
                for x in 0..SIZE {
                    let pos1 = PosOnOff::from((x, y, z));
                    if board.get(&pos1) == True {
                        count += 1;
                    }
                }
                if count == SIZE/2 {
                    for x in 0..SIZE {
                        let pos = PosOnOff::from((x,y,z));
                        *(board.getm(&pos)) = match board.get(&pos) {
                            True => {True}
                            False => {False}
                            Poss => {did = true; False}
                        };
                    }
                }
            }
        }
        did
    }

    fn affects(&self, board: &S, v: &PosOnOff<SIZE>) -> Vec<PosOnOff<SIZE>> {
        assert_eq!(SIZE%2, 0, "SIZE must be even");
        let mut ret = vec![];
        if board.get(&v) != Poss {
            return ret;
        }
        let (x,y,z) = v.pos;
        let mut count = 0;
        for x_ in 0..SIZE {
            let pos = PosOnOff::from((x_,y,z));
            if board.get(&pos) == True {
                count += 1;
            }
        }
        if count == SIZE/2 -1 {
            for x_ in 0..SIZE {
                if x_==x {
                    continue;
                }
                let pos = PosOnOff::from((x_,y,z));
                if board.get(&pos) == Poss {
                    ret.push(pos);
                }
            }
        }
        ret
    }
}

fn same_row<S: Board<PosOnOff<SIZE>>, const SIZE: usize>(board: &S, full: usize, test: usize, full_full: bool) -> Option<(PosOnOff<SIZE>, PosOnOff<SIZE>)> {
    let mut first = None;
    let mut second = None;
    let mut fullc = 0;
    for y in 0..SIZE {
        let pos1t = PosOnOff::from((full, y, true));
        let pos1f = PosOnOff::from((full, y, false));
        let pos2t = PosOnOff::from((test, y, true));
        let pos2f = PosOnOff::from((test, y, false));
        if board.get(&pos1t) == Poss || board.get(&pos1f) == Poss {
            if full_full {
                return None;
            }
            else {
                fullc += 1;
            }
            if fullc > 2 {
                return None;
            }
        }
        if board.get(&pos1t) == True && board.get(&pos2t) == False {
            return None;
        }
        if board.get(&pos1f) == True && board.get(&pos2f) == False {
            return None;
        }
        if board.get(&pos2t) == Poss || board.get(&pos2f) == Poss {
            let pos = match board.get(&pos1t) {
                True => {pos2t}
                False => {pos2f}
                Poss => {if full_full {panic!("Row not complete");} pos2t}
            };
            if first == None {
                first = Some(pos);
            }
            else if second == None {
                second = Some(pos);
            }
            else {
                return None;
            }
        }
    }
    match first {
        None => {None}
        Some(f) => {match second {
            None => {None}
            Some(s) => {Some((f,s))}
        }}
    }
}

#[derive(Debug)]
pub struct OhHiRowUniqueConstraint;

impl Clone for OhHiRowUniqueConstraint {
    fn clone(&self) -> Self {
        Self
    }

    fn clone_from(&mut self, source: &Self) {}
}

impl<S : Board<PosOnOff<SIZE>>, const SIZE:usize> Constraint<PosOnOff<SIZE>, S> for OhHiRowUniqueConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for x in 0..SIZE {
            for x2 in 0..SIZE {
                if x == x2 {
                    continue;
                }
                match same_row(board, x, x2, true) {
                    None => {}
                    Some((f,s)) => {
                        did = true;
                        *(board.getm(&f)) = False;
                        *(board.getm(&s)) = False;
                    }
                }
            }
        }
        did
    }

    fn affects(&self, board: &S, v: &PosOnOff<SIZE>) -> Vec<PosOnOff<SIZE>> {
        let mut ret = vec![];
        if board.get(v) != Poss {
            return ret;
        }
        for x in 0..SIZE {
            let (x_,_,_) = v.pos;
            if x==x_ {
                continue;
            }
            let pair = same_row(board, x_, x, false);
            match pair {
                None => {}
                Some((_,_)) => {
                    ret.push(PosOnOff::from((x, v.pos.1, v.pos.2)));
                }
            }
        }
        ret
    }
}

fn same_col<S: Board<PosOnOff<SIZE>>, const SIZE: usize>(board: &S, full: usize, test: usize, full_full:bool) -> Option<(PosOnOff<SIZE>, PosOnOff<SIZE>)> {
    let mut first = None;
    let mut second = None;
    let mut fullc = 0;
    for x in 0..SIZE {
        let pos1t = PosOnOff::from((x, full, true));
        let pos1f = PosOnOff::from((x, full, false));
        let pos2t = PosOnOff::from((x, test, true));
        let pos2f = PosOnOff::from((x, test, false));
        if board.get(&pos1t) == Poss || board.get(&pos1f) == Poss {
            if full_full {
                return None;
            }
            else {
                fullc += 1;
            }
            if fullc > 2 {
                return None;
            }
        }
        if board.get(&pos1t) == True && board.get(&pos2t) == False {
            return None;
        }
        if board.get(&pos1f) == True && board.get(&pos2f) == False {
            return None;
        }
        if board.get(&pos2t) == Poss || board.get(&pos2f) == Poss {
            let pos = match board.get(&pos1t) {
                True => {pos2t}
                False => {pos2f}
                Poss => {if full_full {panic!("Row not complete");} pos2t}
            };
            if first == None {
                first = Some(pos);
            }
            else if second == None {
                second = Some(pos);
            }
            else {
                return None;
            }
        }
    }
    match first {
        None => {None}
        Some(f) => {match second {
            None => {None}
            Some(s) => {Some((f,s))}
        }}
    }
}

#[derive(Debug)]
pub struct OhHiColUniqueConstraint;

impl Clone for OhHiColUniqueConstraint {
    fn clone(&self) -> Self {
        Self
    }

    fn clone_from(&mut self, source: &Self) {}
}

impl<S : Board<PosOnOff<SIZE>>, const SIZE:usize> Constraint<PosOnOff<SIZE>, S> for OhHiColUniqueConstraint {
    fn apply(&self, board: &mut S) -> bool {
        let mut did = false;
        for y in 0..SIZE {
            for y2 in 0..SIZE {
                if y == y2 {
                    continue;
                }
                match same_col(board, y, y2, true) {
                    None => {}
                    Some((f,s)) => {
                        did = true;
                        *(board.getm(&f)) = False;
                        *(board.getm(&s)) = False;
                    }
                }
            }
        }
        did
    }

    fn affects(&self, board: &S, v: &PosOnOff<SIZE>) -> Vec<PosOnOff<SIZE>> {
        let mut ret = vec![];
        if board.get(v) != Poss {
            return ret;
        }
        for y in 0..SIZE {
            let (_,y_,_) = v.pos;
            if y==y_ {
                continue;
            }
            let pair = same_col(board, y_, y, false);
            match pair {
                None => {}
                Some((_,_)) => {
                    ret.push(PosOnOff::from((v.pos.0, y, v.pos.2)));
                }
            }
        }
        ret
    }
}