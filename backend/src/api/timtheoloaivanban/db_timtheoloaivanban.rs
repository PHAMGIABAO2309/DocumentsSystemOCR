pub const SQL_TIM_THEO_LOAI_VAN_BAN: &str = " 
SELECT f.FileCode, f.Title, f.StartDate, f.dateupdate, t.TypeName
FROM files f
JOIN type_documents t ON f.TypeId = t.TypeId
WHERE t.TypeName LIKE  ?
";