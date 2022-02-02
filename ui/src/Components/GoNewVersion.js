import { useState } from 'react';
import Board from "./board";
var GRID_SIZE = 40;

const BoardIntersection = (props) => {
    const handleClick = (e) => {
        e.preventDefault()
        if (props.board.play(props.row, props.col, props.rerender))
            props.onPlay();
    }

    var style = {
        top: props.row * GRID_SIZE,
        left: props.col * GRID_SIZE
    }
    var classes = "intersection"
    if (props.color != Board.EMPTY)
        classes += props.color == Board.BLACK ? " black" : " white";
    return (
        <div onClick={handleClick} 
            className={classes} style={style}></div>
    );

}

const BoardView = (props) => {

    var intersections = [];
    let k = 0;
    for (var i = 0; i < props.board.size; i++)
        for (var j = 0; j < props.board.size; j++)
            intersections.push(<BoardIntersection
                {...{
                    board: props.board,
                    color: props.board.board[i][j],
                    row: i,
                    col: j,
                    onPlay: props.onPlay,
                    key: k++,
                    rerender: props.rerender
                }}
            />);
    var style = {
        with: props.board.size * GRID_SIZE,
        height: props.board.size * GRID_SIZE
    };
    return <div style={style} id="board">{intersections}</div>

}

const AlertView = (props) => {
    var text = "";
    console.log(props);
    if (props.board.in_atari)
        text = "ATARI!";
    else if (props.board.attempted_suicide)
        text = "SUICIDE!";

    return (
        <div id="alerts">{text}</div>
    );
}

const PassView = (props) => {

    const handleClick = (e) => {
        props.board.pass();
    }
    return (
        <input id="pass-btn" type="button" value="Pass" onClick={handleClick} />
    )
}

//  var board = new Board(19);

const ContainerView = (props) => {
    const [board, setBoard] = useState(props.board);
    function getInitialState() {
        return { 'board': this.props.board };
    }
    function onBoardUpdate() {
        setBoard(props.board)
    }
    return (
        <div>
            <AlertView board={board} />
            <PassView board={board} />
            <BoardView board={board}
                onPlay={onBoardUpdate}
                rerender={props.rerender}
            />
        </div>
    )
}


export default ContainerView;
