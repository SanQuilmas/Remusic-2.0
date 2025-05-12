import { useContext, useEffect } from "react";
import { MainAPI } from "../../data/api_endpoints/enpoints";
import type { instanciaPartitura } from "../../data/entities_types/types";
import { SheetListContext } from "../MainWindow/MainWindow";
import { TableRow } from "../TableRow/TableRow";
import "./MainTable.css";

export const MainTable = () => {
  const { sheetList, setSheetList } = useContext(SheetListContext);

  const fetchData = async () => {
    const response = await fetch(MainAPI);
    const data: instanciaPartitura[] = await response.json();
    setSheetList(data);
  };

  useEffect(() => {
    fetchData();
  }, []);

  return (
    <div className="table_container">
      <div className="table_header">
        <div> Name </div>
        <div> Image </div>
        <div> Details </div>
        <div> Actions </div>
      </div>
      {sheetList.map((instance) => {
        return <TableRow instanceInfo={instance} key={instance.id} />;
      })}
    </div>
  );
};
