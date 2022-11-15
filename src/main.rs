use crate::board::{Puzzle, SdkStd};
use crate::board::SdkStd::False;
use crate::constraints::{CellConstraint, ColConstraint, Constraint, DigitConstraint, GivenConstraint, RowConstraint};

mod board;
mod constraints;

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


fn main() {
    let mut cons : Vec<Box<dyn Constraint<SdkStd>>> = vec![Box::new(RowConstraint), Box::new(ColConstraint), Box::new(DigitConstraint)];

    for x in 0..3 {
        for y in 0..3 {
            cons.push(Box::new(CellConstraint{cells : vec![
                (3*x,3*y),
                (3*x,3*y+1),
                (3*x,3*y+2),
                (3*x+1,3*y),
                (3*x+1,3*y+1),
                (3*x+1,3*y+2),
                (3*x+2,3*y),
                (3*x+2,3*y+1),
                (3*x+2,3*y+2),
            ]}));
        }
    }

    let cells = vec![
    ];
    for i in cells {
        cons.push(Box::new(CellConstraint{cells: i}));
    }

    let givens = vec![
        (0,0,6),
(0,1,4),
(0,2,3),
(0,5,7),
(0,7,2),
(1,0,7),
(1,1,2),
(1,2,5),
(2,0,0),
(2,1,8),
(2,2,1),
(2,3,2),
(2,4,4),
(2,6,7),
(2,7,3),
(3,0,1),
(3,1,3),
(3,2,4),
(3,3,8),
(4,3,6),
(4,4,3),
(4,5,4),
(5,0,8),
(5,5,2),
(5,6,5),
(5,7,4),
(5,8,3),
(6,1,1),
(6,2,8),
(6,3,7),
(6,4,0),
(7,6,8),
(7,7,0),
(8,1,0),
(8,3,4),
(8,8,7),
    ];
    for i in givens {
        cons.push(Box::new(GivenConstraint{pos: i}));
    }

    let mut game = Puzzle::init(9);
    game.constraints = cons;

    while game.solve() {

        //println!("{}", game.board);
        //println!("{:?}", game.board);
    }

    println!("{}", game.board);
    println!("{:?}", game.board);
    println!("document.getElementById(\"puzzleForm\").onsubmit = function() {{Game.saveState();Game.tickTimer();this.jstimerPersonal.value = Game.getTimer();this.ansH.value=\"{}\"}};\ndocument.getElementById(\"btnReady\").click();", game.board.serialize().unwrap());
}
