import { Link } from "react-router-dom"
import "./TableRow.css"

/*
    Name    
    Image   
    Details 
    Actions 
*/

export const TableRow = () => {
    const selfID: Number = 123
    return(
        <div className="tablerow_container">
            <div> Name </div>
            <div> src/image </div>
            <div> <Link to={`${selfID}`}> View and Download Output </Link> </div>
            <div>
                <button> Delete </button>
            </div>
        </div>
    )
}