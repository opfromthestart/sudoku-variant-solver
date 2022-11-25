use crate::board::SdkStd::{False, Poss, True};
use crate::constraints::Constraint;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;
use std::fmt::{Debug, Display, Formatter};
use std::hash::BuildHasher;

// SudokuStandard
#[derive(Clone, Copy, Debug)]
pub enum SdkStd {
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

impl PartialEq<SdkStd> for &SdkStd {
    fn eq(&self, other: &SdkStd) -> bool {
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

pub struct Puzzle<S> {
    pub board: Board<S>,
    pub constraints: Vec<Box<dyn Constraint<S>>>,
    hasher : RandomState,
}

pub struct Board<S> {
    pub data: Vec<S>,
    pub size: usize,
}

impl<S> Board<S> {
    pub fn getm(&mut self, x: usize, y: usize, z: usize) -> &mut S {
        (self.data[x * self.size * self.size + y * self.size + z]).borrow_mut()
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> &S {
        &(self.data[x * self.size * self.size + y * self.size + z])
    }
}

impl Board<SdkStd> {
    pub fn serialize(&self) -> Option<String> {
        let mut s = String::from("");
        for x in 0..self.size {
            for y in 0..self.size {
                let mut has_digit = false;
                for z in 0..self.size {
                    match self.get(x, y, z) {
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

    pub fn num_solved(&self) -> usize {
        let mut num = 0;
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    match self.get(x, y, z) {
                        True => {num += 1}
                        _ => {}
                    }
                }
            }
        }
        num
    }
}

impl Clone for Board<SdkStd> {
    fn clone(&self) -> Self {
        Board{ data: self.data.clone(), size: self.size}
    }

    fn clone_from(&mut self, source: &Self) where Self: {
        self.data = source.data.clone();
        self.size = source.size;
    }
}

impl Puzzle<SdkStd> {
    pub(crate) fn init(size: usize) -> Puzzle<SdkStd> {
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

    pub(crate) fn solve_simple(&mut self, slow: bool) -> bool {
        let mut did = false;
        for x in 0..self.board.size {
            for y in 0..self.board.size {
                    did = self.set_trues(x, y) || did;
            }
        }
        if did {
            return true;
        }
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
                if self.board.num_solved() == self.board.size*self.board.size {
                    return false;
                }
                eprintln!("Try loops");
                self.rem_odd_loops(None, slow).0
            }
        }
    }

    /// Gets the next cell that can be filled
    pub(crate) fn weak_hint(&mut self) -> String {
        let mut backup = self.board.clone();
        let start = self.board.num_solved();
        while self.board.num_solved() == start {
            let did = self.solve(true);
            if !did {
                self.board = backup;
                return String::from("No hint found");
            }
        }
        for x in 0..self.board.size {
            for y in 0..self.board.size {
                for z in 0..self.board.size {
                    if self.board.get(x, y, z) == True && backup.get(x, y, z) == Poss {
                        let row = char::from(65+(x as u8));
                        self.board = backup;
                        return format!("Consider cell {}{}.", row, y+1)
                    }
                }
            }
        }
        self.board = backup;
        panic!("Cell filled, but not found.");
    }

    /// Gets a hint on what cell to look at to fill and all cells to consider when removing it
    pub(crate) fn strong_hint(&mut self) -> String {
        let mut backup = self.board.clone();
        let start = self.board.num_solved();
        while self.board.num_solved() == start {
            let did = self.solve_simple(false);
            if !did {
                break;
            }
        }
        for x in 0..self.board.size {
            for y in 0..self.board.size {
                for z in 0..self.board.size {
                    if self.board.get(x, y, z) == True && backup.get(x, y, z) == Poss {
                        let row = char::from(65 + (x as u8));
                        self.board = backup;
                        return format!("Consider cell {}{}.", row, y + 1)
                    }
                }
            }
        }
        let mut cycles = vec![];
        while self.board.num_solved() == start {
            let (l, v) = self.find_odd_loops(None, true);
            cycles.extend(v);
            if !self.solve(true) {
                self.board = backup;
                return String::from("No hint found.");
            }
        }
        fn match2(a : &(usize, usize, usize), b : &(usize, usize, usize)) -> bool {
            let (ax, ay, az) = a.to_owned();
            let (bx, by, bz) = b.to_owned();
            if ax==bx && ay==by {
                return true;
            }
            if ax==bx && az==bz {
                return true;
            }
            ay==by && az==bz
        }

        for x in 0..self.board.size {
                for y in 0..self.board.size {
                    for z in 0..self.board.size {
                        if self.board.get(x, y, z) == True && backup.get(x, y, z) == Poss {
                            let row = char::from(65 + (x as u8));
                            //eprintln!("{},{},{}", x, y, z);
                            //for c in &cycles {
                            //    eprint!("{:?}", c);
                            //}
                            //eprintln!();
                            let matched_paths : Vec<&Vec<_>> = cycles.iter()
                                .filter(|v : &&Vec<(usize, usize, usize)>| v.iter().any(|b| match2(&(x,y,z), b))).collect();
                            for path in &matched_paths {
                                eprintln!("{:?}", path);
                            }
                            let matched_cells : Vec<Vec<_>> = matched_paths.iter().map(|v| v.iter().map(|(x,y,z)| (*x,*y)).collect()).collect();
                            //eprintln!("len:{}", matched_paths.len());
                            //let consider_paths : Vec<_> = all_paths
                            //eprintln!("len2:{}", &consider_paths.len());
                            self.board = backup;
                            let mut ret_str = format!("{}{}, \n", row, y+1);
                            for path in matched_cells {
                                let path_r : HashSet<(usize, usize)> = {
                                    let mut temp = HashSet::with_hasher(self.hasher.clone());
                                    temp.extend(path.iter());
                                    temp
                                };
                                for cell in path_r {
                                    //eprint!("c:{:?}", cell);
                                    let row = char::from(65 + (cell.0 as u8));
                                    ret_str += &*format!("{}{}, ", row, cell.1 + 1);
                                }
                                //eprintln!();
                                ret_str += "\n";
                            }
                            return format!("Consider cells {}", ret_str)
                        }
                    }
                }
            }
        self.board = backup;
        panic!("Cell filled, but not found.");
    }

    /// Gets all weak links from a given position
    fn get_weaks(&self, x: usize, y: usize, z: usize) -> HashSet<(usize, usize, usize)> {
        let mut ret = HashSet::with_hasher(self.hasher.clone());
        for con in &self.constraints {
            let temp = con.affects(&self.board, x, y, z);
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
    fn graph(&self) -> Graph<(usize, usize, usize)> {
        let mut graph: Graph<(usize, usize, usize)> = HashMap::with_hasher(self.hasher.clone());
        for x in 0..self.board.size {
            for y in 0..self.board.size {
                for z in 0..self.board.size {
                    if self.board.get(x, y, z) == Poss {
                        let mut node = GraphNode {
                            val: (x, y, z),
                            conn: vec![],
                        };
                        for i in self.get_weaks(x, y, z) {
                            match graph.get_mut(&i) {
                                Some(n) => {
                                    node.conn.push(i);
                                    n.conn.push((x, y, z));
                                }
                                None => {}
                            }
                        }
                        graph.insert((x, y, z), node);
                    }
                }
                //eprintln!("{}", graph.len());
            }
        }
        graph
    }

    /// Get the graph of strong links for the puzzle
    fn graph_strong(&self) -> Graph<(usize, usize, usize)> {
        let mut weak_graph = self.graph();
        let mut to_rem = Vec::new();
        for start in weak_graph.values() {
            for long1 in &start.conn {
                for long2 in &(weak_graph.get(long1).unwrap().conn) {
                    if start.conn.contains(long2) {
                        //eprintln!("({},{},{}),({},{},{}),({},{},{})", start.val.0, start.val.1, start.val.2, long1.0, long1.1, long1.2, long2.0, long2.1, long2.2);
                        to_rem.push((start.val, long1.to_owned()));
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
            to_visit.insert(*spos);
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
                        let (x_, y_, z_) = pos;
                        self.get_weaks(*x_, *y_, *z_);
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
                let s2 = HashSet::from_iter(new_visit.iter().map(|x| x.to_owned()));
                for v in &new_visit {
                    let s1 = HashSet::<(usize, usize, usize)>::from_iter(
                        check_graph
                            .get(v)
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
                        let (x, y, z) = spos;
                        //*(self.board.getm(*x,*y,*z)) = False;
                        to_rem.push((i, (*x, *y, *z)));
                        succ = true;
                        break 'findloop;
                    }
                }
                to_visit = new_visit;
            }
        }
        eprintln!("{}", min);
        for (i, (x, y, z)) in to_rem {
            if i <= min {
                *(self.board.getm(x, y, z)) = False;
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
    fn find_odd_loops(&self, max: Option<usize>, slow : bool) -> (usize, Vec<Vec<(usize, usize, usize)>>) {
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
        'allloop: for (spos, s) in &wg {
            let mut visited = HashMap::with_hasher(self.hasher.clone());
            let mut to_visit = HashMap::with_hasher(self.hasher.clone());
            to_visit.insert(*spos, None);
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
                        let pc = pos.clone();
                        let neighbors = &(graph.get(&pc).unwrap().conn);
                        let (x_, y_, z_) = pos;
                        self.get_weaks(*x_, *y_, *z_);
                        //for i in self.get_weaks(*x_,*y_,*z_) {
                        //    eprint!("({},{},{}),",i.0,i.1,i.2);
                        //}
                        for n in neighbors {
                            let nc = n.clone();
                            if !visited.contains_key(&nc) && !to_visit.contains_key(&nc) {
                                //eprintln!("visit {}: ({},{},{}),({},{},{})", i, pc.0,pc.1,pc.2, nc.0,nc.1,nc.2);
                                new_visit.insert(nc, Some(pc));
                            }
                        }
                        visited.insert(pc, *prev);
                    }
                }
                to_visit.drain();

                /// Checks to see if it can finish a loop
                let s2 : HashSet<(usize, usize, usize)> = HashSet::from_iter(new_visit.iter().map(|(x, y)| x.to_owned()));
                for v in &new_visit {
                    let s1 = HashSet::<(usize, usize, usize)>::from_iter(
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
                            w = Some(*i);
                            break;
                        }
                        min_i = i;
                        succ = true;
                        ends = (Some(v.0.to_owned()), w);
                        visited.extend(new_visit.iter());
                        break 'findloop;
                    }
                }
                to_visit = new_visit;
            }
            if let (Some(v), Some(w)) = ends {
                let mut ret = vec![];
                let mut pnt = Some(v.clone());
                while let Some(p) = pnt {
                    if visited.contains_key(&p) {
                        pnt = *visited.get(&p).unwrap();
                    }
                    else if to_visit.contains_key(&p) {
                        pnt = *to_visit.get(&p).unwrap();
                    }
                    ret.push(p);
                }
                let mut pnt = Some(w.clone());
                while let Some(p) = pnt {
                    if visited.contains_key(&p) {
                        pnt = *visited.get(&p).unwrap();
                    }
                    else if to_visit.contains_key(&p) {
                        pnt = *to_visit.get(&p).unwrap();
                    }
                    ret.push(p);
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

impl Display for Board<SdkStd> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    if self.get(x, y, z) == True {
                        write!(f, "{} ", z + 1)?;
                        break;
                    }
                    if z == (self.size - 1) {
                        write!(f, "? ")?;
                    }
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Debug for Board<SdkStd> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.size {
            for y in 0..self.size {
                write!(f, "[")?;
                for z in 0..self.size {
                    write!(f, "{:?},", *self.get(x, y, z))?;
                }
                write!(f, "]")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
