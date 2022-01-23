/// A draft structure to read and write frames.
///
/// Needs an implementation of `Frame::check`.


use tokio::net::TcoStream;
use tokio::io::{ self, BufWriter, AsyncWriteExt };
use mini_redis::{ Frame, Result };
use mini_redis::frame::Error::Incomplete;
use bytes::{ Buf, BytesMut };
use std::io::Cursor;

const BUF_CAPACITY = 4096;


enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}


struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}


impl Connection {

    pub fn new(stream: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(stream), 
            buffer: BytesMut::with_capacity(BUF_CAPACITY),
        }
    }

    /// Read a frame from the connection. 
    ///
    /// Return `None` if EOF is reached.
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {

        loop {
            // Attempt to parse a frame from the buffered data. 
            // If enough data has been buffered, the frame is returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // There is not enough buffered data to read a frame. 
            // Attempt to read more data from the socket. 
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // A value of 0 indicates a closed stream.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }


    /// Write a frame to the connection.
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Integer(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_decimal(*val).await?;
            },
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
                self.stream.write_decimal(*val).await?;
            },
            Frame::Bulk(val) => {
                let len = val.len();
                self.stream.write_u8(b'$').await?;
                self.stream.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            },
            Frame::Array(_val) => unimplemented!(),
        }

        self.stream.flush().await;
        Ok(())
    }


    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            Ok(_) => {
                
                // get the byte length of the frame
                let len = buf.position() as usize;

                // reset the internal cursor
                buf.set_position(0);

                // parse the frame
                let frame = Frame::parse(&mut buf)?;

                // discard the frame from the buffer
                self.buffer.advance(len);

                // return the frame to the caller
                Ok(Some(frame))
            }
            Err(Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
