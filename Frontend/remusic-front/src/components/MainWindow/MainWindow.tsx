import { Outlet } from "react-router-dom";
import { Footer } from "../Footer/Footer";
import { HeroBanner } from "../HeroBanner/HeroBanner";
import { NavBar } from "../NavBar/NavBar";
import "./MainWindow.css";
import {
  createContext,
  useState,
  type Dispatch,
  type SetStateAction,
} from "react";
import type { instanciaPartitura } from "../../data/entities_types/types";

interface SheetListContextProp {
  sheetList: instanciaPartitura[];
  setSheetList: Dispatch<SetStateAction<instanciaPartitura[]>>;
}

export const SheetListContext = createContext<SheetListContextProp>({
  sheetList: [],
  setSheetList: () => {},
});

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
