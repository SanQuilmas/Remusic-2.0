import { Link } from "react-router-dom";
import "./TableRow.css";
import type { instanciaPartitura } from "../../data/entities_types/types";
import { MainAPI } from "../../data/api_endpoints/enpoints";

interface RowProps {
  instanceInfo: instanciaPartitura;
  onDelete: (id: number) => void;
}

export const TableRow = ({ instanceInfo, onDelete }: RowProps) => {
  const handleDeleteButtonClick = async () => {
    try {
      const response = await fetch(MainAPI + "/" + instanceInfo.id, {
        method: "DELETE",
      });
      if (response.ok) {
        alert("Deleted successfully!");
        onDelete(instanceInfo.id);
      } else {
        const errorText = await response.text();
        alert("Delete failed: " + errorText);
      }
    } catch (err) {
      alert("Network error: " + err);
    }
  };

  return (
    <div className="tablerow_container">
      <div> {instanceInfo.name} </div>
      <div>
        <img src={instanceInfo.image_path} width={100} height={100} />
      </div>
      <div>
        {" "}
        <Link to={`${instanceInfo.id}`}> View and Download Output </Link>{" "}
      </div>
      <div>
        <button onClick={handleDeleteButtonClick}> Delete </button>
      </div>
    </div>
  );
};
