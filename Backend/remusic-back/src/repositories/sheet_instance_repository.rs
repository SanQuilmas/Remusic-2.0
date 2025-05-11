use crate::entities::sheet_instance_entity::SheetInstance;

pub fn find_all() -> Vec<SheetInstance> {
    let mut sheet_list = Vec::new();

    sheet_list.push(SheetInstance::build_sheet_instance(
        1,
        "name".to_owned(),
        "image_path".to_owned(),
    ));

    sheet_list.push(SheetInstance::build_sheet_instance(
        2,
        "test".to_owned(),
        "image_path".to_owned(),
    ));

    sheet_list.push(SheetInstance::build_sheet_instance(
        3,
        "prueba".to_owned(),
        "image_path".to_owned(),
    ));

    sheet_list
}

pub fn find_by_id(id: i32) -> SheetInstance {
    let sheet: SheetInstance =
        SheetInstance::build_sheet_instance(id, "name".to_owned(), "image_path".to_owned());
    sheet
}
