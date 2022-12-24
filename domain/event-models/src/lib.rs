extern crate url;
extern crate uuid;
extern crate serde_cbor;

mod types;
mod api;
mod default;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
