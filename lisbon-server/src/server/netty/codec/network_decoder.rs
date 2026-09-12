//! Mirrors `net.h4bbo.lisbon.server.netty.codec.NetworkDecoder`.
//!
//! Java extends `ByteToMessageDecoder` and pulls frames out of a Netty
//! `ByteBuf`. Here the accumulator is a `Vec<u8>`; `decode` removes a whole
//! frame when one is complete and leaves the buffer untouched otherwise.

use crate::server::netty::streams::netty_request::NettyRequest;
use crate::util::encoding::base64_encoding::Base64Encoding;

/// Decodes inbound game frames.
pub struct NetworkDecoder;

impl NetworkDecoder {
    /// Mirrors `decode(ChannelHandlerContext, ByteBuf, List<Object>)`.
    ///
    /// Returns `Some(NettyRequest)` when a complete frame was decoded (the
    /// frame bytes are removed from `buf`), or `None` when more bytes are
    /// required.
    pub fn decode(buf: &mut Vec<u8>) -> Option<NettyRequest> {
        if buf.len() < 5 {
            // If the incoming data is less than 5 bytes, it's junk.
            return None;
        }

        // int length = Base64Encoding.decode(new byte[]{b0, b1, b2})
        let length = Base64Encoding::decode(&buf[0..3]);
        if length < 0 {
            return None;
        }

        // if (buffer.readableBytes() < length) resetReaderIndex(); return;
        if (buf.len() - 3) < length as usize {
            return None;
        }

        // out.add(new NettyRequest(buffer.readBytes(length)))
        let frame_len = 3 + length as usize;
        let body = buf[3..frame_len].to_vec();
        buf.drain(..frame_len);

        Some(NettyRequest::new(body))
    }
}
