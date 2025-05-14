interface instanciaPartitura {
  id: number;
  name: string;
  image_path: string;
  music_xml_path: string | null;
  midi_path: string | null;
}

export type { instanciaPartitura };
