use std::collections::hash_map::RandomState;
use crate::board::LogicVal::{False, True};
use crate::board::{Puzzle, LogicVal, SdkBoard, Tuple3D, TFBoard, PosOnOff, Board, get_empty_pos, differ};
use crate::constraints::{CellConstraint, ColExistConstraint, ColUniqueConstraint, Constraint, DigitExistConstraint, DigitUniqueConstraint, GivenConstraint, LessThanConstraint, OhHiCol3Constraint, OhHiColBalancedConstraint, OhHiColUniqueConstraint, OhHiRow3Constraint, OhHiRowBalancedConstraint, OhHiRowUniqueConstraint, RowExistConstraint, RowUniqueConstraint};
use crate::det_hash::DetBuildHash;

mod board;
mod constraints;
mod det_hash;

pub fn true_board<const SIZE: usize>() -> TFBoard<SIZE> {
    assert_eq!(SIZE, size);
                let mut true_vec = vec![
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    True, False,
                    False, True,
                    True, False,
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    True, False,
                    True, False,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    True, False,
                    True, False,
                    False, True,
                    True, False,
                    False, True,
                    False, True,
                ];
                TFBoard { data: true_vec }
            }

// Goals
// Offshoots of this one:
// https://www.puzzle-bridges.com/
/* Jigsaw
var cl = "";
var g = "";
for (var i=0; i<Game.puzzleWH; i++) {
  cl += "vec!["
  for (var j=0; j<Game.puzzleWH; j++) {
    if (Game.currentState.cellStatus[i][j].number != 0) {
      var num = parseInt(Game.currentState.cellStatus[i][j].number,20)-1;
      g += "("+i+","+j+","+num+"),\n"
    }
    ap = Game.areaPoints[i+1][j];
    cl += "("+ap.row+", "+ap.col+"),"
  }
  cl += "],\n"
}
console.log(cl);
console.log(g);
document.getElementById("robot").value = 1;
 */

/*  code for futoshiki
var cp = "";
var g = "";
for (var i=0; i<Game.puzzleWH; i++) {
  for (var j=0; j<Game.puzzleWH; j++) {
    if (Game.conditions[i][j].d) {
      cp += "(("+(i+1)+","+(j)+"),("+i+","+j+")),\n"
    }
    if (Game.conditions[i][j].u) {
      cp += "(("+(i-1)+","+(j)+"),("+i+","+j+")),\n"
    }
    if (Game.conditions[i][j].l) {
      cp += "(("+(i)+","+(j-1)+"),("+i+","+j+")),\n"
    }
    if (Game.conditions[i][j].r) {
      cp += "(("+(i)+","+(j+1)+"),("+i+","+j+")),\n"
    }
    if (Game.currentState.cellStatus[i][j].number != 0) {
      var num = parseInt(Game.currentState.cellStatus[i][j].number,20)-1;
      g += "("+i+","+j+","+num+"),\n"
    }
  }
}
console.log(cp);
console.log(g);
document.getElementById("robot").value = 1;
*/
/*  code for binario
var g = "";
for (var i=0; i<Game.puzzleHeight; i++) {
  for (var j=0; j<Game.puzzleHeight; j++) {
    if (Game.currentState.cellStatus[i][j] != 0) {
      var num = Game.currentState.cellStatus[i][j] == 1 ? "false" : "true";
      g += "("+i+","+j+","+num+"),\n"
    }
  }
}
console.log(g);
document.getElementById("robot").value = 1;
*/

/* Normal sudokuwiki.org
var gv = "";
for (var i=0; i<g.cells.length; i++) {
    if (g.cells[i].val != 0) {
      var num = parseInt(g.cells[i].val,20)-1;
      gv += "("+parseInt(i/9)+","+(i%9)+","+num+"),\n"
    }
}
console.log(gv);
 */

#[derive(PartialEq)]
pub enum GameType {
    Normal,
    Jigsaw,
    Futoshiki,
    Thermo,
    OhHi,
}

fn get_hint_string<const SIZE: usize>(vec: &Vec<Tuple3D<SIZE>>) -> String{
    let mut ret_str = String::new();
    for pos in vec {
        let row = char::from(65 + (pos.pos.0 as u8));
        ret_str += &*format!("{}{}, ", row, pos.pos.1 + 1);
    }
    if vec.len() <= 1 {
        format!("Consider cell: {}", ret_str)
    }
    else {
        format!("Consider cells: {}", ret_str)
    }
}

const size : usize = 10;

