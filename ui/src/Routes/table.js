import {React, useState} from "react";
import ContainerView from "../Components/GoNewVersion";
import Board from '../Components/board';
import '../App.css';

function Table(){
    let size = 10;
  
    const [state, setState] = useState(false);
    const [board, setBoard] = useState(new Board(size))  
    return(
        <div className="container">
            <h1>Table</h1>
            <div style={{backgroundColor:"#aaa5"}}>
                    <ContainerView board={board} rerender={()=>{setState(!state)}}/>

            </div>
        </div>
    )


}
export default Table;