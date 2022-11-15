use crate::board::SdkStd::{False, Poss, True};
use crate::constraints::Constraint;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};

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

type Graph<'a, T> = HashMap<T,GraphNode<T>>;

impl PartialEq<SdkStd> for &SdkStd {
    fn eq(&self, other: &SdkStd) -> bool {
        match self {
            True => match other {
                True => true,
                _ => false
            }
            Poss => match other {
                Poss => true,
                _ => false
            }
            False => match other {
                False => true,
                _ => false
            }
        }
    }
}

pub struct Puzzle<S> {
    pub board: Board<S>,
    pub constraints: Vec<Box<dyn Constraint<S>>>,
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
                            s += &((z + 1).to_string()+",");
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

impl Puzzle<SdkStd> {
    pub(crate) fn init(size: usize) -> Puzzle<SdkStd> {
        let s = Self{board: Board{ data: vec![Poss; size * size * size], size}, constraints : vec![]};
        s
    }

    fn set_trues(&mut self, x: usize, y: usize) {
        let mut can_fill = 0;
        let mut digit = self.board.size;
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
            *(self.board.getm(x, y, digit)) = True;
            return;
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
            *(self.board.getm(x, digit, y)) = True;
            return;
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
            *(self.board.getm(digit, y, x)) = True;
        }
    }

    pub(crate) fn solve(&mut self) -> bool {
        for x in 0..self.board.size {
            for y in 0..self.board.size {
                self.set_trues(x, y);
            }
        }
        let mut did = false;
        for con in &self.constraints {
            did = con.apply(&mut self.board) || did;
        }
        match did {
            true => {true}
            false => {
                println!("Try loops");
                self.rem_odd_loops(None)
            }
        }
    }

    fn get_weaks(&self, x: usize, y: usize, z: usize) -> HashSet<(usize, usize, usize)> {
        let mut ret = HashSet::new();
        for con in &self.constraints {
            let temp = con.affects(&self.board, x, y, z);
            /*
            for i in &temp {
                print!("({},{},{}),", i.0,i.1,i.2);
            }
            if temp.len()>0 {
                println!();
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

    fn graph(&self) -> Graph<(usize,usize,usize)>{
        let mut graph : Graph<(usize, usize, usize)> = HashMap::new();
        for x in 0..self.board.size {
            for y in 0..self.board.size {
                for z in 0..self.board.size {
                    if self.board.get(x, y, z) == Poss {
                        let mut node = GraphNode { val: (x, y, z), conn: vec![] };
                        for i in self.get_weaks(x, y, z) {
                            match graph.get_mut(&i) {
                                Some(n) => {
                                    node.conn.push(i);
                                    n.conn.push((x, y, z));
                                }
                                None => {}
                            }
                        }
                        graph.insert((x,y,z), node);
                    }
                }
                //println!("{}", graph.len());
            }
        }
        graph
    }

    fn graph_strong(&self) -> Graph<(usize,usize,usize)> {
        let mut weak_graph = self.graph();
        let mut to_rem = Vec::new();
        for start in weak_graph.values() {
            for long1 in &start.conn {
                for long2 in &(weak_graph.get(long1).unwrap().conn) {
                    if start.conn.contains(long2) {
                        //println!("({},{},{}),({},{},{}),({},{},{})", start.val.0, start.val.1, start.val.2, long1.0, long1.1, long1.2, long2.0, long2.1, long2.2);
                        to_rem.push((start.val, long1.to_owned()));
                    }
                }
            }
        }
        for (s,e) in to_rem {
            //println!("({},{},{}),({},{},{})", s.0, s.1, s.2, e.0, e.1, e.2);
            weak_graph.get_mut(&s).unwrap().conn.retain(|x| *x != e);
            weak_graph.get_mut(&e).unwrap().conn.retain(|x| *x != s);
        }
        weak_graph
    }

    // Basically just inference chain algorithm
    // Returns true if it did something
    fn rem_odd_loops(&mut self, max: Option<usize>) -> bool{
        let m = match max {
            None => {20}
            Some(e) => {e}
        };

        let mut min = m+1;
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
        println!("ln:{},{}", ssum/2, wsum/2);
        let mut succ = false;
        for (spos, s) in &wg {
            let mut visited = HashSet::new();
            let mut to_visit = HashSet::from([*spos]);
            let mut need_strong = false;
            'findloop: for i in 0..min {
                let (graph, check_graph, csum) = if need_strong {
                    (&sg, &wg, wsum)
                } else { (&wg, &sg, ssum) };
                need_strong = !need_strong;
                let mut new_visit = HashSet::new();
                {
                    for pos in &to_visit {
                        let pc = pos.clone();
                        let neighbors = &(graph.get(&pc).unwrap().conn);
                        let (x_,y_,z_) = pos;
                        self.get_weaks(*x_,*y_,*z_);
                        //for i in self.get_weaks(*x_,*y_,*z_) {
                        //    print!("({},{},{}),",i.0,i.1,i.2);
                        //}
                        for n in neighbors {
                            let nc = n.clone();
                            if !visited.contains(&nc) && !to_visit.contains(&nc) {
                                //println!("visit {}: ({},{},{}),({},{},{})", i, pc.0,pc.1,pc.2, nc.0,nc.1,nc.2);
                                new_visit.insert(nc);
                            }
                        }
                        visited.insert(pc);
                    }
                }
                to_visit.drain();
                let s2 = HashSet::from_iter(new_visit.iter().map(|x| x.to_owned()));
                for v in &new_visit {
                    let s1 = HashSet::<(usize,usize,usize)>::from_iter(check_graph.get(v).unwrap().conn.iter().map(|x| x.to_owned()));
                    let inter : HashSet<_> = s1.intersection(&s2).collect();
                    if inter.len()>0 {
                        //println!("From {}:({},{},{}), {}", i, v.0,v.1,v.2, csum/2);
                        /*
                        for i in inter {
                            println!("({},{},{})", i.0,i.1,i.2);
                        }
                         */
                        //println!("Removes:({},{},{})", spos.0, spos.1, spos.2);
                        if i<min {
                            min = i;
                        }
                        let (x,y,z) = spos;
                        //*(self.board.getm(*x,*y,*z)) = False;
                        to_rem.push((i, (*x,*y,*z)));
                        succ = true;
                        break 'findloop;
                    }
                }
                to_visit = new_visit;
            }
        }
        println!("{}",min);
        for (i,(x,y,z)) in to_rem {
            if i <= min {
                *(self.board.getm(x,y,z)) = False;
            }
        }
        succ
    }
}

impl Display for Board<SdkStd> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    if self.get(x,y,z) == True {
                        write!(f, "{} ", z+1)?;
                        break;
                    }
                    if z==(self.size-1) {
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
                    write!(f, "{:?},", *self.get(x,y,z))?;
                }
                write!(f, "]")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}