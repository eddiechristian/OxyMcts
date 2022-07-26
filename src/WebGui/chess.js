//https://www.petercollingridge.co.uk/tutorials/svg/interactive/dragging/
console.clear();
const svg = document.querySelector("svg");
const svgns = "http://www.w3.org/2000/svg";
var selectedElement = false;
var offset;
var values_map = {};

let columns = 8;
let rows = 8;
let counter = 0;
let width =80;;
let height =80;

const colorArray = ["#774C3B","#C99468","#774C3B","#C99468","#774C3B","#C99468","#774C3B","#C99468",
                    "#C99468","#774C3B","#C99468","#774C3B","#C99468","#774C3B","#C99468","#774C3B"];

const fen_initial_state= "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0";

var starterPosition;
var current_fen_state = "rn1qk1nr/pb1p4/Bpp3p1/4Pp1Q/3bPB1P/N6R/PPP2PP1/3RK1N1 b kq - 5 25"

var from_spot;

getValidMoves(); //only call when turn changes

function get_player() {
  let player_char = current_fen_state.split(" ");
  switch (player_char[1]) {
    case "w": return "white";
    case "b": return "black";
  }
}

function start_game() {
  svg.innerHTML = ""
  start_player = get_player();
  
  for (let i = 0; i < rows; i++) {
      for (let j = 0; j < columns; j++) {
        square_id = getSquareId(j,i);
        counter++;
        let newRect = document.createElementNS(svgns, "rect");
        gsap.set(newRect, {
          attr: {
            x: j * width,
            y: i * height,
            width: width,
            height: height,
            fill: colorArray[(counter-1) % colorArray.length],
            id: square_id,
            class: "static",
          }
        });
          newRect.addEventListener('mouseleave', mouseLeave);
          newRect.addEventListener('mouseover', mouseOver);
          svg.appendChild(newRect);
      }
    }
    counter = 0
    for (let i = 0; i < rows; i++) {
      for (let j = 0; j < columns; j++) {
        square_id = getSquareId(j,i);
        counter++;
        let green_circle = document.createElementNS(svgns, "circle");
        gsap.set(green_circle, {
          attr: {
            cx: j * width + 40,
            cy: i * height + 40,
            r: 9,
            id:  "cir" + square_id,
            class: "valid_moves"
          },
        });
        svg.appendChild(green_circle);
        if (starterPosition[i][j] != '.'){
          piece = document.createElementNS(svgns, "image");
          piece_href = getPieceImageSource(starterPosition[i][j]);
          if (piece_href.includes(start_player) ){
            piece_class= "draggable";
          }else{
            piece_class= "static";
          }
          
          gsap.set(piece, {
                attr: { 
                    x: j * width  + 15 , 
                    y: i * height + 15,
                    href: piece_href, 
                    height: 45, 
                    width: 45,
                    class: piece_class,
                  }
          });
          piece.addEventListener('mousedown', startDrag);
          piece.addEventListener('mousemove', drag);
          piece.addEventListener('mouseup', endDrag);
          svg.appendChild(piece);
          
          
        } 
      }
    }


}


   async function getValidMoves() {
    var xhr = new XMLHttpRequest();
    var url = "http://localhost:9090/valid_moves";
    xhr.open("POST", url, true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.onreadystatechange = function () {
        if (xhr.readyState === 4 && xhr.status === 200) {
            get_valid_move_resp = JSON.parse(xhr.responseText);
            starterPosition = get_valid_move_resp.web_game.state;
            values_map = get_valid_move_resp.moves;
            start_game()
        }
    };
    var data = JSON.stringify({"fen_state": current_fen_state});
    xhr.send(data);
   }

   async function update_state(chess_move) {
    var xhr = new XMLHttpRequest();
    var url = "http://localhost:9090/move_req";
    xhr.open("POST", url, true);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.onreadystatechange = function () {
        if (xhr.readyState === 4 && xhr.status === 200) {
            get_resp = JSON.parse(xhr.responseText);
            starterPosition = get_resp.web_game.state;
            current_fen_state = get_resp.resulting_fen;
            values_map = get_resp.moves;
            console.log(current_fen_state)
            console.log(starterPosition)
            start_game()
        }
    };
    var data = JSON.stringify({"current_fen_state": current_fen_state, "chess_move":  chess_move});
    xhr.send(data);
   }

  function getSquareId(col_num, row_num) {
    col = '';
    row = '';
    switch (col_num) {
        case 0: col ='a';break;
        case 1: col ='b';break;
        case 2: col ='c';break;
        case 3: col ='d';break;
        case 4: col ='e';break;
        case 5: col ='f';break;
        case 6: col ='g';break;
        case 7: col ='h';break;
    }
    switch (row_num) {
        case 0: row ='8';break;
        case 1: row ='7';break;
        case 2: row ='6';break;
        case 3: row ='5';break;
        case 4: row ='4';break;
        case 5: row ='3';break;
        case 6: row ='2';break;
        case 7: row ='1';break;
    }
    return col + row;
  }

  function getPieceImageSource(piece) {
    switch (piece) {
        case 'r': return 'piece_images/black_rook.png';
        case 'n': return 'piece_images/black_knight.png';
        case 'b': return 'piece_images/black_bishop.png';
        case 'q': return 'piece_images/black_queen.png';
        case 'k': return 'piece_images/black_king.png';
        case 'p': return 'piece_images/black_pawn.png';
        case 'R': return 'piece_images/white_rook.png';
        case 'N': return 'piece_images/white_knight.png';
        case 'B': return 'piece_images/white_bishop.png';
        case 'Q': return 'piece_images/white_queen.png';
        case 'K': return 'piece_images/white_king.png';
        case 'P': return 'piece_images/white_pawn.png';
    }
}

function getMousePosition(evt) {
  var CTM = svg.getScreenCTM();
  return {
    x: (evt.clientX - CTM.e) / CTM.a,
    y: (evt.clientY - CTM.f) / CTM.d
  };
}

function get_spot_from_coords(x, y) {
  col_num = parseInt(x/width);
  row = 8 - parseInt(y/height);
 
  switch (col_num) {
    case 0: col = 'a';break;
    case 1: col = 'b';break;
    case 2: col = 'c';break;
    case 3: col = 'd';break;
    case 4: col = 'e';break;
    case 5: col = 'f';break;
    case 6: col = 'g';break;
    case 7: col = 'h';break;
  }
  return col + row
}

function startDrag(evt) {
  if (evt.target.classList.contains('draggable')) {
    selectedElement = evt.target;
    offset = getMousePosition(evt);
    from_spot = get_spot_from_coords(offset.x, offset.y)
    console.log(from_spot)
    offset.x -= parseFloat(selectedElement.getAttributeNS(null, "x"));
    offset.y -= parseFloat(selectedElement.getAttributeNS(null, "y"));
  }
}
function drag(evt) {
  if (selectedElement) {
    evt.preventDefault();
    var coord = getMousePosition(evt);
    selectedElement.setAttributeNS(null, "x", coord.x - offset.x);
    selectedElement.setAttributeNS(null, "y", coord.y -offset.y);
  }
}

function endDrag(evt) {
  if (selectedElement) {
    evt.preventDefault();
    var coord = getMousePosition(evt);
    x_adjusted = parseInt((coord.x / width)) * width + 15;
    y_adjusted = parseInt((coord.y / height)) * height + 15 ;
    selectedElement.setAttributeNS(null, "x", x_adjusted );
    selectedElement.setAttributeNS(null, "y", y_adjusted);
    to_spot = get_spot_from_coords(coord.x, coord.y)
    update_state(from_spot + to_spot)
  }
  selectedElement = null;
}

function mouseLeave(evt) {
  elements = document.getElementsByClassName("valid_moves");
  for (var i = 0; i < elements.length; i++) {
    elements[i].style.display = 'none';
  }
}

function show_green_circles(square) {
  let e = document.getElementById("cir" + square);
  e.style.display = 'block';
}


function hide_green_circles(square) {
 
  let e = document.getElementById("cir" + square);
  e.style.display = 'none';
}
function mouseOver(evt) {
  if (evt.target.id in values_map) {  
  let squares_to_make_visible = values_map[evt.target.id];
   squares_to_make_visible.forEach(show_green_circles);
  }
}
