pub const GET_LAST_FILECODE_SQL: &str = "SELECT FileCode 
FROM files 
ORDER BY CAST(SUBSTRING(FileCode, 3) AS UNSIGNED) DESC 
LIMIT 1;";

pub const GET_LAST_INFOID_SQL: &str = "SELECT InfoId 
FROM infomation_documents_out 
ORDER BY CAST(SUBSTRING(InfoId, 3) AS UNSIGNED) DESC 
LIMIT 1;";

pub const INSERT_FILES_SQL: &str = "INSERT INTO files (FileCode, Title, path) 
VALUES (?, ?, ?)";

pub const DELETE_FILES_SQL: &str = r#"DELETE FROM files WHERE FileCode =  ?"#;

pub const DELETE_IDO_SQL: &str = r#"DELETE FROM infomation_documents_out WHERE InfoId = ?"#;

pub const DELETE_DOCUMENTS_EN_SQL: &str = r#"DELETE FROM documents_eng WHERE InfoId = ?"#;

pub const INSERT_IDO_SQL: &str = "INSERT INTO infomation_documents_out (InfoId, FileCode) 
VALUES (?, ?)";

pub const INSERT_DOCUMENTS_EN_SQL: &str = "INSERT INTO documents_eng (LanId, InfoId) 
VALUES ('EN', ?)";

pub const UPDATE_DOCUMENTS_SQL: &str = r#"
UPDATE infomation_documents_out 
SET LanId= 'VN', TypeId= ?, Receives= ?, FileCode= ?, ValidityStatus= ? ,PosId= ?,FieldCode= ?
WHERE InfoId = ? 
"#;

pub const UPDATE_FILES_SQL: &str = r#"
UPDATE files 
SET Title= ?, StartDate= ?, OranId= ?,FileNoNation= ?,TypeId= ?, dateupdate= ?
WHERE FileCode = ?
"#;