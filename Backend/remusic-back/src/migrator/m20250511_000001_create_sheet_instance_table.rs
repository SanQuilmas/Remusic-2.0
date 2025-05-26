use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20250511_000001_create_sheet_instance_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the SheetInstance table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SheetInstance::Table)
                    .col(
                        ColumnDef::new(SheetInstance::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SheetInstance::Name).string().not_null())
                    .col(ColumnDef::new(SheetInstance::ImagePath).string().null())
                    .col(ColumnDef::new(SheetInstance::MusicXmlPath).string().null())
                    .col(ColumnDef::new(SheetInstance::MidiPath).string().null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the SheetInstance table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SheetInstance::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum SheetInstance {
    Table,
    Id,
    Name,
    ImagePath,
    MusicXmlPath,
    MidiPath,
}
