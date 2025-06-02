import { createContext } from "react";
import type { instanciaPartitura } from "../entities_types/types";

interface instanceContextType {
  instance: instanciaPartitura | null;
}

const InstanceContext = createContext<instanceContextType>({
  instance: null,
});

export { InstanceContext };
