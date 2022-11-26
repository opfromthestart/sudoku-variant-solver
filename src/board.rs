use crate::board::LogicVal::{False, Poss, True};
use crate::constraints::Constraint;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{BuildHasher, Hash, Hasher};

// SudokuStandard
#[derive(Clone, Copy, Debug)]
pub enum LogicVal {
    True,
    Poss,
    False,
}

// T is the identifier/ value of each node
// Making it unique is good for my case
pub struct GraphNode<T> {
    val: T,
    conn: Vec<T>,
}

type Graph<'a, T> = HashMap<T, GraphNode<T>>;

impl PartialEq<LogicVal> for &LogicVal {
    fn eq(&self, other: &LogicVal) -> bool {
        match self {
            True => match other {
                True => true,
                _ => false,
            },
            Poss => match other {
                Poss => true,
                _ => false,
            },
            False => match other {
                False => true,
                _ => false,
            },
        }
    }
}

pub struct Puzzle<T : Eq + Hash + Enumerable + Clone, S : Board<T>> {
    pub board: S,
    pub constraints: Vec<Box<dyn Constraint<T,S>>>,
    hasher : RandomState,
}

/*
pub struct Board<S> {
    pub data: Vec<S>,
    pub size: usize,
}
 */

pub trait Enumerable {
    fn positions() -> Vec<Self> where Self: Sized;
}

pub struct Tuple3D<const MAX: usize> {
    pub(crate) pos : (usize, usize, usize)
}

impl<const MAX: usize> From<(usize, usize, usize)> for Tuple3D<MAX> {
    fn from(v: (usize, usize, usize)) -> Self {
        Tuple3D {pos : v}
    }
}

impl<const SIZE: usize> Hash for Tuple3D<SIZE> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl<const SIZE: usize>  PartialEq<Self> for Tuple3D<SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl<const SIZE: usize> Eq for Tuple3D<SIZE> {

}

impl<const MAX: usize> Enumerable for Tuple3D<MAX> {
    fn positions() -> Vec<Self> {
        let mut ret = Vec::new();
        for x in 0..MAX {
            for y in 0..MAX {
                for z in 0..MAX {
                    ret.push(Tuple3D::<MAX>::from((x, y, z)));
                }
            }
        }
        ret
    }
}

impl<const MAX:usize> Clone for Tuple3D<MAX> {
    fn clone(&self) -> Self {
        Self{pos: self.pos}
    }

    fn clone_from(&mut self, source: &Self) where Self: {
        self.pos = source.pos;
    }
}

impl<const MAX: usize> Copy for Tuple3D<MAX> {

}

pub trait Board<T : Eq + Hash + Enumerable> {
    fn getm(&mut self, x: &T) -> &mut LogicVal;

    fn get(&self, x: &T) -> &LogicVal;

    fn num_solved(&self) -> usize;

    fn max_solved(&self) -> usize;

    fn clone(&self) -> Self;
}

pub struct SdkBoard<const SIZE: usize> {
    pub data: Vec<LogicVal>,
}

impl<const SIZE: usize> SdkBoard<SIZE> {
    pub fn serialize(&self) -> Option<String> {
        let mut s = String::from("");
        for x in 0..SIZE {
            for y in 0..SIZE {
                let mut has_digit = false;
                for z in 0..SIZE {
                    match self.get(&Tuple3D::from((x, y, z))) {
                        True => {
                            s += &((z + 1).to_string() + ",");
                            has_digit = true;
                            break;
                        }
                        _ => {}
                    }
                }
                if !has_digit {
                    return None;
                }
            }
        }
        Some(s)
    }
}

impl<const SIZE: usize> Board<Tuple3D<SIZE>> for SdkBoard<SIZE> {
    fn getm(&mut self, v: &Tuple3D<SIZE>) -> &mut LogicVal {
        let (x,y,z) = v.pos;
        self.data[SIZE*SIZE*x+SIZE*y+z].borrow_mut()
    }

    fn get(&self, v: &Tuple3D<SIZE>) -> &LogicVal {
        let (x,y,z) = v.pos;
        &self.data[SIZE * SIZE * x + SIZE * y + z]
    }

    fn num_solved(&self) -> usize {
        let mut num = 0;
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    match self.get(&Tuple3D::from((x, y, z))) {
                        True => {num += 1}
                        _ => {}
                    }
                }
            }
        }
        num
    }

    fn max_solved(&self) -> usize {
        SIZE*SIZE
    }

    fn clone(&self) -> Self {
        SdkBoard{ data: self.data.clone()}
    }
}

