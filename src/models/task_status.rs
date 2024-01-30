use super::DbObj;
use rusqlite::{Error, Row};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TaskStatus {
    pub id: i32,
    pub name: String,
}

impl Default for TaskStatus {
    fn default() -> TaskStatus {
        TaskStatus::new(0, "".to_string())
    }
}
impl DbObj for TaskStatus {
    fn fields() -> Vec<String> {
        vec!["id".to_string(), "name".to_string()]
    }

    fn get_id(&mut self) -> i32 {
        self.id
    }

    fn table_name() -> String {
        "task_status".to_string()
    }

    fn from_row(row: &Row) -> TaskStatus {
        TaskStatus::new(row.get_unwrap(0), row.get_unwrap(1))
    }

    fn to_hashmap(&self) -> HashMap<String, String> {
        let mut rv: HashMap<String, String> = HashMap::new();
        rv.insert("id".to_string(), self.id.to_string());
        rv.insert("name".to_string(), self.name.to_string());

        rv
    }
    fn from_hashmap(fieldmap: &mut HashMap<String, String>) -> Result<Self, Error> {
        let expected_fields = Self::fields();
        let existing_fields = expected_fields.iter().fold(0, |acc, item| -> usize {
            if fieldmap.contains_key(item) {
                return acc + 1;
            }
            acc
        });
        if expected_fields.len() == existing_fields {
            Ok(Self::new(
                fieldmap.get("id").unwrap().parse::<i32>().unwrap(),
                fieldmap.get("name").unwrap().to_string(),
            ))
        } else {
            Err(Error::InvalidQuery)
        }
    }
}

impl TaskStatus {
    pub fn new(iid: i32, name: String) -> TaskStatus {
        TaskStatus { id: iid, name }
    }
}
