import { Link } from "react-router-dom";
import { MainAPI } from "../../data/api_endpoints/enpoints";
import type { instanciaPartitura } from "../../data/entities_types/types";
import "./TableRow.css";
import { useEffect, useState } from "react";

interface RowProps {
  instanceInfo: instanciaPartitura;
  onDelete: (id: number) => void;
}

export const TableRow = ({ instanceInfo, onDelete }: RowProps) => {
  const stored = sessionStorage.getItem(`progress_val_${instanceInfo.id}`);
  const [progress_val, setProgress_val] = useState<number>(
    stored ? JSON.parse(stored) : 0
  );

  useEffect(() => {
    if (instanceInfo.DONE || instanceInfo.ERROR) return;
    const interval = setInterval(() => {
      setProgress_val((prev) => prev + 1);
      sessionStorage.setItem(
        `progress_val_${instanceInfo.id}`,
        JSON.stringify(progress_val)
      );
      if (instanceInfo.CURRENT_PROGRESS && instanceInfo.CURRENT_PROGRESS > 100)
        return () => clearInterval(interval);
    }, 1250);
    return () => clearInterval(interval);
  }, [instanceInfo, progress_val]);

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
            <progress id="progress__content" max="100" value={progress_val} />
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
