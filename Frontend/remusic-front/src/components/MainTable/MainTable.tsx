import { useCallback, useContext, useEffect } from "react";
import { KafkaAPI, MainAPI } from "../../data/api_endpoints/enpoints";
import { SheetListContext } from "../../data/contexts/SheetListContext";
import type { instanciaPartitura } from "../../data/entities_types/types";
import {
  DONE_TOPIC,
  ERROR_TOPIC,
} from "../../data/kafka_channels/kafka_channels";
import { sanitizePath } from "../../utilities/StaticURLSanitize";
import { TableRow } from "../TableRow/TableRow";
import "./MainTable.css";

export const MainTable = () => {
  const { sheetList, setSheetList } = useContext(SheetListContext);

  function updateSheetStatus(
    sheet_List: instanciaPartitura[],
    doneIds: number[],
    errorIds: number[]
  ) {
    return sheet_List.map((item) => ({
      ...item,
      DONE: doneIds.includes(item.id),
      ERROR: errorIds.includes(item.id),
    }));
  }

  const fetchAndSetStatuses = useCallback(async () => {
    const [doneRes, errorRes] = await Promise.all([
      fetch(KafkaAPI + "/" + DONE_TOPIC),
      fetch(KafkaAPI + "/" + ERROR_TOPIC),
    ]);
    const doneData = await doneRes.json();
    const errorData = await errorRes.json();

    const DONE_KEYS = Object.keys(doneData).map((entry) => Number(entry));
    const ERROR_KEYS = Object.keys(errorData).map((entry) => Number(entry));

    setSheetList((prevSheetList) =>
      updateSheetStatus(prevSheetList, DONE_KEYS, ERROR_KEYS)
    );
  }, [setSheetList]);

  const fetchData = useCallback(async () => {
    const response = await fetch(MainAPI);
    const data: instanciaPartitura[] = await response.json();
    const sanitizedData = data.map((item) => ({
      ...item,
      image_path: sanitizePath(item.image_path),
      music_xml_path: sanitizePath(item.music_xml_path),
      midi_path: sanitizePath(item.midi_path),
    }));
    setSheetList(sanitizedData);
  }, [setSheetList]);

  useEffect(() => {
    const fetchAll = async () => {
      await fetchData();
      await fetchAndSetStatuses();
    };
    fetchAll();
  }, [fetchAndSetStatuses, fetchData]);

  const handleDeleteRow = (id: number) => {
    setSheetList((prevList) => prevList.filter((item) => item.id !== id));
  };

  return (
    <div className="table_container">
      <div className="table_header">
        <div> Name </div>
        <div> Image </div>
        <div> Details </div>
        <div> Actions </div>
      </div>
      {sheetList.map((instance) => {
        return (
          <TableRow
            instanceInfo={instance}
            key={instance.id}
            onDelete={handleDeleteRow}
          />
        );
      })}
    </div>
  );
};
