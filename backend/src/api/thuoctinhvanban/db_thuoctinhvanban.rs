pub const GET_TYPE_DOCUMENTS_SQL: &str = r#"
    SELECT TypeId as type_id, TypeName as type_name FROM type_documents
"#;

pub const GET_ORGANIZATION_SQL: &str = r#"
    SELECT OranId as oran_id, OranName as oran_name FROM organization 
"#;

pub const GET_POSITION_SQL: &str = r#"
    SELECT PosId as pos_id, SingerInfo as singer_info FROM positions 
"#;

pub const GET_FIELD_SQL: &str = r#"
 SELECT field_name, MIN(field_code) AS field_code
FROM (
    SELECT 
        TRIM(SUBSTRING_INDEX(FieldName, ',', 1)) AS field_name,
        TRIM(SUBSTRING_INDEX(FieldCode, ',', 1)) AS field_code
    FROM field

    UNION

    SELECT 
        TRIM(SUBSTRING_INDEX(SUBSTRING_INDEX(FieldName, ',', 2), ',', -1)) AS field_name,
        TRIM(SUBSTRING_INDEX(SUBSTRING_INDEX(FieldCode, ',', 2), ',', -1)) AS field_code
    FROM field
    WHERE FieldName LIKE '%,%'

    UNION

    SELECT 
        TRIM(SUBSTRING_INDEX(SUBSTRING_INDEX(FieldName, ',', 3), ',', -1)) AS field_name,
        TRIM(SUBSTRING_INDEX(SUBSTRING_INDEX(FieldCode, ',', 3), ',', -1)) AS field_code
    FROM field
    WHERE FieldName LIKE '%,%,%'
) AS combined_fields
GROUP BY field_name



"#;