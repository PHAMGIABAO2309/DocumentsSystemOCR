pub const SQL_DANG_NHAP: &str = "
    SELECT ID, FullName, Password, Email, PhoneNumber FROM account WHERE (Email = ? OR PhoneNumber = ?) AND Password = ?
";