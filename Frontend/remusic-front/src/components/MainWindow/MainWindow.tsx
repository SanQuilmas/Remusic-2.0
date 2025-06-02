import { useState } from "react";
import { Outlet } from "react-router-dom";
import { SheetListContext } from "../../data/contexts/SheetListContext";
import type { instanciaPartitura } from "../../data/entities_types/types";
import { Footer } from "../Footer/Footer";
import { HeroBanner } from "../HeroBanner/HeroBanner";
import { NavBar } from "../NavBar/NavBar";
import "./MainWindow.css";

export const MainWindow = () => {
  const [sheetList, setSheetList] = useState<instanciaPartitura[]>([]);
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
