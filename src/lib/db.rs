use crate::lib::db_id::DbId;
use std::path::Path;

pub struct Db {
    pub id: DbId,
    pub path: Path,
}
