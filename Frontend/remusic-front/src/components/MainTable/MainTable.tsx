import { TableRow } from "../TableRow/TableRow"
import "./MainTable.css"

export const MainTable = () => {
    const rowNum: Number = 5;
    return(
        <div className="table_container">
            <div className="table_header">
                <div> Name       </div>
                <div> Image      </div>
                <div> Details    </div>
                <div> Actions    </div>
            </div>
            {[...Array(rowNum).keys()].map(() =>{
                 return <TableRow />
            })}
        </div>
    )
}