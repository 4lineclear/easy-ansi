use std::{fmt, io};

use crate::{DiscreteSGR, SGRString};

/// An interface for an [`SGRWriter`] to work with
///
/// Does not provide SGR writing capability itself
pub trait CapableWriter: Sized {
    /// The type of error returned by trait methods
    ///
    /// Will typically be [`std::io::Error`] or [`std::fmt::Error`]
    type Error: std::error::Error;
    /// Writes a [`str`] to the inner writer
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    fn write(&mut self, s: &str) -> Result<(), Self::Error>;
}
/// A writer built on top of a [`CapableWriter`]
/// that has the ability to work with SGR codes
pub trait SGRWriter: CapableWriter {
    /// Writes a [`str`] to the inner writer
    ///
    /// A shortcut to [`CapableWriter::write`] without having to import it
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn write_inner(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write(s)
    }
    /// Returns a new, empty [`SGRBuilder`]
    ///
    /// Used for convenience
    #[inline]
    fn builder(&self) -> SGRBuilder {
        SGRBuilder::default()
    }
    /// Writes the contained SGR codes to the writer through calling [`SGRString::place_all`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn place_sgr(&mut self, sgr: &SGRString) -> Result<(), Self::Error> {
        let mut builder = SGRBuilder::default();
        sgr.place_all(&mut builder);
        builder.write_to(self)
    }
    /// Writes the contained SGR codes to the writer through calling [`SGRString::clean_all`]
    ///
    /// Supposed to reverse the effects made by [`SGRString::place_all`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn clean_sgr(&mut self, sgr: &SGRString) -> Result<(), Self::Error> {
        let mut builder = SGRBuilder::default();
        sgr.clean_all(&mut builder);
        builder.write_to(self)
    }
    /// Writes the contained SGR codes to the writer through calling [`DiscreteSGR::write`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    fn inline_sgr(&mut self, sgr: &impl DiscreteSGR) -> Result<(), Self::Error> {
        let mut builder = SGRBuilder::default();
        sgr.write(&mut builder);
        builder.write_to(self)
    }
    /// Writes the contained SGR codes to the writer
    ///
    /// Uses [`EasyWrite`] so the it can be used for both
    /// [`SGRString`] and [`DiscreteSGR`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    fn sgr(&mut self, sgr: &impl EasyWrite) -> Result<(), Self::Error> {
        let mut builder = SGRBuilder::default();
        sgr.sgr(&mut builder);
        builder.write_to(self)
    }
    /// Writes the contained SGR codes to the writer
    ///
    /// Does not write the escape or end sequences
    ///
    /// Uses [`EasyWrite`] so the it can be used for both
    /// [`SGRString`] and [`DiscreteSGR`]
    ///
    /// # Errors
    ///
    /// Returns an error if writing fails.
    /// Error type specified by [`CapableWriter::Error`]
    #[inline]
    #[cfg(feature = "partial")]
    fn partial_sgr(&mut self, sgr: &impl EasyWrite) -> Result<(), Self::Error> {
        let mut builder = SGRBuilder::default();
        sgr.sgr(&mut builder);
        builder.write_partial(self)
    }
}
/// A Standard SGR writer
#[derive(Debug, Clone)]
pub struct StandardWriter<W: CapableWriter> {
    /// A writer capable of writing a [`str`]
    pub writer: W,
}
impl<W: CapableWriter> From<W> for StandardWriter<W> {
    fn from(value: W) -> Self {
        Self { writer: value }
    }
}
impl<W: std::fmt::Write> From<W> for StandardWriter<FmtWriter<W>> {
    fn from(value: W) -> Self {
        Self {
            writer: FmtWriter(value),
        }
    }
}
impl<W: std::io::Write> From<W> for StandardWriter<IoWriter<W>> {
    fn from(value: W) -> Self {
        Self {
            writer: IoWriter(value),
        }
    }
}
impl<W: CapableWriter> CapableWriter for StandardWriter<W> {
    type Error = W::Error;
    #[inline]
    fn write(&mut self, s: &str) -> Result<(), Self::Error> {
        self.writer.write(s)
    }
}
impl<W: CapableWriter> SGRWriter for StandardWriter<W> {}
/// Used to implement [`CapableWriter`] for [`std::io::Write`]
#[derive(Debug, Clone)]
pub struct IoWriter<W: std::io::Write>(pub W);
impl<W: std::io::Write> CapableWriter for IoWriter<W> {
    type Error = io::Error;
    #[inline]
    fn write(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.write_all(s.as_bytes())
    }
}
/// Used to implement [`CapableWriter`] for [`std::fmt::Write`]
#[derive(Debug, Clone)]
pub struct FmtWriter<W: std::fmt::Write>(pub W);
impl<W: std::fmt::Write> CapableWriter for FmtWriter<W> {
    type Error = fmt::Error;
    #[inline]
    fn write(&mut self, s: &str) -> Result<(), Self::Error> {
        self.0.write_str(s)
    }
}
/// Builds a SGR sequence
#[derive(Debug, Default)]
pub struct SGRBuilder(pub Vec<u8>);

