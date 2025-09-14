use actix_web::{post, web, Responder, HttpResponse};
use sqlx::{MySql, Pool, Row};
use serde::Deserialize;
use chrono::NaiveDate;

pub const GET_LAST_FILECODE_SQL: &str = r#"
    SELECT FileCode 
    FROM files 
    ORDER BY CAST(SUBSTRING(FileCode, 3) AS UNSIGNED) DESC 
    LIMIT 1;
"#;

pub const GET_LAST_INFOID_SQL: &str = "SELECT InfoId 
FROM infomation_documents_out 
ORDER BY CAST(SUBSTRING(InfoId, 3) AS UNSIGNED) DESC 
LIMIT 1;";

#[derive(Deserialize)]
pub struct RequestBody {
    pub oran_name: Option<String>,
    pub type_name: Option<String>,
    pub file_nonation: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub subject: Option<String>,
}

#[post("/api/admin/quanlyvanbanocr")]
pub async fn post_admin_quanlyvanbanocr(
    pool: web::Data<Pool<MySql>>, 
    params: web::Json<RequestBody>, 
) -> impl Responder {
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("DB transaction begin error: {}", e));
        }
    };

    // 1. Tìm OranId theo OranName
let oran_id: Option<String> = match sqlx::query("SELECT OranId FROM organization WHERE OranName = ?")
    .bind(&params.oran_name)
    .fetch_optional(&mut *tx)
    .await
{
    Ok(opt) => opt.map(|row| row.get::<String, _>("OranId")),
    Err(e) => {
        return HttpResponse::InternalServerError()
            .body(format!("Select error: {}", e));
    }
};

let oran_id = match oran_id {
    Some(id) => id,
    None => {
        // 1a. Lấy OranId cuối cùng để sinh ID mới
        let last_oranid: Option<String> = match sqlx::query(
            "SELECT OranId FROM organization 
             ORDER BY CAST(SUBSTRING(OranId, 3) AS UNSIGNED) DESC 
             LIMIT 1"
        )
        .fetch_optional(&mut *tx)
        .await
        {
            Ok(opt) => opt.map(|row| row.get::<String, _>("OranId")),
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Get last OranId error: {}", e));
            }
        };

        let new_oranid = match last_oranid {
            Some(code) => {
                let prefix = &code[..2]; // "OR"
                let num: i32 = code[2..].parse().unwrap_or(0);
                format!("{}{}", prefix, num + 1)
            }
            None => "OR1".to_string(),
        };

        // 1b. Insert với OranId mới
        if let Err(e) = sqlx::query("INSERT INTO organization (OranId, OranName) VALUES (?, ?)")
            .bind(&new_oranid)
            .bind(&params.oran_name)
            .execute(&mut *tx)
            .await
        {
            return HttpResponse::InternalServerError()
                .body(format!("Insert organization error: {}", e));
        }

        new_oranid
    }
};

    // 1. Tìm TypeId theo TypeName
let type_id: Option<String> = match sqlx::query("SELECT TypeId FROM type_documents WHERE TypeName = ?")
    .bind(&params.type_name)
    .fetch_optional(&mut *tx)
    .await
{
    Ok(opt) => opt.map(|row| row.get::<String, _>("TypeId")),
    Err(e) => {
        return HttpResponse::InternalServerError()
            .body(format!("Select error: {}", e));
    }
};

let type_id = match type_id {
    Some(id) => id,
    None => {
        // 1a. Lấy OranId cuối cùng để sinh ID mới
        let last_typeid: Option<String> = match sqlx::query(
            "SELECT TypeId FROM type_documents 
             ORDER BY CAST(SUBSTRING(TypeId, 3) AS UNSIGNED) DESC 
             LIMIT 1"
        )
        .fetch_optional(&mut *tx)
        .await
        {
            Ok(opt) => opt.map(|row| row.get::<String, _>("TypeId")),
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .body(format!("Get last OranId error: {}", e));
            }
        };

        let new_typeid = match last_typeid {
            Some(code) => {
                let prefix = "TL"; // "OR"
                let num: i32 = code[2..].parse().unwrap_or(0);
                format!("{}{}", prefix, num + 1)
            }
            None => "TL1".to_string(),
        };

        // 1b. Insert với OranId mới
        if let Err(e) = sqlx::query("INSERT INTO type_documents (TypeId, TypeName) VALUES (?, ?)")
            .bind(&new_typeid)
            .bind(&params.type_name)
            .execute(&mut *tx)
            .await
        {
            return HttpResponse::InternalServerError()
                .body(format!("Insert organization error: {}", e));
        }

        new_typeid
    }
};


    // 3. Lấy FileCode mới
    let last_filecode: Option<String> = match sqlx::query(GET_LAST_FILECODE_SQL)
        .fetch_optional(&mut *tx)
        .await
    {
        Ok(opt) => opt.map(|row| row.get::<String, _>("FileCode")),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Get last FileCode error: {}", e));
        }
    };

    let new_filecode = match last_filecode {
        Some(code) => {
            let prefix = &code[..2]; // "HS"
            let num: i32 = code[2..].parse().unwrap_or(0);
            format!("{}{}", prefix, num + 1)
        }
        None => "HS1".to_string(), 
    };

    // 3. Lấy InfoId mới
    let last_infoid: Option<String> = match sqlx::query(GET_LAST_INFOID_SQL)
        .fetch_optional(&mut *tx)
        .await
    {
        Ok(opt) => opt.map(|row| row.get::<String, _>("InfoId")),
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Get last FileCode error: {}", e));
        }
    };

   let new_infoid = match last_infoid {
    Some(code) => {
        let prefix = &code[..2]; // "TT"
        let num: i32 = code[2..].parse().unwrap_or(0);
        format!("{}{}", prefix, num + 1)
    }
    None => "TT1".to_string(), 
};

    // 4. Insert vào FILES
    if let Err(e) = sqlx::query("INSERT INTO files (FileCode, OranId, TypeId, FileNoNation, StartDate) VALUES (?, ?, ?, ?, ?)")
        .bind(&new_filecode)
        .bind(&oran_id)
        .bind(&type_id)
        .bind(&params.file_nonation)
        .bind(&params.start_date)
        .execute(&mut *tx)
        .await
    {
        return HttpResponse::InternalServerError()
            .body(format!("Insert file error: {}", e));
    }
    // 5. Insert vào INFO (dùng FileCode làm khóa ngoại)
if let Err(e) = sqlx::query("INSERT INTO infomation_documents_out (InfoId, FileCode, Subject) VALUES (?, ?, ?)")
.bind(&new_infoid)
.bind(&new_filecode)
.bind(&params.subject)
.execute(&mut *tx)
.await
{
    return HttpResponse::InternalServerError()
        .body(format!("Insert info error: {}", e));
}

    if let Err(e) = tx.commit().await {
        return HttpResponse::InternalServerError()
            .body(format!("Commit error: {}", e));
    }

    HttpResponse::Ok().body(format!("Thêm thành công với FileCode: {}", new_filecode))
}
