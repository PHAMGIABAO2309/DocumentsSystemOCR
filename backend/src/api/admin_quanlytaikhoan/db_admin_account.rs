pub const GET_ACCOUNT_SQL: &str = "SELECT ID, FullName, Password, Email, PhoneNumber 
FROM account
WHERE ID <> 'Admin';
";

pub const UPDATE_ACCOUNT_SQL: &str = "UPDATE account SET FullName = ?, Email = ?, PhoneNumber = ? WHERE ID = ?";

pub const DELETE_ACCOUNT_SQL: &str = "DELETE FROM account WHERE ID = ?";