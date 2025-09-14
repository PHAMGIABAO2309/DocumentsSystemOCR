pub const GET_TYPE_DOCUMENTS_SQL: &str = r#"
    SELECT TypeId as type_id, TypeName as type_name, OranId as oran_id FROM type_documents
"#;

pub const INSERT_TYPE_DOCUMENTS_SQL: &str = r#"INSERT INTO type_documents(TypeId, TypeName, OranId) 
VALUES (?, ?, ?)"#;

pub const CHECK_TYPE_DOCUMENTS_TYPE_ID_SQL: &str = r#"SELECT COUNT(*) FROM type_documents WHERE TypeId = ? "#;

pub const CHECK_TYPE_DOCUMENTS_TYPE_NAME_SQL: &str = r#"SELECT COUNT(*) FROM type_documents WHERE TypeName = ? "#;

pub const UPDATE_TYPE_DOCUMENTS_SQL: &str = r#"UPDATE type_documents SET TypeName= ?, OranId= ? WHERE TypeId= ? "#;

pub const DELETE_TYPE_DOCUMENTS_SQL: &str = r#"DELETE FROM type_documents WHERE TypeId = ?"#;