pub mod handler;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::handler::Handler;

    use super::*;

    #[test]
    fn it_works() {
        if let Err(e) = Handler::new(handler::Storage::File(String::from("./test.db"))) {
            println!("{:?}", e);
        }
    }

    #[test]
    fn register() {
        let handler = Handler::new(handler::Storage::Memory).unwrap();
        let res = handler.register(String::from("user@name.com"), String::from("AnSecretPassword"));
        assert!(res.is_ok());
    }

    #[test]
    fn verify() {
        let handler = Handler::new(handler::Storage::Memory).unwrap();
        let ret = handler.verify(String::from("user@name.com"), String::from("AnSecretPassword"));
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), false);
        let _ = handler.register(String::from("user@name.com"), String::from("AnSecretPassword"));
        let ret = handler.verify(String::from("user@name.com"), String::from("AnSecretPassword"));
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), true);
    }

    #[test]
    fn verify_reject() {
        let handler = Handler::new(handler::Storage::Memory).unwrap();
        let ret = handler.verify(String::from("user@name.com"), String::from("AnSecretPassword"));
        println!("reject ret:{:?}", ret);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), false);
        let _ = handler.register(String::from("user@name.com"), String::from("AnSecretPassword"));
        let ret = handler.verify(String::from("user@name.com"), String::from("AnSecretPassword"));
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), true);
        let ret = handler.verify(String::from("user@name.com"), String::from("NotThePassword"));
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), false);
        let ret = handler.verify(String::from("wronguser&name.com"), String::from("AnSecretPassword"));
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), false);
    }

    #[test]
    fn verify_star() {
        let handler = Handler::new(handler::Storage::Memory).unwrap();
        let ret = handler.verify(String::from("user@name.com"), String::from("AnSecretPassword"));
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), false);
        let _ = handler.register(String::from("user@name.com"), String::from("AnSecretPassword"));
        let ret = handler.verify(String::from("user@name.com"), String::from("AnSecretPassword"));
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), true);
        let ret = handler.verify(String::from("*"), String::from("AnSecretPassword"));
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), false);
    }

    #[test]
    fn unregister() {
        let handler = Handler::new(handler::Storage::Memory).unwrap();
        let ret = handler.verify(String::from("user@name.com"), String::from("AnSecretPassword"));
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), false);
        let _ = handler.register(String::from("user@name.com"), String::from("AnSecretPassword"));
        let ret = handler.verify(String::from("user@name.com"), String::from("AnSecretPassword"));
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), true);
        let _ = handler.unregister(String::from("user@name.com"));
        let ret = handler.verify(String::from("user@name.com"), String::from("AnSecretPassword"));
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), false);
    }
}
