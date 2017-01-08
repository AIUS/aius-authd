use std::error;
use std::fmt;
use openldap;
use openldap::*;
use openldap::errors::*;

use config::LdapConfig;

#[derive(Debug)]
pub enum Error {
    LDAPError(LDAPError),
    InvalidCredentialsError,
    UnknownError(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::LDAPError(ref err) => write!(f, "LDAP error: {}", error::Error::description(err)),
            Error::InvalidCredentialsError => write!(f, "Invalid Credentials"),
            Error::UnknownError(code) => write!(f, "Unknown error: {}", code),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::LDAPError(ref e) => e.description(),
            Error::InvalidCredentialsError => "LDAP_INVALID_CREDENTIALS",
            Error::UnknownError(_) => "UnknownError",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::LDAPError(ref e) => Some(e),
            _ => None,
        }
    }
}

pub fn login(user: &str, pass: &str, config: LdapConfig) -> Result<(), Error> {
    let full_user = config.user_pattern.replace("%USER%", user);
    let conn = try!(RustLDAP::new(config.uri.as_str()).map_err(|e| Error::LDAPError(e)));
    conn.set_option(openldap::codes::options::LDAP_OPT_PROTOCOL_VERSION, &3);
    match try!(conn.simple_bind(full_user.as_str(), pass).map_err(|e| Error::LDAPError(e))) {
        v if v == openldap::codes::results::LDAP_SUCCESS => Ok(()),
        v if v == openldap::codes::results::LDAP_INVALID_CREDENTIALS => Err(Error::InvalidCredentialsError),
        v => Err(Error::UnknownError(v)),
    }
}
