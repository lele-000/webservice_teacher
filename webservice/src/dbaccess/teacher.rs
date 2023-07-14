use crate::error::MyError;
use crate::models::teacher::{self, *};
use sqlx::mysql::MySqlPool;

pub async fn get_all_teacher_db(pool: &MySqlPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!("SELECT * FROM teacher")
        .fetch_all(pool)
        .await?;

    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone().unwrap(),
            picture_url: r.picture_url.clone().unwrap(),
            profile: r.profile.clone().unwrap(),
        })
        .collect();

    match teachers.len() {
        0 => Err(MyError::NotFound("Not teachers found".into())),
        _ => Ok(teachers),
    }
}

pub async fn get_teacher_detail_db(pool: &MySqlPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query!("SELECT * FROM teacher WHERE id=?", teacher_id)
        .fetch_one(pool)
        .await
        .map(|r| Teacher {
            id: r.id,
            name: r.name.unwrap(),
            picture_url: r.picture_url.unwrap(),
            profile: r.profile.unwrap(),
        })
        .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;
    Ok(row)
}

pub async fn post_new_teacher_db(
    pool: &MySqlPool,
    new_teacher: CreateTeacher,
) -> Result<Teacher, MyError> {
    sqlx::query!(
        "INSERT INTO teacher (name,picture_url,profile)
        VALUE (?,?,?)",
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile,
    )
    .execute(pool)
    .await?;

    let row = sqlx::query!("SELECT * FROM teacher ORDER BY id DESC LIMIT 1")
        .fetch_one(pool)
        .await?;

    Ok(Teacher {
        id: row.id,
        name: row.name.unwrap(),
        picture_url: row.picture_url.unwrap(),
        profile: row.profile.unwrap(),
    })
}

pub async fn update_teacher_details_db(
    pool: &MySqlPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!("SELECT * FROM teacher WHERE id = ?", teacher_id)
        .fetch_one(pool)
        .await
        .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;

    let temp = Teacher {
        id: row.id,
        name: if let Some(name) = update_teacher.name {
            name
        } else {
            row.name.unwrap()
        },
        picture_url: if let Some(pic) = update_teacher.picture_url {
            pic
        } else {
            row.picture_url.unwrap()
        },
        profile: if let Some(profile) = update_teacher.profile {
            profile
        } else {
            row.profile.unwrap()
        },
    };

    sqlx::query!(
        "UPDATE teacher SET name=?,picture_url=?,profile=? WHERE id=?",
        temp.name,
        temp.picture_url,
        temp.profile,
        teacher_id
    )
    .execute(pool)
    .await?;
    let update_row = sqlx::query!("SELECT * FROM teacher WHERE id=?", teacher_id)
        .fetch_one(pool)
        .await
        .map(|r| Teacher {
            id: r.id,
            name: r.name.unwrap(),
            picture_url: r.picture_url.unwrap(),
            profile: r.profile.unwrap(),
        })
        .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;
    Ok(update_row)
}

pub async fn delete_teacher_db(pool: &MySqlPool, teacher_id: i32) -> Result<String, MyError> {
    let row = sqlx::query(&format!("DELETE FROM teacher WHERE id={}", teacher_id))
        .execute(pool)
        .await
        .map_err(|_err| MyError::DBError("Unable to delete teacher".into()))?;

    Ok(format!("Delete {:?} record", row))
}
