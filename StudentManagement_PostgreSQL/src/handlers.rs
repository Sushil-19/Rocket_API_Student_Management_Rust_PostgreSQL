use rocket::serde::json::Json;
use rocket::State;
use uuid::Uuid;
use diesel::prelude::*;
use crate::models::{CreateStudent, Student, UpdateStudent};
use crate::schema::students::dsl::*;
use crate::db::Pool;

#[get("/students")]
pub async fn get_students(pool: &State<Pool>) -> Json<Vec<Student>> {
    use crate::schema::students::dsl::*;
    let conn = pool.get().expect("Failed to get DB connection");
    let result = students.load::<Student>(&conn).expect("Error loading students");
    Json(result)
}

#[get("/students/<id>")]
pub async fn get_student_by_id(pool: &State<Pool>, id: Uuid) -> Option<Json<Student>> {
    let conn = pool.get().expect("Failed to get DB connection");
    let result = students.filter(id.eq(id))
        .first::<Student>(&conn)
        .ok()?;
    Some(Json(result))
}

#[post("/students", data = "<new_student>")]
pub async fn create_student(pool: &State<Pool>, new_student: Json<CreateStudent>) -> Json<Student> {
    let conn = pool.get().expect("Failed to get DB connection");
    let new_student = Student {
        id: Uuid::new_v4(),
        name: new_student.name.clone(),
        age: new_student.age,
        department: new_student.department.clone(),
    };
    diesel::insert_into(students)
        .values(&new_student)
        .execute(&conn)
        .expect("Error inserting new student");
    Json(new_student)
}

#[put("/students/<id>", data = "<updated_student>")]
pub async fn update_student(pool: &State<Pool>, id: Uuid, updated_student: Json<UpdateStudent>) -> Option<Json<Student>> {
    let conn = pool.get().expect("Failed to get DB connection");
    let target = students.filter(id.eq(id));
    diesel::update(target)
        .set(&*updated_student)
        .execute(&conn)
        .expect("Error updating student");
    let result = students.filter(id.eq(id))
        .first::<Student>(&conn)
        .ok()?;
    Some(Json(result))
}

#[delete("/students/<id>")]
pub async fn delete_student(pool: &State<Pool>, id: Uuid) -> Option<Json<()>> {
    let conn = pool.get().expect("Failed to get DB connection");
    let num_deleted = diesel::delete(students.filter(id.eq(id)))
        .execute(&conn)
        .expect("Error deleting student");
    if num_deleted == 1 {
        Some(Json(()))
    } else {
        None
    }
}
