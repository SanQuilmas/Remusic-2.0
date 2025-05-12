interface instanciaPartitura {
  id: number;
  name: string;
  image_path: string;
  musicxml_path: string | null;
  midi_path: string | null;
}

export type { instanciaPartitura };
