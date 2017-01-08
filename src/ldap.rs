use openldap::*;
use openldap::errors::*;

use config::LdapConfig;

pub struct LdapConn {
    config: LdapConfig,
    conn: RustLDAP,
}

impl LdapConn {
    pub fn from_config(config: LdapConfig) -> Result<Self, LDAPError> {
        let conn = try!(RustLDAP::new(config.uri.as_str()));
        try!(conn.simple_bind(config.user.as_str(), config.pass.as_str()));

        Ok(LdapConn {
            config: config,
            conn: conn,
        })
    }
}