impl<T : Eq + Hash + Enumerable + Clone, S : Board<T>> Puzzle<T, S> {
    /*
    pub(crate) fn init(size: usize) -> Puzzle<T, S>;
    {
        let s = Self {
            board: Board {
                data: vec![Poss; size * size * size],
                size,
            },
            constraints: vec![],
            hasher: RandomState::new(),
        };
        s
    }


    fn set_trues(&mut self, x: usize, y: usize) -> bool {
        let mut can_fill = 0;
        let mut digit = self.board.size;
        let mut did = false;
        for z in 0..self.board.size {
            let r = self.board.get(x, y, z);
            if r != False {
                can_fill += 1;
                digit = z;
                if can_fill >= 2 {
                    break;
                }
            }
        }
        if can_fill == 1 {
            did = self.board.get(x,y,digit) == Poss;
            *(self.board.getm(x, y, digit)) = True;
        }
        can_fill = 0;
        for z in 0..self.board.size {
            let r = self.board.get(x, z, y);
            if r != False {
                can_fill += 1;
                digit = z;
                if can_fill >= 2 {
                    break;
                }
            }
        }
        if can_fill == 1 {
            did = did || self.board.get(x,digit,y) == Poss;
            *(self.board.getm(x, digit, y)) = True;
        }
        can_fill = 0;
        for z in 0..self.board.size {
            let r = self.board.get(z, y, x);
            if r != False {
                can_fill += 1;
                digit = z;
                if can_fill >= 2 {
                    break;
                }
            }
        }
        if can_fill == 1 {
            did = did || self.board.get(digit,y,x) == Poss;
            *(self.board.getm(digit, y, x)) = True;
        }
        did
    }
     */

    pub(crate) fn solve_simple(&mut self, slow: bool) -> bool {
        let mut did = false;
        for con in &self.constraints {
            if !slow {
                did = con.apply(&mut self.board) || did;
            }
            else {
                did = did || con.apply(&mut self.board);
            }
        }
        did
    }

    /// One iteration of attempting to solve the puzzle
    pub(crate) fn solve(&mut self, slow: bool) -> bool {
        match self.solve_simple(slow) {
            true => true,
            false => {
                if self.board.num_solved() == self.board.max_solved() {
                    return false;
                }
                eprintln!("Try loops");
                self.rem_odd_loops(None, slow).0
            }
        }
    }

    /// Gets the next cell that can be filled
    pub(crate) fn weak_hint(&mut self) -> Option<T> {
        let mut backup = self.board.clone();
        let start = self.board.num_solved();
        while self.board.num_solved() == start {
            let did = self.solve(true);
            if !did {
                self.board = backup;
                //return String::from("No hint found");
                return None;
            }
        }
        for v in T::positions() {
            if self.board.get(&v) == True && backup.get(&v) == Poss {
                //let row = char::from(65 + (x as u8));
                self.board = backup;
                //return format!("Consider cell {}{}.", row, y + 1)
                return Some(v);
            }
        }
        self.board = backup;
        panic!("Cell filled, but not found.");
    }

