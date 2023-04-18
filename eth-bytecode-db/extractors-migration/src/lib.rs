pub use sea_orm_migration::prelude::*;

mod m20221222_155714_create_table_bytecode_types;
mod m20221222_157903_fill_table_bytecode_types;
mod m20221222_159236_create_table_pending_addresses;
mod m20221222_162032_create_table_pending_tasks;
mod m20221222_163818_create_table_verified_contracts;
mod m20221222_164114_create_table_failures;
mod m20221222_167826_create_table_metadata;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221222_155714_create_table_bytecode_types::Migration),
            Box::new(m20221222_157903_fill_table_bytecode_types::Migration),
            Box::new(m20221222_159236_create_table_pending_addresses::Migration),
            Box::new(m20221222_162032_create_table_pending_tasks::Migration),
            Box::new(m20221222_163818_create_table_verified_contracts::Migration),
            Box::new(m20221222_164114_create_table_failures::Migration),
            Box::new(m20221222_167826_create_table_metadata::Migration),
        ]
    }
}