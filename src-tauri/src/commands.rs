use crate::db::establish_connection;
use crate::models::Assignment;
use diesel::prelude::*;

#[tauri::command]
pub fn get_all_assignments() -> Result<Vec<Assignment>, String> {
    use crate::schema::assignments::dsl::*;

    let conn = &mut establish_connection();
    
    assignments.load::<Assignment>(conn)
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub fn get_unsubmitted_assignments() -> Result<Vec<Assignment>, String> {
    use crate::schema::assignments::dsl::*;

    let conn = &mut establish_connection();

    let results = assignments
        .filter(submitted.eq(0))
        .load::<Assignment>(conn)
        .map_err(|err| err.to_string());

    Ok(results?)
}

#[tauri::command]
pub fn submit_assignment(ass_id: i32) -> Result<(), String> {
    use crate::schema::assignments::dsl::*;

    let conn = &mut establish_connection();

    diesel::update(assignments.find(ass_id))
        .set(submitted.eq(1))
        .execute(conn)
        .map(|_| ())
        .map_err(|err| err.to_string())
}