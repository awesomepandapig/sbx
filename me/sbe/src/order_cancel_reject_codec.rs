use crate::*;

pub use decoder::OrderCancelRejectDecoder;
pub use encoder::OrderCancelRejectEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 43;
pub const SBE_TEMPLATE_ID: u16 = 4;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct OrderCancelRejectEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for OrderCancelRejectEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for OrderCancelRejectEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderCancelRejectEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        #[inline]
        pub fn cl_ord_id_at(&mut self, index: usize, value: u8) {
            let offset = self.offset;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset + index, value);
        }

        /// primitive array field 'ClOrdId'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: String
        /// - encodedOffset: 0
        /// - encodedLength: 16
        /// - version: 0
        #[inline]
        pub fn cl_ord_id(&mut self, value: &[u8]) {
            debug_assert_eq!(16, value.len());
            let offset = self.offset;
            let buf = self.get_buf_mut();
            buf.put_slice_at(offset, value);
        }

        /// primitive array field 'ClOrdId' from an Iterator
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: String
        /// - encodedOffset: 0
        /// - encodedLength: 16
        /// - version: 0
        #[inline]
        pub fn cl_ord_id_from_iter(&mut self, iter: impl Iterator<Item = u8>) {
            let offset = self.offset;
            let buf = self.get_buf_mut();
            for (i, v) in iter.enumerate() {
                buf.put_u8_at(offset + i, v);
            }
        }

        /// primitive array field 'ClOrdId' with zero padding
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: String
        /// - encodedOffset: 0
        /// - encodedLength: 16
        /// - version: 0
        #[inline]
        pub fn cl_ord_id_zero_padded(&mut self, value: &[u8]) {
            let iter = value.iter().copied().chain(std::iter::repeat(0_u8)).take(16);
            self.cl_ord_id_from_iter(iter);
        }

        #[inline]
        pub fn orig_cl_ord_id_at(&mut self, index: usize, value: u8) {
            let offset = self.offset + 16;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset + index, value);
        }

        /// primitive array field 'OrigClOrdID'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: String
        /// - encodedOffset: 16
        /// - encodedLength: 16
        /// - version: 0
        #[inline]
        pub fn orig_cl_ord_id(&mut self, value: &[u8]) {
            debug_assert_eq!(16, value.len());
            let offset = self.offset + 16;
            let buf = self.get_buf_mut();
            buf.put_slice_at(offset, value);
        }

        /// primitive array field 'OrigClOrdID' from an Iterator
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: String
        /// - encodedOffset: 16
        /// - encodedLength: 16
        /// - version: 0
        #[inline]
        pub fn orig_cl_ord_id_from_iter(&mut self, iter: impl Iterator<Item = u8>) {
            let offset = self.offset + 16;
            let buf = self.get_buf_mut();
            for (i, v) in iter.enumerate() {
                buf.put_u8_at(offset + i, v);
            }
        }

        /// primitive array field 'OrigClOrdID' with zero padding
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: String
        /// - encodedOffset: 16
        /// - encodedLength: 16
        /// - version: 0
        #[inline]
        pub fn orig_cl_ord_id_zero_padded(&mut self, value: &[u8]) {
            let iter = value.iter().copied().chain(std::iter::repeat(0_u8)).take(16);
            self.orig_cl_ord_id_from_iter(iter);
        }

        /// primitive field 'OrderID'
        /// - min value: 0
        /// - max value: -2
        /// - null value: 0xffffffffffffffff_u64
        /// - characterEncoding: null
        /// - semanticType: uint64
        /// - encodedOffset: 32
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn order_id(&mut self, value: u64) {
            let offset = self.offset + 32;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_status(&mut self, value: ord_status_enum::OrdStatusEnum) {
            let offset = self.offset + 40;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn cxl_rej_response_to(&mut self, value: cxl_rej_response_to_enum::CxlRejResponseToEnum) {
            let offset = self.offset + 41;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn cxl_rej_reason(&mut self, value: cxl_rej_reason_enum::CxlRejReasonEnum) {
            let offset = self.offset + 42;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

    }

} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct OrderCancelRejectDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl ActingVersion for OrderCancelRejectDecoder<'_> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for OrderCancelRejectDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for OrderCancelRejectDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> OrderCancelRejectDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>, offset: usize) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                offset + message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        #[inline]
        pub fn cl_ord_id(&self) -> [u8; 16] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset)
        }

        #[inline]
        pub fn orig_cl_ord_id(&self) -> [u8; 16] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 16)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn order_id(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 32)
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_status(&self) -> ord_status_enum::OrdStatusEnum {
            self.get_buf().get_u8_at(self.offset + 40).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn cxl_rej_response_to(&self) -> cxl_rej_response_to_enum::CxlRejResponseToEnum {
            self.get_buf().get_u8_at(self.offset + 41).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn cxl_rej_reason(&self) -> cxl_rej_reason_enum::CxlRejReasonEnum {
            self.get_buf().get_u8_at(self.offset + 42).into()
        }

    }

} // end decoder

