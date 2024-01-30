use super::DbObj;
use rusqlite::{Error, Row};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Project {
    pub id: i32,
    pub reference: String,
    pub name: String,
    pub description: String,
    pub created_by: i32,
    pub start_date: String,
    pub end_date: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Default for Project {
    fn default() -> Project {
        Project::new(
            0,
            String::from(""),
            String::from(""),
            String::from(""),
            0,
            String::from(""),
            String::from(""),
            0,
            0,
        )
    }
}
impl Project {
    pub fn new(
        id: i32,
        reference: String,
        name: String,
        description: String,
        created_by: i32,
        start_date: String,
        end_date: String,
        created_at: i64,
        updated_at: i64,
    ) -> Project {
        Project {
            id,
            reference,
            name,
            description,
            created_by,
            start_date,
            end_date,
            created_at,
            updated_at,
        }
    }

    pub fn tasks(&mut self) {}
}

impl DbObj for Project {
    fn table_name() -> String {
        "project".to_string()
    }
    fn get_id(&mut self) -> i32 {
        self.id
    }
    fn fields() -> Vec<String> {
        vec![
            "id".to_string(),
            "reference".to_string(),
            "name".to_string(),
            "description".to_string(),
            "created_by".to_string(),
            "start_date".to_string(),
            "end_date".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }
    fn from_row(row: &Row) -> Project {
        Project {
            id: row.get_unwrap(0),
            reference: row.get_unwrap(1),
            name: row.get_unwrap(2),
            description: row.get_unwrap(3),
            created_by: row.get_unwrap(4),
            start_date: row.get_unwrap(5),
            end_date: row.get_unwrap(6),
            created_at: row.get_unwrap(7),
            updated_at: row.get_unwrap(8),
        }
    }
    fn to_hashmap(&self) -> HashMap<String, String> {
        let mut rv: HashMap<String, String> = HashMap::new();
        rv.insert("id".to_string(), self.id.to_string());
        rv.insert("reference".to_string(), self.reference.to_string());
        rv.insert("name".to_string(), self.name.to_string());
        rv.insert("description".to_string(), self.description.to_string());
        rv.insert("created_by".to_string(), self.created_by.to_string());
        rv.insert("start_date".to_string(), self.start_date.to_string());
        rv.insert("end_date".to_string(), self.end_date.to_string());
        rv.insert("created_at".to_string(), self.created_at.to_string());
        rv.insert("updated_at".to_string(), self.updated_at.to_string());

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
                fieldmap.get("reference").unwrap().to_string(),
                fieldmap.get("name").unwrap().to_string(),
                fieldmap.get("description").unwrap().to_string(),
                fieldmap.get("created_by").unwrap().parse::<i32>().unwrap(),
                fieldmap.get("start_date").unwrap().to_string(),
                fieldmap.get("end_date").unwrap().to_string(),
                fieldmap.get("created_at").unwrap().parse::<i64>().unwrap(),
                fieldmap.get("updated_at").unwrap().parse::<i64>().unwrap(),
            ))
        } else {
            Err(Error::InvalidQuery)
        }
    }
}
