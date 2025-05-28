use crate::*;

pub use decoder::ExecutionReportDecoder;
pub use encoder::ExecutionReportEncoder;

pub use crate::SBE_SCHEMA_ID;
pub use crate::SBE_SCHEMA_VERSION;
pub use crate::SBE_SEMANTIC_VERSION;

pub const SBE_BLOCK_LENGTH: u16 = 114;
pub const SBE_TEMPLATE_ID: u16 = 3;

pub mod encoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Debug, Default)]
    pub struct ExecutionReportEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for ExecutionReportEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for ExecutionReportEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> ExecutionReportEncoder<'a> {
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
            let iter = value
                .iter()
                .copied()
                .chain(std::iter::repeat(0_u8))
                .take(16);
            self.cl_ord_id_from_iter(iter);
        }

        #[inline]
        pub fn party_id_at(&mut self, index: usize, value: u8) {
            let offset = self.offset + 16;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset + index, value);
        }

        /// primitive array field 'PartyID'
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: String
        /// - encodedOffset: 16
        /// - encodedLength: 16
        /// - version: 0
        #[inline]
        pub fn party_id(&mut self, value: &[u8]) {
            debug_assert_eq!(16, value.len());
            let offset = self.offset + 16;
            let buf = self.get_buf_mut();
            buf.put_slice_at(offset, value);
        }

        /// primitive array field 'PartyID' from an Iterator
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: String
        /// - encodedOffset: 16
        /// - encodedLength: 16
        /// - version: 0
        #[inline]
        pub fn party_id_from_iter(&mut self, iter: impl Iterator<Item = u8>) {
            let offset = self.offset + 16;
            let buf = self.get_buf_mut();
            for (i, v) in iter.enumerate() {
                buf.put_u8_at(offset + i, v);
            }
        }

        /// primitive array field 'PartyID' with zero padding
        /// - min value: 0
        /// - max value: 254
        /// - null value: 0xff_u8
        /// - characterEncoding: null
        /// - semanticType: String
        /// - encodedOffset: 16
        /// - encodedLength: 16
        /// - version: 0
        #[inline]
        pub fn party_id_zero_padded(&mut self, value: &[u8]) {
            let iter = value
                .iter()
                .copied()
                .chain(std::iter::repeat(0_u8))
                .take(16);
            self.party_id_from_iter(iter);
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

        /// primitive field 'ExecID'
        /// - min value: 0
        /// - max value: -2
        /// - null value: 0xffffffffffffffff_u64
        /// - characterEncoding: null
        /// - semanticType: uint64
        /// - encodedOffset: 40
        /// - encodedLength: 8
        /// - version: 0
        #[inline]
        pub fn exec_id(&mut self, value: u64) {
            let offset = self.offset + 40;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn transact_time_encoder(
            self,
        ) -> utc_timestamp_nanos_codec::UTCTimestampNanosEncoder<Self> {
            let offset = self.offset + 48;
            utc_timestamp_nanos_codec::UTCTimestampNanosEncoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn price_encoder(self) -> decimal_64_codec::Decimal64Encoder<Self> {
            let offset = self.offset + 56;
            decimal_64_codec::Decimal64Encoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn last_qty_encoder(self) -> decimal_64_codec::Decimal64Encoder<Self> {
            let offset = self.offset + 64;
            decimal_64_codec::Decimal64Encoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn last_px_encoder(self) -> decimal_64_codec::Decimal64Encoder<Self> {
            let offset = self.offset + 72;
            decimal_64_codec::Decimal64Encoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn leaves_qty_encoder(self) -> decimal_64_codec::Decimal64Encoder<Self> {
            let offset = self.offset + 80;
            decimal_64_codec::Decimal64Encoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn cum_qty_encoder(self) -> decimal_64_codec::Decimal64Encoder<Self> {
            let offset = self.offset + 88;
            decimal_64_codec::Decimal64Encoder::default().wrap(self, offset)
        }

        /// COMPOSITE ENCODER
        #[inline]
        pub fn avg_px_encoder(self) -> decimal_64_codec::Decimal64Encoder<Self> {
            let offset = self.offset + 96;
            decimal_64_codec::Decimal64Encoder::default().wrap(self, offset)
        }

        #[inline]
        pub fn symbol_at(&mut self, index: usize, value: u8) {
            let offset = self.offset + 104;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset + index, value);
        }

        /// primitive array field 'Symbol'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: ISO_8859_1
        /// - semanticType: String
        /// - encodedOffset: 104
        /// - encodedLength: 6
        /// - version: 0
        #[inline]
        pub fn symbol(&mut self, value: &[u8]) {
            debug_assert_eq!(6, value.len());
            let offset = self.offset + 104;
            let buf = self.get_buf_mut();
            buf.put_slice_at(offset, value);
        }

        /// primitive array field 'Symbol' from an Iterator
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: ISO_8859_1
        /// - semanticType: String
        /// - encodedOffset: 104
        /// - encodedLength: 6
        /// - version: 0
        #[inline]
        pub fn symbol_from_iter(&mut self, iter: impl Iterator<Item = u8>) {
            let offset = self.offset + 104;
            let buf = self.get_buf_mut();
            for (i, v) in iter.enumerate() {
                buf.put_u8_at(offset + i, v);
            }
        }

        /// primitive array field 'Symbol' with zero padding
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0_u8
        /// - characterEncoding: ISO_8859_1
        /// - semanticType: String
        /// - encodedOffset: 104
        /// - encodedLength: 6
        /// - version: 0
        #[inline]
        pub fn symbol_zero_padded(&mut self, value: &[u8]) {
            let iter = value.iter().copied().chain(std::iter::repeat(0_u8)).take(6);
            self.symbol_from_iter(iter);
        }

        /// REQUIRED enum
        #[inline]
        pub fn exec_type(&mut self, value: exec_type_enum::ExecTypeEnum) {
            let offset = self.offset + 110;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_status(&mut self, value: ord_status_enum::OrdStatusEnum) {
            let offset = self.offset + 111;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_rej_reason(&mut self, value: ord_rej_reason_enum::OrdRejReasonEnum) {
            let offset = self.offset + 112;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&mut self, value: side_enum::SideEnum) {
            let offset = self.offset + 113;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }
    }
} // end encoder

pub mod decoder {
    use super::*;
    use message_header_codec::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct ExecutionReportDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl ActingVersion for ExecutionReportDecoder<'_> {
        #[inline]
        fn acting_version(&self) -> u16 {
            self.acting_version
        }
    }

    impl<'a> Reader<'a> for ExecutionReportDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for ExecutionReportDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> ExecutionReportDecoder<'a> {
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
        pub fn party_id(&self) -> [u8; 16] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 16)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn order_id(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 32)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn exec_id(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 40)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn transact_time_decoder(
            self,
        ) -> utc_timestamp_nanos_codec::UTCTimestampNanosDecoder<Self> {
            let offset = self.offset + 48;
            utc_timestamp_nanos_codec::UTCTimestampNanosDecoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn price_decoder(self) -> decimal_64_codec::Decimal64Decoder<Self> {
            let offset = self.offset + 56;
            decimal_64_codec::Decimal64Decoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn last_qty_decoder(self) -> decimal_64_codec::Decimal64Decoder<Self> {
            let offset = self.offset + 64;
            decimal_64_codec::Decimal64Decoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn last_px_decoder(self) -> decimal_64_codec::Decimal64Decoder<Self> {
            let offset = self.offset + 72;
            decimal_64_codec::Decimal64Decoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn leaves_qty_decoder(self) -> decimal_64_codec::Decimal64Decoder<Self> {
            let offset = self.offset + 80;
            decimal_64_codec::Decimal64Decoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn cum_qty_decoder(self) -> decimal_64_codec::Decimal64Decoder<Self> {
            let offset = self.offset + 88;
            decimal_64_codec::Decimal64Decoder::default().wrap(self, offset)
        }

        /// COMPOSITE DECODER
        #[inline]
        pub fn avg_px_decoder(self) -> decimal_64_codec::Decimal64Decoder<Self> {
            let offset = self.offset + 96;
            decimal_64_codec::Decimal64Decoder::default().wrap(self, offset)
        }

        #[inline]
        pub fn symbol(&self) -> [u8; 6] {
            let buf = self.get_buf();
            ReadBuf::get_bytes_at(buf.data, self.offset + 104)
        }

        /// REQUIRED enum
        #[inline]
        pub fn exec_type(&self) -> exec_type_enum::ExecTypeEnum {
            self.get_buf().get_u8_at(self.offset + 110).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_status(&self) -> ord_status_enum::OrdStatusEnum {
            self.get_buf().get_u8_at(self.offset + 111).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn ord_rej_reason(&self) -> ord_rej_reason_enum::OrdRejReasonEnum {
            self.get_buf().get_u8_at(self.offset + 112).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&self) -> side_enum::SideEnum {
            self.get_buf().get_u8_at(self.offset + 113).into()
        }
    }
} // end decoder
