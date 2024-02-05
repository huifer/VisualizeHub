use crypto::aes;
use crypto::aes::KeySize::KeySize256;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};
use crypto::symmetriccipher::SymmetricCipherError;
use base64;

pub trait Cipher {
    // Encrypt the given data
    fn encrypt(&self, data: String) -> String;

    // Decrypt the given ciphertext
    fn decrypt(&self, ciphertext: String) -> Option<String>;
}

pub struct AesCipher {
    key: [u8; 32],
    iv: [u8; 16],
}

impl AesCipher {
    pub fn new(key_string: String, iv_string: String) -> Self {
        let mut key = [0; 32];

        // Ensure the key string length is at most 32 bytes
        let key_bytes = key_string.as_bytes();
        let copy_len = key_bytes.len().min(key.len());

        key[..copy_len].copy_from_slice(&key_bytes[..copy_len]);


        let mut iv = [0; 16];

        // Ensure the IV string length is at most 16 bytes
        let iv_bytes = iv_string.as_bytes();
        let copy_len = iv_bytes.len().min(iv.len());

        iv[..copy_len].copy_from_slice(&iv_bytes[..copy_len]);

        AesCipher { key, iv }
    }
}

impl Cipher for AesCipher {
    #[warn(deprecated)]
    fn encrypt(&self, data: String) -> String {
        base64::encode(&aes256_cbc_encrypt(data.as_bytes(), &self.key, &self.iv).unwrap())
    }

    #[warn(deprecated)] fn decrypt(&self, ciphertext: String) -> Option<String> {
        let result = aes256_cbc_decrypt(&base64::decode(&ciphertext).unwrap(), &self.key, &self.iv)
            .map(|bytes| String::from_utf8_lossy(&bytes).to_string());

        // Optionally return the result
        result.ok()
    }
}


/// Encrypt a buffer with the given key and iv using AES256/CBC/Pkcs encryption.
fn aes256_cbc_encrypt(
    data: &[u8],
    key: &[u8; 32],
    iv: &[u8; 16],
) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(
        KeySize256,
        key,
        iv,
        PkcsPadding,
    );

    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut read_buffer = RefReadBuffer::new(data);
    let mut final_result = Vec::new();

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            _ => continue,
        }
    }

    Ok(final_result)
}

/// Decrypt a buffer with the given key and iv using AES256/CBC/Pkcs encryption.
fn aes256_cbc_decrypt(
    data: &[u8],
    key: &[u8; 32],
    iv: &[u8; 16],
) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(
        KeySize256,
        key,
        iv,
        PkcsPadding,
    );

    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut read_buffer = RefReadBuffer::new(data);
    let mut final_result = Vec::new();

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            _ => continue,
        }
    }

    Ok(final_result)
}