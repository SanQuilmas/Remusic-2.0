use crate::entities::sheet_instance_entity::SheetInstance;

pub fn find_all() -> Vec<SheetInstance> {
    let mut sheet_list = Vec::new();

    let mut temp =
        SheetInstance::build_sheet_instance(1, "Vivaldi".to_owned(), "image_path".to_owned());
    temp._set_midi_path(String::from(
        "https://magenta.github.io/magenta-js/music/demos/melody.mid",
    ));
    temp._set_musicxml_path(String::from("/MozaVeilSample.xml"));
    sheet_list.push(temp);

    let mut temp =
        SheetInstance::build_sheet_instance(2, "Davinci".to_owned(), "image_path".to_owned());
    temp._set_midi_path(String::from(
        "https://magenta.github.io/magenta-js/music/demos/melody.mid",
    ));
    temp._set_musicxml_path(String::from("/MozaVeilSample.xml"));
    sheet_list.push(temp);

    let mut temp = SheetInstance::build_sheet_instance(3, "Yo".to_owned(), "image_path".to_owned());
    temp._set_midi_path(String::from(
        "https://magenta.github.io/magenta-js/music/demos/melody.mid",
    ));
    temp._set_musicxml_path(String::from("/MozaVeilSample.xml"));
    sheet_list.push(temp);

    sheet_list
}

pub fn find_by_id(id: i32) -> SheetInstance {
    let mut temp =
        SheetInstance::build_sheet_instance(id, "Yo".to_owned(), "image_path".to_owned());
    temp._set_midi_path(String::from(
        "https://magenta.github.io/magenta-js/music/demos/melody.mid",
    ));
    temp._set_musicxml_path(String::from("/MozaVeilSample.xml"));
    temp
}

pub fn create_instance(req_body: String) -> SheetInstance {
    serde_json::from_str(&req_body).expect("Failed to deserialize JSON")
}

pub fn put_instance(_id: i32, req_body: String) -> SheetInstance {
    serde_json::from_str(&req_body).expect("Failed to deserialize JSON")
}
