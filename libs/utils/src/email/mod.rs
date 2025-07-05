#[derive(Debug, Clone, PartialEq)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(email: &str) -> Result<Self, &str> {
        if email.contains('@') {
            Ok(EmailAddress(email.into()))
        } else {
            Err("Invalid email address")
        }
    }
}

impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for EmailAddress {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::From<&str> for EmailAddress {
    fn from(s: &str) -> EmailAddress {
        EmailAddress::new(s).unwrap()
    }
}

#[cfg(test)]
mod tests2 {
    use crate::email::EmailAddress;
    #[test]
    fn example() {
        let email: EmailAddress = "test@xample.com".into();
        println!("email:{}", email);
    }
}
