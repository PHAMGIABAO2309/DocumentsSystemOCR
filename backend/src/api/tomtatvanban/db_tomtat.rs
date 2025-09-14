pub const SQL_TOM_TAT: &str = "
SELECT 
    org.OranName, typeDoc.TypeName, 
    p.SingerInfo, 
    f.StartDate,  f.EndDate,  f.FileNoNation,  f.Title, 
    doc.PosId, 
    fi.FieldName
FROM 
    organization org
    JOIN Files f ON f.OranId = org.OranId
    JOIN infomation_documents_out doc ON doc.FileCode = f.FileCode
    JOIN type_documents typeDoc ON doc.TypeId = typeDoc.TypeId
    JOIN positions p ON  p.PosId = doc.PosId
    LEFT JOIN field fi ON doc.FieldCode = fi.FieldCode 
WHERE 
    doc.FileCode = ?";