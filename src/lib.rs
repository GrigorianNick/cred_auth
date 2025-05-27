pub mod handler;

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

    #[test]
    fn double_dbs()
    {
        let handler1 = Handler::new(handler::Storage::File(String::from("./dbldbl.sqlite")));
        assert!(handler1.is_ok());
        let handler2 = Handler::new(handler::Storage::File(String::from("./dbldbl.sqlite")));
        assert!(handler2.is_ok());
    }

    #[test]
    fn double_db_access()
    {
        let res1 = Handler::new(handler::Storage::File(String::from("./dbl.sqlite")));
        assert!(res1.is_ok());
        let handler1 = res1.unwrap();
        let res2 = Handler::new(handler::Storage::File(String::from("./dbl.sqlite")));
        assert!(res2.is_ok());
        let handler2 = res2.unwrap();

        let reg = handler1.register(String::from("user@name.com"), String::from("Hunter2"));
        assert!(reg.is_ok());
        let verify = handler2.verify(String::from("user@name.com"), String::from("Hunter2"));
        assert!(verify.is_ok());
        assert!(verify.unwrap());
    }
}
