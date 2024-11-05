use std::{env, fs::create_dir_all, path::Path};

use redb::{Database, ReadableTable, TableDefinition};

use super::errors::{TagbaseError, TagbaseResult};
const FILENAME: &str = "tags.redb";
const TABLE: TableDefinition<&str, &str> = TableDefinition::new("tags");

pub(crate) struct Tagbase {
    db: Database,
}

impl Tagbase {
    pub(crate) fn try_new() -> TagbaseResult<Self> {
        let home_dir = env::var("HOME").map_err(|e| TagbaseError::Internal(e.to_string()))?;
        let jumptag_dir = Path::new(&home_dir).join(".jumptag");

        if !jumptag_dir.is_dir() {
            create_dir_all(&jumptag_dir).map_err(|e| TagbaseError::Internal(e.to_string()))?;
        }

        let tags_file = jumptag_dir.join(FILENAME);
        if !tags_file.is_file() {
            return Ok(Self {
                db: Database::create(tags_file)
                    .map_err(|e| TagbaseError::Internal(e.to_string()))?,
            });
        }

        Ok(Self {
            db: Database::open(tags_file).map_err(|e| TagbaseError::Internal(e.to_string()))?,
        })
    }

    pub(crate) fn set(&mut self, tag: &str, dir: &str) -> TagbaseResult<()> {
        let write_txn = self
            .db
            .begin_write()
            .map_err(|e| TagbaseError::Internal(e.to_string()))?;
        {
            let mut table = write_txn
                .open_table(TABLE)
                .map_err(|e| TagbaseError::Internal(e.to_string()))?;

            table
                .insert(tag, dir)
                .map_err(|e| TagbaseError::Internal(e.to_string()))?;
        }

        write_txn
            .commit()
            .map_err(|e| TagbaseError::Internal(e.to_string()))
    }

    pub(crate) fn delete(&mut self, tag: &str) -> TagbaseResult<()> {
        let write_txn = self
            .db
            .begin_write()
            .map_err(|e| TagbaseError::Internal(e.to_string()))?;
        {
            let mut table = write_txn
                .open_table(TABLE)
                .map_err(|e| TagbaseError::Internal(e.to_string()))?;

            table
                .remove(tag)
                .map_err(|e| TagbaseError::Internal(e.to_string()))?;
        }

        write_txn
            .commit()
            .map_err(|e| TagbaseError::Internal(e.to_string()))
    }

    pub(crate) fn get(&mut self, tag: &str) -> TagbaseResult<String> {
        let read_txn = self
            .db
            .begin_read()
            .map_err(|e| TagbaseError::Internal(e.to_string()))?;

        let table = read_txn.open_table(TABLE);

        let table = match table {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => {
                let write_txn = self
                    .db
                    .begin_write()
                    .map_err(|e| TagbaseError::Internal(e.to_string()))?;

                {
                    let _table = write_txn
                        .open_table(TABLE)
                        .map_err(|e| TagbaseError::Internal(e.to_string()))?;
                }

                write_txn
                    .commit()
                    .map_err(|e| TagbaseError::Internal(e.to_string()))?;

                // make new transaction here
                // since transaction is atomic and cannot see
                let read_txn = self
                    .db
                    .begin_read()
                    .map_err(|e| TagbaseError::Internal(e.to_string()))?;

                read_txn
                    .open_table(TABLE)
                    .map_err(|e| TagbaseError::Internal(e.to_string()))?
            }
            Err(e) => return Err(TagbaseError::Internal(e.to_string())),
        };

        let val_opt = table
            .get(tag)
            .map_err(|e| TagbaseError::Internal(e.to_string()))?;

        val_opt
            .ok_or(TagbaseError::TagNotFound(tag.to_string()))
            .map(|v| v.value().to_string())
    }

    pub(crate) fn list(&mut self) -> TagbaseResult<Vec<(String, String)>> {
        let read_txn = self
            .db
            .begin_read()
            .map_err(|e| TagbaseError::Internal(e.to_string()))?;

        let table = read_txn.open_table(TABLE);

        let table = match table {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => {
                let write_txn = self
                    .db
                    .begin_write()
                    .map_err(|e| TagbaseError::Internal(e.to_string()))?;

                {
                    let _table = write_txn
                        .open_table(TABLE)
                        .map_err(|e| TagbaseError::Internal(e.to_string()))?;
                }

                write_txn
                    .commit()
                    .map_err(|e| TagbaseError::Internal(e.to_string()))?;

                // make new transaction here
                // since transaction is atomic and cannot see
                let read_txn = self
                    .db
                    .begin_read()
                    .map_err(|e| TagbaseError::Internal(e.to_string()))?;

                read_txn
                    .open_table(TABLE)
                    .map_err(|e| TagbaseError::Internal(e.to_string()))?
            }
            Err(e) => return Err(TagbaseError::Internal(e.to_string())),
        };

        // collect from <iter-of-result> to <result-of-collection>
        // https://stackoverflow.com/questions/26368288
        table
            .iter()
            .map_err(|e| TagbaseError::Internal(e.to_string()))?
            .map(|kv| match kv {
                Ok((k, v)) => Ok((k.value().to_string(), v.value().to_string())),
                Err(e) => TagbaseResult::Err(TagbaseError::Internal(e.to_string())),
            })
            .collect()
    }
}
