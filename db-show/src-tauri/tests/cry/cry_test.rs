use base64;
use crypto::buffer::{ReadBuffer, WriteBuffer};
use rand::RngCore;

#[cfg(test)]
mod tests {
    use db_show::cry::aes::{AesCipher, Cipher};

    #[test]
    fn main2() {
        let cipher = AesCipher::new("aaa".to_string(), "bbb".to_string());
        let enc = cipher.encrypt("hello,word".to_string());
        println!("{}", enc);
        ;
        let option = cipher.decrypt(enc);
        dbg!(option);
    }
}
