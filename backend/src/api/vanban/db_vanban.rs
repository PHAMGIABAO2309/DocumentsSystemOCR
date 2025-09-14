pub const SQL_VAN_BAN: &str = "
SELECT f.FileCode, CodeNumber, FileCatalog, Receives, ido.Subject, ValidityStatus, Title, f.StartDate, de.SubjectEN, f.FileNoNation
FROM infomation_documents_out ido, files f, documents_eng de
WHERE ido.FileCode = f.FileCode
AND ido.InfoId = de.InfoId
AND f.FileCode = ?;";