fn main() {
    let t = GameType::OhHi;

    match t {
        GameType::Normal | GameType::Jigsaw | GameType::Futoshiki | GameType::Thermo  => {
            let mut cons: Vec<Box<dyn Constraint<_,
                SdkBoard<size>>>> = vec![
                Box::new(RowUniqueConstraint),
                Box::new(ColUniqueConstraint),
                Box::new(DigitUniqueConstraint),
                Box::new(RowExistConstraint),
                Box::new(ColExistConstraint),
                Box::new(DigitExistConstraint),
            ];

            if t == GameType::Normal {
                for x in 0..3 {
                    for y in 0..3 {
                        cons.push(Box::new(CellConstraint {
                            cells: vec![
                                (3 * x, 3 * y),
                                (3 * x, 3 * y + 1),
                                (3 * x, 3 * y + 2),
                                (3 * x + 1, 3 * y),
                                (3 * x + 1, 3 * y + 1),
                                (3 * x + 1, 3 * y + 2),
                                (3 * x + 2, 3 * y),
                                (3 * x + 2, 3 * y + 1),
                                (3 * x + 2, 3 * y + 2),
                            ],
                        }));
                    }
                }
            } else if t == GameType::Jigsaw {
                let cells = vec![];
                for i in cells {
                    cons.push(Box::new(CellConstraint {
                        cells: i
                    }));
                }
            } else if t == GameType::Futoshiki || t == GameType::Thermo {
                let pairs = vec![
                    ((0, 2), (0, 3)),
                    ((0, 3), (0, 4)),
                    ((0, 1), (1, 1)),
                    ((1, 2), (1, 3)),
                    ((0, 4), (1, 4)),
                    ((2, 0), (2, 1)),
                    ((1, 3), (2, 3)),
                    ((3, 4), (2, 4)),
                    ((4, 1), (3, 1)),
                    ((4, 2), (3, 2)),
                ];
                for (l, h) in pairs {
                    cons.push(Box::new(LessThanConstraint { lpos: l, hpos: h }));
                }
            }

            let givens = vec![];
            for i in givens {
                cons.push(Box::new(GivenConstraint { pos: i }));
            }

            let mut game = Puzzle::<Tuple3D<size>,_,RandomState>::init(size);
            game.constraints = cons;

            println!("{}", get_hint_string(&vec![game.weak_hint().unwrap()]));

            let mut tries = 0;
            while game.solve_simple(false) {
                tries += 1;
                //println!("{}", game.board);
                //println!("{:?}", game.board);
                //println!("{}", tries);
            }
            println!("{}", get_hint_string(&game.strong_hint()));

            println!("Rounds of filling: {}", tries);
            println!("{:?}", game.board);
            println!("{}", game.board);
            println!("document.getElementById(\"puzzleForm\").onsubmit = function() {{Game.saveState();Game.tickTimer();this.jstimerPersonal.value = Game.getTimer();this.ansH.value=\"{}\"}};\ndocument.getElementById(\"btnReady\").click();", game.board.serialize().unwrap());
        }
        GameType::OhHi => {
            let mut cons: Vec<Box<dyn Constraint<_,
                TFBoard<size>>>> = vec![
                Box::new(DigitUniqueConstraint),
                Box::new(DigitExistConstraint),
                Box::new(OhHiRow3Constraint),
                Box::new(OhHiCol3Constraint),
                Box::new(OhHiRowBalancedConstraint),
                Box::new(OhHiColBalancedConstraint),
                Box::new(OhHiRowUniqueConstraint),
                Box::new(OhHiColUniqueConstraint),
            ];

            let givens = vec![
                (0,0,false),
(0,4,false),
(0,9,false),
(1,6,true),
(1,9,true),
(2,7,false),
(3,2,false),
(3,7,false),
(4,0,true),
(4,3,true),
(4,5,true),
(5,6,false),
(6,3,true),
(7,2,false),
(7,4,false),
(7,7,true),
(7,8,true),
(8,6,false),
(9,0,false),
(9,2,false),
(9,3,true),
(9,8,true),
            ];
            for i in givens {
                cons.push(Box::new(GivenConstraint { pos: PosOnOff::from(i) }));
            }


            let mut game = Puzzle::<PosOnOff<size>, _, DetBuildHash>::init(size);
            game.constraints = cons;

            let mut tries = 0;
            game.solve_simple(false);
            //println!("{}", PosOnOff::from((2,4,true)).ha);
            println!("{}", game.board);
            let mut last = game.board.clone();
            let mut chain = game.find_odd_loops(None, true);

            let fail = |b : &TFBoard<size>, con: &Box<dyn Constraint<PosOnOff<size>, TFBoard<size>>>| {match get_empty_pos(b) {
                None => {None}
                Some(v) => {Some(format!("{:?}\n{:?}", v, con))}
            }};

            let fail2 = |b : &TFBoard<size>, con: &Box<dyn Constraint<PosOnOff<size>, TFBoard<size>>>| {match b.get(&PosOnOff::from((1,3,true))) {
                LogicVal::Poss | LogicVal::True => {None}
                LogicVal::False => {Some(format!("{:?}", con))}
            }};

            let fail3 : Box<dyn Fn(&TFBoard<size>, &Box<dyn Constraint<PosOnOff<size>, TFBoard<size>>>) -> Option<String>> = Box::new(|b : &TFBoard<size>, con: &Box<dyn Constraint<PosOnOff<size>, TFBoard<size>>>|
                {match differ(b,&true_board()) {
                false => {None}
                true => {Some(format!("{:?}", con))}
            }});

            while game.solve_debug(true, &Box::new(&fail3)) {
                tries += 1;


                if game.board.get(&PosOnOff::from((1,3,true))) == LogicVal::False {
                    break;
                }
                last = game.board.clone();
                chain = game.find_odd_loops(None, true);




                //println!("{}", game.board);
                //println!("{:?}", game.board);
                //println!("{}", tries);
            }
            println!("{:?}", last);
            println!("{}", last);

            chain = game.find_odd_loops(None, true);
            for i in chain.1 {
                println!("{:?}", i);
            }

            println!("{:?}", game.board);
            println!("{}", game.board);

        }
    }

}
