use serde::Serialize;

#[derive(Serialize)]
pub struct SheetInstance {
    id: i32,
    name: String,
    image_path: String,
    musicxml_path: Option<String>,
    midi_path: Option<String>,
}

impl SheetInstance {
    pub fn build_sheet_instance(id: i32, name: String, image_path: String) -> SheetInstance {
        SheetInstance {
            id,
            name,
            image_path,
            musicxml_path: None,
            midi_path: None,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_image_path(&self) -> String {
        self.image_path.clone()
    }
    pub fn get_musicxml_path(&self) -> String {
        self.musicxml_path.clone().unwrap_or(String::from(""))
    }
    pub fn get_midi_path(&self) -> String {
        self.midi_path.clone().unwrap_or(String::from(""))
    }

    pub fn set_id(&mut self, new_id: i32) {
        self.id = new_id;
    }
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    pub fn set_image_path(&mut self, new_image_path: String) {
        self.image_path = new_image_path;
    }
    pub fn set_musicxml_path(&mut self, new_musicxml_path: String) {
        self.musicxml_path = Some(new_musicxml_path);
    }
    pub fn set_midi_path(&mut self, new_midi_path: String) {
        self.midi_path = Some(new_midi_path);
    }
}
