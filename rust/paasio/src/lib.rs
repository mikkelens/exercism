use std::{
	io::{Read, Result, Write},
	marker::PhantomData
};
// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R>(PhantomData<R>);

impl<R: Read> ReadStats<R> {
	// _wrapped is ignored because R is not bounded on Debug or Display and
	// therefore can't be passed through format!(). For actual implementation you
	// will likely wish to remove the leading underscore so the variable is not
	// ignored.
	pub fn new(_wrapped: R) -> ReadStats<R> {
		ReadStats(PhantomData::default())
	}

	pub fn get_ref(&self) -> &R {
		todo!()
	}

	pub fn bytes_through(&self) -> usize {
		todo!()
	}

	pub fn reads(&self) -> usize {
		todo!()
	}
}

impl<R: Read> Read for ReadStats<R> {
	fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		todo!("Collect statistics about this call reading {buf:?}")
	}
}

pub struct WriteStats<W>(PhantomData<W>);

impl<W: Write> WriteStats<W> {
	// _wrapped is ignored because W is not bounded on Debug or Display and
	// therefore can't be passed through format!(). For actual implementation you
	// will likely wish to remove the leading underscore so the variable is not
	// ignored.
	pub fn new(_wrapped: W) -> WriteStats<W> {
		WriteStats(PhantomData::default())
	}

	pub fn get_ref(&self) -> &W {
		todo!()
	}

	pub fn bytes_through(&self) -> usize {
		todo!()
	}

	pub fn writes(&self) -> usize {
		todo!()
	}
}

impl<W: Write> Write for WriteStats<W> {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		todo!("Collect statistics about this call writing {buf:?}")
	}

	fn flush(&mut self) -> Result<()> {
		todo!()
	}
}