    /// Gets a hint on what cell to look at to fill and all cells to consider when removing it
    pub(crate) fn strong_hint(&mut self) -> Vec<T> {
        let mut backup = self.board.clone();
        let start = self.board.num_solved();
        while self.board.num_solved() == start {
            let did = self.solve_simple(false);
            if !did {
                break;
            }
        }
        for v in T::positions() {
            if self.board.get(&v) == True && backup.get(&v) == Poss {
                //let row = char::from(65 + (x as u8));
                self.board = backup;
                //return format!("Consider cell {}{}.", row, y + 1)
                return vec![v];
            }
        }
        let mut cycles = vec![];
        while self.board.num_solved() == start {
            let (l, v) = self.find_odd_loops(None, true);
            cycles.extend(v);
            if !self.solve(true) {
                self.board = backup;
                //return String::from("No hint found.");
                return vec![];
            }
        }

        for pos in T::positions() {
            if self.board.get(&pos) == True && backup.get(&pos) == Poss {
                //let row = char::from(65 + (x as u8));
                //eprintln!("{},{},{}", x, y, z);
                //for c in &cycles {
                //    eprint!("{:?}", c);
                //}
                //eprintln!();
                let matched_paths: Vec<&Vec<_>> = cycles.iter()
                    .filter(|v: &&Vec<T>| v.iter().any(|b| self.get_weaks(&pos).contains(b))).collect();
                /*
                for path in &matched_paths {
                    eprintln!("{:?}", path);
                }
                 */
                //let matched_cells: Vec<Vec<_>> = matched_paths.iter().map(|v| v.iter().map(|(x, y, z)| (*x, *y)).collect()).collect();
                //eprintln!("len:{}", matched_paths.len());
                //let consider_paths : Vec<_> = all_paths
                //eprintln!("len2:{}", &consider_paths.len());
                self.board = backup;
                //let mut ret_str = format!("{}{}, \n", row, y + 1);
                let mut ret_vec = vec![];
                for path in matched_paths {
                    let path_r: HashSet<&T> = {
                        let mut temp = HashSet::with_hasher(self.hasher.clone());
                        temp.extend(path.iter());
                        temp
                    };
                    for cell in path_r {
                        //eprint!("c:{:?}", cell);
                        //let row = char::from(65 + (cell.0 as u8));
                        //ret_str += &*format!("{}{}, ", row, cell.1 + 1);
                        ret_vec.push(cell.clone());
                    }
                    //eprintln!();
                    //ret_str += "\n";
                }
                //return format!("Consider cells {}", ret_str)
                return ret_vec;
            }
        }
        self.board = backup;
        panic!("Cell filled, but not found.");
    }

    /// Gets all weak links from a given position
    fn get_weaks(&self, pos: &T) -> HashSet<T> {
        let mut ret = HashSet::with_hasher(self.hasher.clone());
        for con in &self.constraints {
            let temp = con.affects(&self.board, pos);
            /*
            for i in &temp {
                eprint!("({},{},{}),", i.0,i.1,i.2);
            }
            if temp.len()>0 {
                eprintln!();
            }

             */
            ret.extend(temp);
        }
        ret
    }

    // xwing done using graphs
    /*
    fn do_squares(&mut self) {
        for x in 0..self.board.size {
            for y in 0..self.board.size {
                for z in 0..self.board.size {
                    let one = self.affects(x, y, z);
                    let mut three = self.affects(x, y, z);
                    for i in 0..3 {
                        three = three
                                .iter()
                                .map(|(x_, y_, z_)| self.affects(*x_, *y_, *z_))
                                .reduce(|s1, s2| s1.union(&s2).map(|x| x.to_owned()).collect())
                                .unwrap();
                        if i<2 {
                            three = HashSet::from_iter(three.difference(&one).map(|x| x.to_owned()));
                        }
                    }

                }
            }
        }
    }
     */

    /// Get the graph of weak links for the puzzle
    fn graph(&self) -> Graph<T> {
        let mut graph: Graph<T> = HashMap::with_hasher(self.hasher.clone());
        for pos in T::positions() {
            if self.board.get(&pos) == Poss {
                let mut node = GraphNode {
                    val: (&pos).clone(),
                    conn: vec![],
                };
                for i in self.get_weaks(&pos) {
                    match graph.get_mut(&i) {
                        Some(n) => {
                            node.conn.push(i);
                            n.conn.push(pos.clone());
                        }
                        None => {}
                    }
                }
                graph.insert(pos, node);
            }
        }
                //eprintln!("{}", graph.len());
        graph
    }

    /// Get the graph of strong links for the puzzle
    fn graph_strong(&self) -> Graph<T> {
        let mut weak_graph = self.graph();
        let mut to_rem = Vec::new();
        for start in weak_graph.values() {
            for long1 in &start.conn {
                for long2 in &(weak_graph.get(long1).unwrap().conn) {
                    if start.conn.contains(long2) {
                        //eprintln!("({},{},{}),({},{},{}),({},{},{})", start.val.0, start.val.1, start.val.2, long1.0, long1.1, long1.2, long2.0, long2.1, long2.2);
                        to_rem.push((start.val.clone(), long1.to_owned()));
                    }
                }
            }
        }
        for (s, e) in to_rem {
            //eprintln!("({},{},{}),({},{},{})", s.0, s.1, s.2, e.0, e.1, e.2);
            weak_graph.get_mut(&s).unwrap().conn.retain(|x| *x != e);
            weak_graph.get_mut(&e).unwrap().conn.retain(|x| *x != s);
        }
        weak_graph
    }

