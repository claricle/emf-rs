//! EMF/EMF+/WMF metafile record parser.
//!
//! Rust port of the pure-Ruby bindata parser in claricle/emf. Goal:
//! byte-for-byte record-model parity with the Ruby parser across the
//! fixture corpus (see claricle/emf#15 for the port plan and the
//! parsanol-pair structure this workspace mirrors).

/// Parse error for malformed/truncated metafile input.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("truncated record at offset {offset}")]
    Truncated { offset: usize },
    #[error("unsupported record type {record_type}")]
    Unsupported { record_type: u32 },
}

/// Placeholder for the typed record model (EMF/EMF+/WMF records).
pub mod record {
    use super::ParseError;

    /// One decoded metafile record. Shape to be finalized against the
    /// Ruby domain model in claricle/emf.
    #[derive(Debug, Clone)]
    pub struct Record {
        pub record_type: u32,
        pub size: u32,
        pub payload: Vec<u8>,
    }

    /// Parse the metafile record sequence from a byte slice.
    ///
    /// TODO(#15): replace this scaffold with the full record dispatch —
    /// parity harness: claricle/emf spec/fixtures/{emf,emf-ea,simple}.
    pub fn parse_records(_input: &[u8]) -> Result<Vec<Record>, ParseError> {
        Ok(Vec::new())
    }
}
