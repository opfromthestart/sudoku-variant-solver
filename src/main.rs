use crate::board::LogicVal::False;
use crate::board::{Puzzle, LogicVal, SdkBoard};
use crate::constraints::{CellConstraint, ColExistConstraint, ColUniqueConstraint, Constraint, DigitExistConstraint, DigitUniqueConstraint, GivenConstraint, LessThanConstraint, RowExistConstraint, RowUniqueConstraint};

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
}

fn main() {
    let t = GameType::Futoshiki;
    const size : usize = 5;

    let mut cons: Vec<Box<dyn Constraint<_, SdkBoard<size>>>> = vec![
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
            cons.push(Box::new(CellConstraint { cells: i }));
        }
    } else if t == GameType::Futoshiki || t == GameType::Thermo {
        let pairs = vec![
            ((0,2),(0,3)),
((0,3),(0,4)),
((0,1),(1,1)),
((1,2),(1,3)),
((0,4),(1,4)),
((2,0),(2,1)),
((1,3),(2,3)),
((3,4),(2,4)),
((4,1),(3,1)),
((4,2),(3,2)),
        ];
        for (l, h) in pairs {
            cons.push(Box::new(LessThanConstraint { lpos: l, hpos: h }));
        }
    }

    let givens = vec![];
    for i in givens {
        cons.push(Box::new(GivenConstraint { pos: i }));
    }

    let mut game = Puzzle::init(size);
    game.constraints = cons;

    //println!("{}", game.weak_hint());

    let mut tries = 0;
    while game.solve(false) {
        tries += 1;
        //println!("{}", game.board);
        //println!("{:?}", game.board);
        //println!("{}", tries);
    }
    //println!("{}", game.strong_hint());

    println!("Rounds of filling: {}", tries);
    println!("{}", game.board);
    println!("{:?}", game.board);
    println!("document.getElementById(\"puzzleForm\").onsubmit = function() {{Game.saveState();Game.tickTimer();this.jstimerPersonal.value = Game.getTimer();this.ansH.value=\"{}\"}};\ndocument.getElementById(\"btnReady\").click();", game.board.serialize().unwrap());


}