impl SGRBuilder {
    /// Writes a code to the internal buffer
    #[inline]
    pub fn write_code(&mut self, code: u8) {
        self.0.push(code);
    }
    /// Writes codes to the internal buffer
    #[inline]
    pub fn write_codes(&mut self, codes: &[u8]) {
        self.0.extend_from_slice(codes);
    }
    /// Writes a code to the internal buffer
    ///
    /// Returns self to allow for chaining
    #[inline]
    pub fn chain_code(&mut self, code: u8) -> &mut Self {
        self.0.push(code);
        self
    }
    /// Writes codes to the internal buffer
    ///
    /// Returns self to allow for chaining
    #[inline]
    pub fn chain_codes(&mut self, codes: &[u8]) -> &mut Self {
        self.0.extend_from_slice(codes);
        self
    }
    /// Writes buffered codes to the provided writer
    ///
    /// # Errors
    ///
    /// Writing failed
    pub fn write_to<W: SGRWriter>(&mut self, writer: &mut W) -> Result<(), W::Error> {
        if self.0.is_empty() {
            Ok(())
        } else {
            writer.write("\x1b[")?;
            writer.write_inner(&self.0[0].to_string())?;

            for code in &self.0[1..] {
                writer.write(";")?;
                writer.write(&code.to_string())?;
            }
            self.0.clear();
            writer.write("m")
        }
    }
    /// Writes buffered codes to the writer
    ///
    /// Does not write the escape or end sequences
    ///
    /// Performs IO operations with the internal [`SGRWriter`]
    ///
    /// # Errors
    ///
    /// Writing failed
    pub fn write_partial<W: SGRWriter>(&mut self, writer: &mut W) -> Result<(), W::Error> {
        if !self.0.is_empty() {
            writer.write_inner(&self.0[0].to_string())?;

            for code in &self.0[1..] {
                writer.write(";")?;
                writer.write(&code.to_string())?;
            }
            self.0.clear();
        }
        Ok(())
    }
}

/// Helps to make writing easier
///
/// Allows to use the same method for both
/// [`SGRString`] and [`DiscreteSGR`] types
pub trait EasyWrite {
    /// Writes a set of codes to the builder
    fn sgr(&self, builder: &mut SGRBuilder);
}

impl EasyWrite for SGRString {
    /// Writes a set of codes to the builder
    ///
    /// Uses [`SGRString::place_all`]
    fn sgr(&self, builder: &mut SGRBuilder) {
        self.place_all(builder);
    }
}

impl<D: DiscreteSGR> EasyWrite for D {
    /// Writes a set of codes to the builder
    ///
    /// Uses [`DiscreteSGR::write`]
    fn sgr(&self, builder: &mut SGRBuilder) {
        self.write(builder);
    }
}
