import { useCallback, useContext, useEffect } from "react";
import { KafkaAPI, MainAPI } from "../../data/api_endpoints/enpoints";
import { SheetListContext } from "../../data/contexts/SheetListContext";
import type { instanciaPartitura } from "../../data/entities_types/types";
import {
  DONE_TOPIC,
  ERROR_TOPIC,
  INPROGRESS_TOPIC,
} from "../../data/kafka_channels/kafka_channels";
import { sanitizePath } from "../../utilities/StaticURLSanitize";
import { TableRow } from "../TableRow/TableRow";
import "./MainTable.css";
import { Link, useParams } from "react-router-dom";
import { PAGE_LIMIT } from "../../data/variables/env_variables";

export const MainTable = () => {
  const { sheetList, setSheetList } = useContext(SheetListContext);
  const { id } = useParams();

  // checks if the id param exists, if it exists it is assigned as current page. If it does not exist it defaults to one
  const page = id !== undefined ? Number(id) : 1;
  const pageCount = Math.ceil(sheetList.length / PAGE_LIMIT);

  function updateSheetStatus(
    sheet_List: instanciaPartitura[],
    doneIds: number[],
    errorIds: number[],
    progressIds: number[],
    progressVal: number[]
  ) {
    return sheet_List.map((item, index) => ({
      ...item,
      DONE: doneIds.includes(item.id),
      ERROR: errorIds.includes(item.id),
      IN_PROGRESS: progressIds.includes(item.id),
      CURRENT_PROGRESS: progressVal[index],
    }));
  }

  const fetchAndSetStatuses = useCallback(async () => {
    const [doneRes, errorRes, progressRes] = await Promise.all([
      fetch(KafkaAPI + "/" + DONE_TOPIC),
      fetch(KafkaAPI + "/" + ERROR_TOPIC),
      fetch(KafkaAPI + "/" + INPROGRESS_TOPIC),
    ]);
    const doneData = await doneRes.json();
    const errorData = await errorRes.json();
    const progressData = await progressRes.json();

    const DONE_KEYS = Object.keys(doneData).map((entry) => Number(entry));
    const ERROR_KEYS = Object.keys(errorData).map((entry) => Number(entry));
    const PROGRESS_KEYS = Object.keys(progressData).map((entry) =>
      Number(entry)
    );
    const PROGRESS_VALUES = Object.values(progressData).map((entry) =>
      Number(entry)
    );

    setSheetList((prevSheetList) => {
      const newSheetList = updateSheetStatus(
        prevSheetList,
        DONE_KEYS,
        ERROR_KEYS,
        PROGRESS_KEYS,
        PROGRESS_VALUES
      );
      if (JSON.stringify(prevSheetList) !== JSON.stringify(newSheetList)) {
        return newSheetList;
      }
      return prevSheetList;
    });
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
    if (sheetList.length === 0) return;

    const interval = setInterval(() => {
      fetchAndSetStatuses();
    }, 2000);

    return () => clearInterval(interval);
  }, [sheetList, fetchAndSetStatuses]);

  const handleDeleteRow = (id: number) => {
    setSheetList((prevList) => prevList.filter((item) => item.id !== id));
  };

  return (
    <div className="table_container">
      <div>
        <button onClick={fetchData}> Refresh </button>
      </div>
      <div className="page_numbers_container">
        {Array.from({ length: pageCount }).map((_entry, index) => {
          return (
            <span className="page_numbers" key={index + 1}>
              <Link to={`/gallery/${index + 1}`}>
                <p>{index + 1}</p>
              </Link>
            </span>
          );
        })}
      </div>
      <div className="table_header">
        <div> Name </div>
        <div> Image </div>
        <div> Details </div>
        <div> Actions </div>
      </div>
      <div className="table_rows">
        {sheetList.map((instance, index) => {
          if (
            index < page * PAGE_LIMIT &&
            index >= page * PAGE_LIMIT - PAGE_LIMIT
          ) {
            return (
              <TableRow
                instanceInfo={instance}
                key={instance.id}
                onDelete={handleDeleteRow}
              />
            );
          }
        })}
      </div>
    </div>
  );
};
