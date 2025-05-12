import { Link } from "react-router-dom";
import "./TableRow.css";
import type { instanciaPartitura } from "../../data/entities_types/types";

/*
    Name    
    Image   
    Details 
    Actions 
*/

interface RowProps {
  instanceInfo: instanciaPartitura;
}

export const TableRow = ({ instanceInfo }: RowProps) => {
  return (
    <div className="tablerow_container">
      <div> {instanceInfo.name} </div>
      <div> {instanceInfo.image_path} </div>
      <div>
        {" "}
        <Link to={`${instanceInfo.id}`}> View and Download Output </Link>{" "}
      </div>
      <div>
        <button> Delete </button>
      </div>
    </div>
  );
};
