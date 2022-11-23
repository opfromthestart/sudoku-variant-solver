use crate::board::SdkStd::False;
use crate::board::{Puzzle, SdkStd};
use crate::constraints::{
    CellConstraint, ColConstraint, Constraint, DigitConstraint, GivenConstraint,
    LessThanConstraint, RowConstraint,
};

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
    let size = 9;

    let mut cons: Vec<Box<dyn Constraint<SdkStd>>> = vec![
        Box::new(RowConstraint),
        Box::new(ColConstraint),
        Box::new(DigitConstraint),
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
            ((1, 0), (0, 0)),
            ((0, 4), (0, 3)),
            ((0, 7), (0, 6)),
            ((0, 7), (0, 8)),
            ((1, 4), (1, 3)),
            ((0, 6), (1, 6)),
            ((1, 6), (1, 7)),
            ((1, 8), (1, 7)),
            ((2, 0), (2, 1)),
            ((2, 2), (2, 1)),
            ((2, 3), (2, 2)),
            ((1, 5), (2, 5)),
            ((2, 4), (2, 5)),
            ((2, 5), (2, 6)),
            ((2, 8), (2, 7)),
            ((3, 1), (3, 2)),
            ((3, 2), (3, 3)),
            ((3, 4), (3, 3)),
            ((2, 6), (3, 6)),
            ((3, 5), (3, 6)),
            ((4, 8), (3, 8)),
            ((5, 0), (4, 0)),
            ((4, 0), (4, 1)),
            ((5, 3), (4, 3)),
            ((4, 3), (4, 4)),
            ((4, 4), (4, 5)),
            ((3, 6), (4, 6)),
            ((5, 8), (4, 8)),
            ((5, 1), (5, 0)),
            ((4, 2), (5, 2)),
            ((6, 7), (5, 7)),
            ((5, 7), (5, 8)),
            ((6, 1), (6, 0)),
            ((6, 2), (6, 1)),
            ((6, 5), (6, 6)),
            ((6, 6), (6, 7)),
            ((6, 0), (7, 0)),
            ((6, 3), (7, 3)),
            ((7, 2), (7, 3)),
            ((8, 4), (7, 4)),
            ((7, 6), (7, 5)),
            ((8, 2), (8, 1)),
            ((8, 3), (8, 2)),
            ((8, 6), (8, 5)),
            ((7, 8), (8, 8)),
        ];
        for (l, h) in pairs {
            cons.push(Box::new(LessThanConstraint { lpos: l, hpos: h }));
        }
    }

    let givens = vec![(0, 2, 4), (0, 5, 7), (1, 2, 0), (7, 7, 6), (8, 7, 8)];
    for i in givens {
        cons.push(Box::new(GivenConstraint { pos: i }));
    }

    let mut game = Puzzle::init(size);
    game.constraints = cons;

    while game.solve() {

        //println!("{}", game.board);
        //println!("{:?}", game.board);
    }

    println!("{}", game.board);
    println!("{:?}", game.board);
    println!("document.getElementById(\"puzzleForm\").onsubmit = function() {{Game.saveState();Game.tickTimer();this.jstimerPersonal.value = Game.getTimer();this.ansH.value=\"{}\"}};\ndocument.getElementById(\"btnReady\").click();", game.board.serialize().unwrap());
}
