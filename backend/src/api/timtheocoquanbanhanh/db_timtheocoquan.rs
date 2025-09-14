pub const SQL_TIM_THEO_CO_QUAN_BAN_HANH: &str = " 
SELECT f.FileCode, f.Title, f.StartDate, f.dateupdate, org.OranName
FROM files f
JOIN organization org ON f.OranId = org.OranId
WHERE org.OranName LIKE ? ";