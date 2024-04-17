use std::fmt::Debug;

#[derive(Clone, PartialEq, Eq)]
pub struct SqlServerAuth {
    user: String,
    password: String,
}

impl SqlServerAuth {
    pub(crate) fn user(&self) -> &str {
        &self.user
    }

    pub(crate) fn password(&self) -> &str {
        &self.password
    }
}

impl Debug for SqlServerAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqlServerAuth")
            .field("user", &self.user)
            .field("password", &"<HIDDEN>")
            .finish()
    }
}

/// Defines the method of authentication to the server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthMethod {
    /// Authenticate directly with SQL Server.
    SqlServer(SqlServerAuth),
    /// Authenticate with an AAD token. The token should encode an AAD user/service principal
    /// which has access to SQL Server.
    AADToken(String),
    #[doc(hidden)]
    None,
}

impl AuthMethod {
    /// Construct a new SQL Server authentication configuration.
    pub fn sql_server(user: impl ToString, password: impl ToString) -> Self {
        Self::SqlServer(SqlServerAuth {
            user: user.to_string(),
            password: password.to_string(),
        })
    }

    /// Construct a new configuration with AAD auth token.
    pub fn aad_token(token: impl ToString) -> Self {
        Self::AADToken(token.to_string())
    }
}
