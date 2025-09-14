pub const SQL_DANH_SACH_VAN_BAN: &str = " 
SELECT FileCode, Title, StartDate, dateupdate, path 
FROM files 
ORDER BY CAST(SUBSTRING(FileCode, 3) AS UNSIGNED) DESC ";