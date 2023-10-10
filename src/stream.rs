use anyhow::Result;
use bytes::Bytes;

use wasmtime_wasi::preview2::{HostInputStream, HostOutputStream, OutputStreamError, StreamState};

pub struct InputStream {}

#[async_trait::async_trait]
impl HostInputStream for InputStream {
    fn read(&mut self, size: usize) -> Result<(Bytes, StreamState)> {
        println!("read {} bytes", size);
        let ret = Bytes::from(vec![0; size]);
        Ok((ret, StreamState::Open))
    }

    async fn ready(&mut self) -> Result<()> {
        Ok(())
    }
}

pub struct OutputStream {}

#[async_trait::async_trait]
impl HostOutputStream for OutputStream {
    fn write(&mut self, bytes: Bytes) -> Result<(), OutputStreamError> {
        println!(
            "write {}",
            String::from_utf8(bytes.to_vec()).map_err(|e| OutputStreamError::Trap(e.into()))?
        );
        Ok(())
    }

    fn flush(&mut self) -> Result<(), OutputStreamError> {
        Ok(())
    }

    async fn write_ready(&mut self) -> Result<usize, OutputStreamError> {
        Ok(256 * 256)
    }
}
