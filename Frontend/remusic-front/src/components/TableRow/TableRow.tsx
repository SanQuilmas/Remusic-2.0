import { Link } from "react-router-dom";
import { MainAPI } from "../../data/api_endpoints/enpoints";
import type { instanciaPartitura } from "../../data/entities_types/types";
import "./TableRow.css";
import { PROGRESS_WHEEL } from "../../data/variables/env_variables";
import { useEffect, useState } from "react";

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

  const [progress_pos, setProgress_pos] = useState<number>(0);
  useEffect(() => {
    const interval = setInterval(() => {
      setProgress_pos((prev) => (prev + 1) % 4);
    }, 250);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="tablerow_container">
      <div> {instanceInfo.name} </div>
      <div>
        <img src={instanceInfo.image_path} width={100} height={100} />
      </div>
      <div>
        {" "}
        {instanceInfo.ERROR ? (
          <p className="error_parraf">
            ERROR: Operation failed — No output will be generated
          </p>
        ) : instanceInfo.DONE ? (
          <Link to={`/details/${instanceInfo.id}`}>
            {" "}
            View and Download Output{" "}
          </Link>
        ) : instanceInfo.IN_PROGRESS ? (
          <div className="progress__bar">
            <label htmlFor="progress__content">Currently in Progress </label>
            {/* <progress
              id="progress__content"
              max="100"
              value={instanceInfo.CURRENT_PROGRESS ?? 0}
            /> */}
            <p id="progress__content" className="progress__wheel">
              {PROGRESS_WHEEL[progress_pos]}
            </p>
          </div>
        ) : (
          <p>Loading...</p>
        )}{" "}
      </div>
      <div>
        <button onClick={handleDeleteButtonClick} className="delete__button">
          {" "}
          Delete{" "}
        </button>
      </div>
    </div>
  );
};
