use super::DbObj;
use rusqlite::{Error, Row};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

impl Default for Person {
    fn default() -> Person {
        Person::new(0, "".to_string(), "".to_string(), "".to_string())
    }
}
impl DbObj for Person {
    fn fields() -> Vec<String> {
        vec![
            "id".to_string(),
            "first_name".to_string(),
            "last_name".to_string(),
            "email".to_string(),
        ]
    }

    fn get_id(&mut self) -> i32 {
        self.id
    }

    fn table_name() -> String {
        "person".to_string()
    }

    fn from_row(row: &Row) -> Person {
        Person::new(
            row.get_unwrap(0),
            row.get_unwrap(1),
            row.get_unwrap(2),
            row.get_unwrap(3),
        )
    }

    fn to_hashmap(&self) -> HashMap<String, String> {
        let mut rv: HashMap<String, String> = HashMap::new();
        rv.insert("id".to_string(), self.id.to_string());
        rv.insert("first_name".to_string(), self.first_name.to_string());
        rv.insert("last_name".to_string(), self.last_name.to_string());
        rv.insert("email".to_string(), self.email.to_string());

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
                fieldmap.get("fname").unwrap().to_string(),
                fieldmap.get("lname").unwrap().to_string(),
                fieldmap.get("email").unwrap().to_string(),
            ))
        } else {
            Err(Error::InvalidQuery)
        }
    }
}

impl Person {
    pub fn new(iid: i32, fname: String, lname: String, semail: String) -> Person {
        Person {
            id: iid,
            first_name: fname,
            last_name: lname,
            email: semail,
        }
    }
}
