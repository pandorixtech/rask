use rusqlite::{params, types::ValueRef, Connection, Error, Result, Row, ToSql};
use std::collections::HashMap;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
pub trait DbObj: Sized + Clone + std::fmt::Debug {
    fn fields() -> Vec<String>;
    fn table_name() -> String;
    fn from_row(row: &Row) -> Self;
    fn to_hashmap(&self) -> HashMap<String, String>;
    fn from_hashmap(fieldmap: &mut HashMap<String, String>) -> Result<Self, Error>;
    fn get_id(&mut self) -> i32;

    fn primary_key() -> String {
        "id".to_string()
    }

    fn get_current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    fn dictionary_from_row(row: &Row, field_names: &Vec<String>) -> HashMap<String, String> {
        let mut rv: HashMap<String, String> = HashMap::new();
        field_names.iter().enumerate().for_each(|(idx, field)| {
            let val = match row.get_ref_unwrap(idx) {
                ValueRef::Null => "".to_string(),
                ValueRef::Integer(i) => i.to_string(),
                ValueRef::Real(f) => f.to_string(),
                ValueRef::Text(t) => str::from_utf8(t).unwrap().to_string(),
                ValueRef::Blob(b) => format!("{:?}", b),
            };

            rv.insert(field.clone(), val);
        });
        rv
    }

    fn query(conn: &Connection, sql: String) -> Result<Vec<HashMap<String, String>>, Error> {
        match conn.prepare(&sql) {
            Ok(mut stmt) => {
                let fields: Vec<String> = stmt
                    .column_names()
                    .iter()
                    .map(|f| format!("{}", f))
                    .collect();

                match stmt.query_map(params![], |row: &Row| -> Result<HashMap<String, String>> {
                    Ok(Self::dictionary_from_row(row, &fields))
                }) {
                    Ok(rs_iter) => return rs_iter.collect(),
                    Err(e) => return Err(e),
                }
            }
            Err(e) => return Err(e),
        }
    }

    fn save(&self, conn: &Connection) -> Result<Self, Error> {
        let mut self_map: HashMap<String, String> = self.to_hashmap();
        let pkey = Self::primary_key();
        let fields = Self::fields();
        let id: String = self_map.get(&Self::primary_key()).unwrap().to_string();
        let mut f_str = "".to_string();
        let mut v_str = "".to_string();
        let mut pparams: Vec<&dyn ToSql> = Vec::new();
        //let mut pparams: Vec<&String> = Vec::new();
        let mut cc = 0;
        let mut current_timestamp: i64 = 0;
        if id.parse::<i32>().unwrap() < 1 {
            if self_map.contains_key(&"created_at".to_string()) {
                current_timestamp = Self::get_current_timestamp();
                self_map.insert(
                    "created_at".to_string(),
                    format!("{}", current_timestamp).to_string(),
                );
            }
            if self_map.contains_key(&"updated_at".to_string()) {
                self_map.insert(
                    "updated_at".to_string(),
                    format!("{}", current_timestamp).to_string(),
                );
            }

            for v in &fields {
                if pkey.trim() == v.trim() {
                    continue;
                }
                cc = cc + 1;
                if f_str.len() > 0 {
                    f_str.push_str(",");
                }
                if v_str.len() > 0 {
                    v_str.push_str(",");
                }
                //pparams.push(self_map.get(v).unwrap().to_sql().unwrap());
                f_str.push_str(v);
                //v_str.push_str(&format!("?{:}", cc));
                v_str.push_str("'");
                v_str.push_str(&self_map.get(v).unwrap().to_string());
                v_str.push_str("'");
            }
            let str_q = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                Self::table_name(),
                f_str,
                v_str
            );

            match conn.prepare(&str_q) {
                Ok(mut stmt) => match stmt.execute(params![]) {
                    Ok(rec) => return Self::get_by_id(conn, rec as i32),
                    Err(e) => return Err(e),
                },
                Err(e) => return Err(e),
            }
            /*
            match conn.execute(&str_q, &pparams) {
                Ok(rec) => return Self::get_by_id(conn, rec as i32),
                Err(e) => return Err(e),
            }
            */
        } else {
            if self_map.contains_key(&"updated_at".to_string()) {
                current_timestamp = Self::get_current_timestamp();
                //println!("updated timestamp {}", current_timestamp);
                self_map.insert(
                    "updated_at".to_string(),
                    format!("{}", current_timestamp).to_string(),
                );
            }

            for v in &fields {
                if id.trim() == v.trim() {
                    continue;
                }
                cc = cc + 1;
                if f_str.len() > 0 {
                    f_str.push_str(",");
                }
                pparams.push(self_map.get(v).unwrap());
                f_str.push_str(&format!("{}='{}'", v, self_map.get(v).unwrap().to_string()));
                //f_str.push_str(&format!("{}=?{}", v, cc));
            }
            //pparams.push(&id);
            let str_q = format!(
                "UPDATE {} SET {}  WHERE {}={}",
                Self::table_name(),
                f_str,
                pkey,
                id.to_string()
            );

            match conn.execute(&str_q, params![]) {
                Ok(_rec) => {
                    return Self::get_by_id(conn, id.parse::<i32>().unwrap());
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn del(&mut self, conn: &Connection) -> Result<(), Error> {
        let q = format!("DELETE FROM {} WHERE id=?", Self::table_name());
        match conn.prepare(q.as_str()) {
            Ok(mut stmt) => match stmt.execute(&[&self.get_id()]) {
                Ok(_i) => return Ok(()),
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }
    fn list(conn: &Connection, sql_filters: String) -> Result<Vec<Self>, Error> {
        let bq = format!("SELECT * FROM {}", Self::table_name());
        let qr = if sql_filters.len() > 0 {
            format!("{} WHERE {}", bq, sql_filters)
        } else {
            format!("{}", bq)
        };
        let mut stmt = conn.prepare(&qr)?;
        let rs_iter = stmt.query_map(params![], |row: &Row| -> Result<Self> {
            Ok(Self::from_row(row))
        })?;

        rs_iter.collect()
    }

    fn get_by_id(conn: &Connection, id: i32) -> Result<Self, Error> {
        let qr_filter = format!("{}={}", Self::primary_key(), id);
        match Self::list(conn, qr_filter) {
            Ok(vals) => {
                if vals.len() > 0 {
                    let rv = vals[0].clone();
                    Ok(rv)
                } else {
                    Err(Error::QueryReturnedNoRows)
                }
            }
            Err(e) => Err(e),
        }
    }
}

pub mod person;
pub mod project;
pub mod task;
pub mod task_status;
