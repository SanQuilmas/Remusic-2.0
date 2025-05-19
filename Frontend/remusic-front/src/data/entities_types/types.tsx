interface instanciaPartitura {
  id: number;
  name: string;
  image_blob: string;
  music_xml_blob: string | null;
  midi_blob: string | null;

  image_path: string;
  music_xml_path: string | null;
  midi_path: string | null;
}

export type { instanciaPartitura };
