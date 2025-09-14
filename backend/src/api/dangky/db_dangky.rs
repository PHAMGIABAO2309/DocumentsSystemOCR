pub const GET_LAST_ACCOUNT_SQL: &str = "SELECT ID FROM account ORDER BY ID DESC LIMIT 1";

pub const INSERT_ACCOUNT_SQL: &str = "INSERT INTO account(ID, FullName, Password, Email, PhoneNumber) 
VALUES (?, ?, ?, ?, ?)";