import { createContext, type Dispatch, type SetStateAction } from "react";
import type { instanciaPartitura } from "../entities_types/types";

interface SheetListContextProp {
  sheetList: instanciaPartitura[];
  setSheetList: Dispatch<SetStateAction<instanciaPartitura[]>>;
}

const SheetListContext = createContext<SheetListContextProp>({
  sheetList: [],
  setSheetList: () => {},
});

export { SheetListContext };
