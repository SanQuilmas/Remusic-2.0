interface instanciaPartitura {
  id: number;
  name: string;

  image_path: string;
  music_xml_path: string;
  midi_path: string;

  ERROR: boolean | null;
  DONE: boolean | null;
}

export type { instanciaPartitura };
