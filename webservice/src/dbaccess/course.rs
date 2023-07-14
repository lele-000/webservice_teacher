use crate::error::MyError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use sqlx::mysql::MySqlPool;

pub async fn get_course_for_teacher_db(
    pool: &MySqlPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE teacher_id =?"#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_course_detail_db(
    pool: &MySqlPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"SELECT * 
        FROM course 
        WHERE teacher_id = ? and id = ?"#,
        teacher_id,
        course_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(
    pool: &MySqlPool,
    new_course: CreateCourse,
) -> Result<Course, MyError> {
    sqlx::query_as!(
        Course,
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
        VALUES(?,?,?,?,?,?,?,?,?)"#,
        // new_course.id.unwrap() as u64,
        new_course.teacher_id as i32,new_course.name,new_course.description,
        new_course.format,new_course.structure,new_course.duration,
        new_course.price,new_course.language,new_course.level
    )
    .execute(pool)
    .await?;

    // let last_insert_id_query = sqlx::query!(r#"SELECT LAST_INSERT_ID() AS id"#)
    //     .fetch_one(pool)
    //     .await
    //     .unwrap();
    // println!("{:?}", &last_insert_id_query);
    // let last_insert_id = last_insert_id_query.id;
    // println!("{}", last_insert_id);
    // let row = sqlx::query!(
    //     r#"SELECT id, teacher_id, name, time
    //        FROM course
    //        WHERE id = ?"#,
    //     last_insert_id
    // )
    // .fetch_one(pool)
    // .await
    // .unwrap();
    let row = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course
        ORDER BY id DESC LIMIT 1"#
    )
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn delete_course_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
) -> Result<String, MyError> {
    let course_row = sqlx::query!(
        "DELETE FROM course where teacher_id=? and id=?",
        teacher_id,
        id,
    )
    .execute(pool)
    .await;

    Ok(format!("Delete {:?} record", course_row))
}

pub async fn update_course_detail_db(
    pool: &MySqlPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse,
) -> Result<Course, MyError> {
    let current_course_row = sqlx::query_as!(
        Course,
        "SELECT * FROM course where teacher_id=? and id=?",
        teacher_id,
        id,
    )
    .fetch_one(pool)
    .await
    .map_err(|_| MyError::NotFound("Course id not found".into()))?;

    let name: String = if let Some(name) = update_course.name {
        name
    } else {
        current_course_row.name
    };
    let description: String = if let Some(desc) = update_course.description {
        desc
    } else {
        current_course_row.description.unwrap_or_default()
    };
    let format: String = if let Some(fm) = update_course.format {
        fm
    } else {
        current_course_row.format.unwrap_or_default()
    };
    let structure: String = if let Some(struc) = update_course.structure {
        struc
    } else {
        current_course_row.structure.unwrap_or_default()
    };
    let duration: String = if let Some(dur) = update_course.duration {
        dur
    } else {
        current_course_row.duration.unwrap_or_default()
    };
    let price: i32 = if let Some(price) = update_course.price {
        price
    } else {
        current_course_row.price.unwrap_or_default()
    };
    let language: String = if let Some(lang) = update_course.language {
        lang
    } else {
        current_course_row.language.unwrap_or_default()
    };
    let level: String = if let Some(level) = update_course.level {
        level
    } else {
        current_course_row.level.unwrap_or_default()
    };

    sqlx::query_as!(
        Course,
        "UPDATE course SET name=?, description=?, format=?, 
        structure=?, duration=?, price=?, language=?, level=? 
        WHERE teacher_id=? and id=?",
        name,
        description,
        format,
        structure,
        duration,
        price,
        language,
        level,
        teacher_id,
        id
    )
    .execute(pool)
    .await?;
    let course_row = sqlx::query_as!(
        Course,
        "SELECT * from course WHERE teacher_id=? and id=?",
        teacher_id,
        id
    )
    .fetch_one(pool)
    .await;
    if let Ok(course) = course_row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }
}