    /// Basically just inference chain algorithm
    /// Returns true if it did something
    /// @param max: max number of iterations to try
    /// @param slow: whether to do only one removal per call. Is not for efficiency
    fn rem_odd_loops(&mut self, max: Option<usize>, slow: bool) -> (bool, usize) {
        let m = match max {
            None => 20,
            Some(e) => e,
        };

        let mut min = m + 1;
        let mut to_rem = vec![];

        let wg = self.graph();
        let sg = self.graph_strong();
        let mut wsum = 0;
        for j in wg.values() {
            wsum += j.conn.len();
        }
        let mut ssum = 0;
        for j in sg.values() {
            ssum += j.conn.len();
        }
        eprintln!("ln:{},{}", ssum / 2, wsum / 2);
        let mut succ = false;
        for (spos, s) in &wg {
            let mut visited = HashSet::with_hasher(self.hasher.clone());
            let mut to_visit = HashSet::with_hasher(self.hasher.clone());
            to_visit.insert(spos.clone());
            let mut need_strong = false;
            'findloop: for i in 0..min {
                let (graph, check_graph, csum) = if need_strong {
                    (&sg, &wg, wsum)
                } else {
                    (&wg, &sg, ssum)
                };
                need_strong = !need_strong;
                let mut new_visit = HashSet::with_hasher(self.hasher.clone());
                {
                    for pos in &to_visit {
                        let pc = pos.clone();
                        let neighbors = &(graph.get(&pc).unwrap().conn);
                        //let (x_, y_, z_) = pos;
                        self.get_weaks(pos);
                        //for i in self.get_weaks(*x_,*y_,*z_) {
                        //    eprint!("({},{},{}),",i.0,i.1,i.2);
                        //}
                        for n in neighbors {
                            let nc = n.clone();
                            if !visited.contains(&nc) && !to_visit.contains(&nc) {
                                //eprintln!("visit {}: ({},{},{}),({},{},{})", i, pc.0,pc.1,pc.2, nc.0,nc.1,nc.2);
                                new_visit.insert(nc);
                            }
                        }
                        visited.insert(pc);
                    }
                }
                to_visit.drain();
                let s2 = HashSet::from_iter(new_visit.iter().map(|x| x.clone()));
                for v in &new_visit {
                    let s1 = HashSet::<T>::from_iter(
                        check_graph
                            .get(v)
                            .unwrap()
                            .conn
                            .iter()
                            .map(|x| x.clone()),
                    );
                    let inter: HashSet<_> = s1.intersection(&s2).collect();
                    if inter.len() > 0 {
                        //eprintln!("From {}:({},{},{}), {}", i, v.0,v.1,v.2, csum/2);
                        /*
                        for i in inter {
                            eprintln!("({},{},{})", i.0,i.1,i.2);
                        }
                         */
                        //eprintln!("Removes:({},{},{})", spos.0, spos.1, spos.2);
                        if i < min {
                            min = i;
                        }
                        //let (x, y, z) = spos;
                        //*(self.board.getm(*x,*y,*z)) = False;
                        to_rem.push((i, spos));
                        succ = true;
                        break 'findloop;
                    }
                }
                to_visit = new_visit;
            }
        }
        eprintln!("{}", min);
        for (i, pos) in to_rem {
            if i <= min {
                *(self.board.getm(pos)) = False;
                if slow {
                    return (succ, min);
                }
            }
        }
        (succ, min)
    }

    /// Does rem_odd_loops but is able to backtrack.
    /// @param max: max number of iterations to try
    /// @param slow: whether to do only one removal per call. Is not for efficiency
    fn find_odd_loops(&self, max: Option<usize>, slow : bool) -> (usize, Vec<Vec<T>>) {
        let m = match max {
            None => 20,
            Some(e) => e,
        };

        let mut min = m + 1;
        let mut to_rem = vec![];

        let wg = self.graph();
        let sg = self.graph_strong();
        let mut wsum = 0;
        for j in wg.values() {
            wsum += j.conn.len();
        }
        let mut ssum = 0;
        for j in sg.values() {
            ssum += j.conn.len();
        }
        eprintln!("ln:{},{}", ssum / 2, wsum / 2);
        let mut succ = false;
        for (spos, s) in &wg {
            let mut visited = HashMap::with_hasher(self.hasher.clone());
            let mut to_visit = HashMap::with_hasher(self.hasher.clone());
            to_visit.insert(spos.clone(), None);
            let mut need_strong = false;
            let mut min_i = min;
            let mut ends = (None,None);
            'findloop: for i in 0..min {
                let (graph, check_graph, csum) = if need_strong {
                    (&sg, &wg, wsum)
                } else {
                    (&wg, &sg, ssum)
                };
                need_strong = !need_strong;
                let mut new_visit = HashMap::with_hasher(self.hasher.clone());
                {
                    for (pos, prev) in &to_visit {
                        let neighbors = &(graph.get(pos).unwrap().conn);
                        //let (x_, y_, z_) = pos;
                        self.get_weaks(pos);
                        //for i in self.get_weaks(*x_,*y_,*z_) {
                        //    eprint!("({},{},{}),",i.0,i.1,i.2);
                        //}
                        for n in neighbors {
                            let nc = n.clone();
                            if !visited.contains_key(&nc) && !to_visit.contains_key(&nc) {
                                //eprintln!("visit {}: ({},{},{}),({},{},{})", i, pc.0,pc.1,pc.2, nc.0,nc.1,nc.2);
                                new_visit.insert(nc, Some(pos.clone()));
                            }
                        }
                        visited.insert(pos.clone(), prev.clone());
                    }
                }
                to_visit.drain();

                /// Checks to see if it can finish a loop
                let s2 : HashSet<T> = HashSet::from_iter(new_visit.iter().map(|(x, y)| x.to_owned()));
                for v in &new_visit {
                    let s1 = HashSet::<T>::from_iter(
                        check_graph
                            .get(&v.0)
                            .unwrap()
                            .conn
                            .iter()
                            .map(|x| x.to_owned()),
                    );
                    let inter: HashSet<_> = s1.intersection(&s2).collect();
                    if inter.len() > 0 {
                        //eprintln!("From {}:({},{},{}), {}", i, v.0,v.1,v.2, csum/2);
                        /*
                        for i in inter {
                            eprintln!("({},{},{})", i.0,i.1,i.2);
                        }
                         */
                        //eprintln!("Removes:({},{},{})", spos.0, spos.1, spos.2);
                        if i < min {
                            min = i;
                        }
                        //let (x, y, z) = spos;
                        //*(self.board.getm(*x,*y,*z)) = False;
                        //to_rem.push((i, (*x, *y, *z)));
                        let mut w = None;
                        for i in inter {
                            w = Some(i.clone());
                            break;
                        }
                        min_i = i;
                        succ = true;
                        ends = (Some(v.0.to_owned()), w);
                        visited.extend(new_visit);
                        break 'findloop;
                    }
                }
                to_visit = new_visit;
            }
            if let (Some(v), Some(w)) = ends {
                let mut ret = vec![];
                let mut pnt = Some(v.clone());
                while let Some(p) = &pnt {
                    let next_pnt =
                    if visited.contains_key(p) {
                        visited.get(p).unwrap().clone()
                    }
                    else { // if to_visit.contains_key(p)
                        to_visit.get(p).unwrap().clone()
                    };
                    ret.push(p.clone());
                    pnt = next_pnt;
                }
                let mut pnt = Some(w.clone());
                while let Some(p) = &pnt {
                    let next_pnt =
                    if visited.contains_key(p) {
                        visited.get(p).unwrap().clone()
                    }
                    else { // if to_visit.contains_key(p)
                        to_visit.get(p).unwrap().clone()
                    };
                    ret.push(p.clone());
                    pnt = next_pnt;
                }
                to_rem.push((min_i,ret));
            }
        }
        eprintln!("{}", min);
        let mut ret = vec![];
        for (i, v) in to_rem {
            if i <= min {
                ret.push( v);
                if slow {
                    return (min, ret);
                }
            }
        }
        (min,ret)
    }
}

impl<const SIZE: usize>  Puzzle<Tuple3D<SIZE>, SdkBoard<SIZE>> {
    pub(crate) fn init(size: usize) -> Self
    {
        let s = Self {
            board: SdkBoard {
                data: vec![Poss; size * size * size],
            },
            constraints: vec![],
            hasher: RandomState::new(),
        };
        s
    }
}

impl<const SIZE: usize>  Display for SdkBoard<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    if self.get(&Tuple3D::from((x, y, z))) == True {
                        write!(f, "{} ", z + 1)?;
                        break;
                    }
                    if z == (SIZE - 1) {
                        write!(f, "? ")?;
                    }
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<const SIZE: usize>  Debug for SdkBoard<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..SIZE {
            for y in 0..SIZE {
                write!(f, "[")?;
                for z in 0..SIZE {
                    write!(f, "{:?},", *self.get(&Tuple3D::from((x, y, z))))?;
                }
                write!(f, "]")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
