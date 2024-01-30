use super::DbObj;
use super::{project::Project, task_status::TaskStatus};
use rusqlite::{Connection, Error, Row};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub struct Task {
    pub id: i32,
    pub project_id: i32,
    pub parent_id: i32,
    pub name: String,
    pub description: String,
    pub weight: i32,
    pub status: i32,
    pub created_by: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Task {
    pub fn new(
        id: i32,
        project_id: i32,
        parent_id: i32,
        name: String,
        description: String,
        weight: i32,
        status: i32,
        created_by: i32,
        created_at: i64,
        updated_at: i64,
    ) -> Task {
        Task {
            id,
            project_id,
            parent_id,
            name,
            description,
            weight,
            status,
            created_by,
            created_at,
            updated_at,
        }
    }

    pub fn get_status(&self, conn: &Connection) -> Option<TaskStatus> {
        match TaskStatus::get_by_id(conn, self.status) {
            Ok(status) => return Option::from(status),
            Err(_e) => return None,
        }
    }
    pub fn get_project(&self, conn: &Connection) -> Option<Project> {
        match Project::get_by_id(conn, self.project_id) {
            Ok(project) => return Option::from(project),
            Err(_e) => return None,
        }
    }
}

impl Default for Task {
    fn default() -> Task {
        Task::new(0, 0, 0, "".to_string(), "".to_string(), 0, 0, 0, 0, 0)
    }
}

impl DbObj for Task {
    fn table_name() -> String {
        "task".to_string()
    }
    fn get_id(&mut self) -> i32 {
        self.id
    }

    fn fields() -> Vec<String> {
        vec![
            "id".to_string(),
            "project_id".to_string(),
            "parent_id".to_string(),
            "name".to_string(),
            "description".to_string(),
            "weight".to_string(),
            "status".to_string(),
            "created_by".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }

    fn from_row(row: &Row) -> Task {
        Task {
            id: row.get_unwrap(0),
            project_id: row.get_unwrap(1),
            parent_id: row.get_unwrap(2),
            name: row.get_unwrap(3),
            description: row.get_unwrap(4),
            weight: row.get_unwrap(5),
            status: row.get_unwrap(6),
            created_by: row.get_unwrap(7),
            created_at: row.get_unwrap(8),
            updated_at: row.get_unwrap(9),
        }
    }

    fn to_hashmap(&self) -> HashMap<String, String> {
        let mut rv: HashMap<String, String> = HashMap::new();
        rv.insert("id".to_string(), self.id.to_string());
        rv.insert("project_id".to_string(), self.project_id.to_string());
        rv.insert("parent_id".to_string(), self.parent_id.to_string());
        rv.insert("name".to_string(), self.name.to_string());
        rv.insert("description".to_string(), self.description.to_string());
        rv.insert("weight".to_string(), self.weight.to_string());
        rv.insert("status".to_string(), self.status.to_string());
        rv.insert("created_by".to_string(), self.created_by.to_string());
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
                fieldmap.get("project_id").unwrap().parse::<i32>().unwrap(),
                fieldmap.get("parent_id").unwrap().parse::<i32>().unwrap(),
                fieldmap.get("name").unwrap().to_string(),
                fieldmap.get("description").unwrap().to_string(),
                fieldmap.get("weight").unwrap().parse::<i32>().unwrap(),
                fieldmap.get("status").unwrap().parse::<i32>().unwrap(),
                fieldmap.get("created_by").unwrap().parse::<i32>().unwrap(),
                fieldmap.get("created_at").unwrap().parse::<i64>().unwrap(),
                fieldmap.get("updated_at").unwrap().parse::<i64>().unwrap(),
            ))
        } else {
            Err(Error::InvalidQuery)
        }
    }
}
