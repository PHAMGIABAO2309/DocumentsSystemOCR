pub const SQL_TIM_THEO_NAM: &str = " 
SELECT f.FileCode, f.Title, f.StartDate, f.dateupdate, doc.FileCatalog
FROM files f
JOIN infomation_documents_out doc  ON f.FileCode = doc.FileCode
WHERE doc.FileCatalog LIKE ?
";