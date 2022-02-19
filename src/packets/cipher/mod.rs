use aes::Aes128;
use cfb8::cipher::AsyncStreamCipher;
use cfb8::Cfb8;
use std::io::{Read, Write};

pub type Cipher = Cfb8<Aes128>;

pub struct CipherWrapper<T> {
	inner: T,
	cipher: Cipher,
}

impl<T> CipherWrapper<T> {
	pub fn new(inner: T, cipher: Cipher) -> Self {
		Self { inner, cipher }
	}
}

impl<T: Read> Read for CipherWrapper<T> {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		let amount = self.inner.read(buf)?;
		self.cipher.decrypt(&mut buf[0..amount]);
		Ok(amount)
	}
}

impl<T: Write> Write for CipherWrapper<T> {
	fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
		let mut buf = [0u8; 1024];
		let amount_to_write = std::cmp::min(data.len(), buf.len());
		let data = &data[0..amount_to_write];
		let buf = &mut buf[0..amount_to_write];
		buf.copy_from_slice(data);
		self.cipher.encrypt(buf);
		self.inner.write_all(buf)?;
		Ok(amount_to_write)
	}
	fn flush(&mut self) -> std::io::Result<()> {
		self.inner.flush()
	}
}
