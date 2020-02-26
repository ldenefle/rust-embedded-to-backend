#![no_std]

use core::panic::PanicInfo;
extern crate parser;
use parser::parse_packet;
use cty;
use core::slice;

const SAMPLE_MAX_SIZE: usize = 20;
const PACKETS_MAX_SIZE: usize = 50;

#[repr(C)]
pub struct CPacket {
    pub timestamp: cty::uint32_t,
    pub stream_id: cty::uint32_t,
    pub sample_len: cty::uint16_t,
    pub sample: [cty::uint8_t; SAMPLE_MAX_SIZE],
}

#[repr(C)]
pub struct CParserResult {
    pub packets: [CPacket; PACKETS_MAX_SIZE],
    pub packets_len: cty::uint16_t,
    pub remaining: cty::uint32_t,
}

#[no_mangle]
pub extern "C" fn C_parse_packet(
    bytes: *mut cty::uint8_t,
    bytes_len: cty::uint16_t,
    result: *mut CParserResult,
) -> cty::c_int {
    // Check that we don't get null pointers
    if result.is_null() || bytes.is_null() {
        return -1;
    }

    unsafe {
        let bytes: &[u8] = slice::from_raw_parts(bytes, bytes_len as usize);
        let parser_result = match parse_packet(bytes) {
            Ok(res) => res,
            Err(_) => return -1,
        };
        // Copy the remaining packets number
        (*result).remaining = parser_result.remaining as cty::uint32_t;
        (*result).packets_len = parser_result.packets.len() as cty::uint16_t;
        // Copy the packets
        parser_result
            .packets
            .iter()
            .map(|y| {
                let mut ret = CPacket {
                    timestamp: y.timestamp as cty::uint32_t,
                    stream_id: y.stream_id as cty::uint32_t,
                    sample_len: y.sample.len() as cty::uint16_t,
                    sample: [0; 20],
                };
                y.sample
                    .iter()
                    .enumerate()
                    .for_each(|(i, val)| ret.sample[i] = *val);
                ret
            })
            .enumerate()
            .for_each(|(i, val)| (*result).packets[i] = val);
    };
    0
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

