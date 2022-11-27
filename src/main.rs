use crate::board::LogicVal::False;
use crate::board::{Puzzle, LogicVal, SdkBoard, Tuple3D, TFBoard, PosOnOff, Board, get_empty_pos};
use crate::constraints::{CellConstraint, ColExistConstraint, ColUniqueConstraint, Constraint, DigitExistConstraint, DigitUniqueConstraint, GivenConstraint, LessThanConstraint, OhHiCol3Constraint, OhHiColBalancedConstraint, OhHiColUniqueConstraint, OhHiRow3Constraint, OhHiRowBalancedConstraint, OhHiRowUniqueConstraint, RowExistConstraint, RowUniqueConstraint};

mod board;
mod constraints;

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

fn main() {
    let t = GameType::OhHi;
    const size : usize = 20;

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

            let mut game = Puzzle::<Tuple3D<size>,_>::init(size);
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
                (0,0,true),
(0,6,true),
(0,7,true),
(0,10,true),
(0,12,false),
(0,17,false),
(1,0,true),
(1,3,true),
(1,9,false),
(1,16,true),
(2,6,true),
(2,13,false),
(2,17,false),
(3,3,true),
(3,5,true),
(3,7,true),
(3,10,false),
(3,18,false),
(3,19,true),
(4,0,true),
(4,2,true),
(4,9,true),
(4,12,false),
(4,16,false),
(4,18,false),
(5,3,false),
(5,6,false),
(5,8,false),
(5,13,true),
(5,19,false),
(6,0,true),
(6,1,true),
(6,4,true),
(6,5,true),
(6,19,true),
(7,2,false),
(7,11,true),
(7,15,true),
(7,18,false),
(8,1,true),
(8,7,false),
(8,10,true),
(8,12,false),
(8,13,false),
(8,16,false),
(8,18,false),
(8,19,false),
(9,2,false),
(9,8,true),
(9,12,false),
(10,2,false),
(10,7,false),
(10,11,true),
(11,1,true),
(11,4,true),
(11,5,true),
(11,7,true),
(11,9,false),
(11,13,false),
(11,16,true),
(12,1,true),
(12,10,true),
(12,15,true),
(12,18,true),
(12,19,true),
(13,6,false),
(13,8,true),
(13,13,false),
(13,15,true),
(14,4,false),
(14,6,true),
(14,12,false),
(14,18,true),
(14,19,true),
(15,11,true),
(15,13,true),
(16,1,true),
(16,5,false),
(16,15,false),
(16,17,false),
(16,18,false),
(17,2,false),
(17,10,true),
(17,11,true),
(17,15,false),
(17,16,false),
(18,0,true),
(18,3,true),
(18,6,true),
(18,8,true),
(18,14,true),
(18,19,true),
(19,3,true),
(19,5,true),
(19,17,false),
(19,18,false),
            ];
            for i in givens {
                cons.push(Box::new(GivenConstraint { pos: PosOnOff::from(i) }));
            }

            let mut game = Puzzle::<PosOnOff<size>, _>::init(size);
            game.constraints = cons;

            let mut tries = 0;
            game.solve_simple(false);
            println!("{}", game.board);
            let mut last = game.board.clone();
            let mut chain = game.find_odd_loops(None, true);
            while game.solve_debug(true) {
                tries += 1;

                /*
                if game.board.get(&PosOnOff::from((9,0,false))) == LogicVal::False {
                    break;
                }
                last = game.board.clone();
                chain = game.find_odd_loops(None, true);

                 */
                //println!("{}", game.board);
                //println!("{:?}", game.board);
                //println!("{}", tries);
            }
            println!("{:?}", last);
            println!("{}", last);

            for i in chain.1 {
                println!("{:?}", i);
            }

            println!("{:?}", game.board);
            println!("{}", game.board);

        }
    }

}
