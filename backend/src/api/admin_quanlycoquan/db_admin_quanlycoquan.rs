pub const GET_ORGANIZATION_SQL: &str = r#"SELECT OranId, OranName FROM organization"#;

pub const GET_LAST_ORGANIZATION_SQL: &str = r#"SELECT OranId FROM organization ORDER BY OranId DESC LIMIT 1"#;

pub const INSERT_ORGANIZATION_SQL: &str = r#"INSERT INTO organization (OranId, OranName) VALUES (?, ?)"#;

pub const UPDATE_ORGANIZATION_SQL: &str = r#"UPDATE organization SET OranName = ? WHERE OranId = ?"#;

pub const DELETE_ORGANIZATION_SQL: &str = r#"DELETE FROM organization WHERE OranId = ?"#;

pub const CHECK_ORGANIZATION_SQL: &str = r#"SELECT COUNT(*) FROM organization WHERE OranName = ?"#;