import { useEffect, useState } from "react";
import { Outlet } from "react-router-dom";
import { SheetListContext } from "../../data/contexts/SheetListContext";
import type { instanciaPartitura } from "../../data/entities_types/types";
import { Footer } from "../Footer/Footer";
import { HeroBanner } from "../HeroBanner/HeroBanner";
import { NavBar } from "../NavBar/NavBar";
import "./MainWindow.css";

export const MainWindow = () => {
  const stored = sessionStorage.getItem("sheetList");
  const [sheetList, setSheetList] = useState<instanciaPartitura[]>(
    stored ? JSON.parse(stored) : []
  );

  useEffect(() => {
    sessionStorage.setItem("sheetList", JSON.stringify(sheetList));
  }, [sheetList]);

  return (
    <SheetListContext.Provider value={{ sheetList, setSheetList }}>
      <div className="mainview_container">
        <div>
          <NavBar />
          <HeroBanner />
        </div>
        <Outlet />
        <div>
          <Footer />
        </div>
      </div>
    </SheetListContext.Provider>
  );
};
