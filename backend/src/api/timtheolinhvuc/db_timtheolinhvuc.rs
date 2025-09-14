pub const SQL_TIM_THEO_LINH_VUC: &str = "
SELECT f.FileCode, f.Title, f.StartDate, f.dateupdate, fi.FieldName
FROM files f
JOIN infomation_documents_out ido ON f.FileCode = ido.FileCode
JOIN field fi ON fi.FieldCode = ido.FieldCode
WHERE fi.FieldName LIKE  ? ";