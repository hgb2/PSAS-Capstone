///////////////////////////////////////////////////////////////////////////////
// File Name: data_fmt.rs
//
// Purpose: Takes sensor data and GPIO states stored in SharedMemory, packages
// the data into format consistent with a message definition defined in the
// PSAS packet serializer, and then sends a UDP packet to the PSAS telemetry
// viewer.
//
///////////////////////////////////////////////////////////////////////////////


extern crate byteorder;

use std::io::Cursor;
use self::byteorder::{WriteBytesExt, BigEndian};
use std::io::Write;
use std::io;
use std::time;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use std::net::UdpSocket;
use SharedMemory;


//const P_LIMIT: usize = 26;                            // MAX size of packet: Default(1432) During testing reduce to 26
const HEADER_SIZE: usize = 12;                          // PSAS header size (bytes)
//const SIZE_OF_MESSAGE: usize = 14;                      // Shared memory/message size (bytes)
//const RCSS_NAME: [u8;4] = [82, 67, 83, 83];             // ASCII name for PSAS message format (RCSS)
const PSAS_TELEMETRY_UDP_PORT: u16 = 35001;             // UDP packet target port

const P_LIMIT: usize = 36;
const SIZE_OF_MESSAGE: usize = 24;
const RCSS_NAME: [u8;4] = [65, 68, 73, 83]; // rust-fc [65, 68, 73, 83] dec


///////////////////////////////////////////////////////////////////////////////
// Function name: pack_header
//
// Purpose: Packs header information into a byte array.
//
// INPUTS: name         -- ASCII PSAS message definition name.
//         time         -- Time duration with respect to boot time.
//         message_size -- Size of message associated with this header type.
//         buffer       -- Buffer/byte array to hold header information.
//
// RETURN: Ok(0)  -- all is well
//         Err(io::Error) -- an error occurred
//
///////////////////////////////////////////////////////////////////////////////
fn pack_header(name: [u8; 4], time: time::Duration, message_size: usize, buffer: &mut [u8; HEADER_SIZE]) -> Result<u8, io::Error> {

    let mut header = Cursor::<&mut [u8]>::new(buffer);

    // Write header name: ID (Four character code)
    try!(header.write(&name));

    // Write timestamp: 6 bytes nanoseconds from boot
    let nanos: u64 = (time.as_secs() * 1000000000) + time.subsec_nanos() as u64;
    let mut time_buffer = [0u8; 8];
    {
        let mut t = Cursor::<&mut [u8]>::new(&mut time_buffer);
        try!(t.write_u64::<BigEndian>(nanos));
    }
    // Truncate to 6 least significant bytes
    try!(header.write(&time_buffer[2..8]));

    // Write the size of the message associated with the header.
    try!(header.write_u16::<BigEndian>(message_size as u16));

    Ok(0)
}


///////////////////////////////////////////////////////////////////////////////
// Function name: as_message
//
// Purpose: Packs select pieces of data from SharedMemory into a byte array.
//
// INPUTS: mem    -- Reverence to SharedMemory
//         buffer -- Buffer/byte array to hold message information.
//
// RETURN: Ok(0)  -- all is well
//         Err(io::Error) -- an error occurred
//
///////////////////////////////////////////////////////////////////////////////
// Pack PSAS data information into a byte array
fn as_message(mem: &mut SharedMemory, buffer: &mut [u8; SIZE_OF_MESSAGE]) -> Result<u8, io::Error> {

    let mut message = Cursor::<&mut [u8]>::new(buffer);

    // Write RCSS message
    /*
    try!(message.write_f32::<BigEndian>(mem.gyro_x));
    try!(message.write_f32::<BigEndian>(mem.gyro_y));
    try!(message.write_f32::<BigEndian>(mem.gyro_z));
    try!(message.write_u8(mem.cw_state));
    try!(message.write_u8(mem.ccw_state));
*/


try!(message.write_u16::<BigEndian>(0));
   try!(message.write_i16::<BigEndian>(f32::floor(mem.gyro_x) as i16));
   try!(message.write_i16::<BigEndian>(f32::floor(mem.gyro_y) as i16));
   try!(message.write_i16::<BigEndian>(f32::floor(mem.gyro_z) as i16));
   try!(message.write_i16::<BigEndian>(0));
   try!(message.write_i16::<BigEndian>(0));
   try!(message.write_i16::<BigEndian>(0));
   try!(message.write_i16::<BigEndian>(0));
   try!(message.write_i16::<BigEndian>(0));
   try!(message.write_i16::<BigEndian>(0));
   try!(message.write_i16::<BigEndian>(0));
   try!(message.write_u16::<BigEndian>(0));
    Ok(0)
}


///////////////////////////////////////////////////////////////////////////////
// Function name: flush_telemetry
//
// Purpose: Establishes an address for the UDP packet target, sends a UDP
// packet, then clears the "telemetry_buffer" for the next packet.
//
// INPUTS: socket -- Reference to source UDP socket
//         mem -- Reference to SharedMemory
//
// RETURN: Ok(0)  -- all is well
//         Err(io::Error) -- an error occurred
//
///////////////////////////////////////////////////////////////////////////////
fn flush_telemetry(socket: &UdpSocket, mem: &mut SharedMemory) -> Result<u8, io::Error> {

    // Address for UDP packet target
    let telemetry_addr: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(192, 168, 2, 12), PSAS_TELEMETRY_UDP_PORT);

    //send UDP packet to UDP packet target
    try!(socket.send_to(&mem.telemetry_buffer, telemetry_addr));

    // Increment SEQN
    mem.sequence_number += 1;

    // Start telemetry buffer over
    mem.telemetry_buffer.clear();

    Ok(0)
}



///////////////////////////////////////////////////////////////////////////////
// Function name: send_packet
//
// Purpose: Calls functions "pack_header" and "as_message" which produces
// byte arrays. These byte arrays are stored in a buffer; if the contents of
// the buffer exceeds the size specified by "P_LIMIT" - this results in the
// transmition of the UDP packet.
//
// INPUTS: socket -- Reference to source UDP socket
//         mem    -- Reference to SharedMemory
//
// RETURN: Ok(0)  -- all is well
//         Err(io::Error) -- an error occurred
//
///////////////////////////////////////////////////////////////////////////////
pub fn send_packet(socket: &UdpSocket, mem: &mut SharedMemory) -> Result<u8, io::Error>  {

    // Get timestamp for this header
    let now = time::Instant::now().duration_since(mem.boot_time);

    // Send UDP packet once packet size limit (P_LIMIT) has been reached
    if (mem.telemetry_buffer.len() + HEADER_SIZE + SIZE_OF_MESSAGE) > P_LIMIT {
        try!(flush_telemetry(socket, mem));
    }

    // Append sequence number into empty telemetry buffer/vector.
    if mem.telemetry_buffer.len() == 0 {
        let mut seqn = Vec::with_capacity(4);
        try!(seqn.write_u32::<BigEndian>(mem.sequence_number));
        mem.telemetry_buffer.extend_from_slice(&mut seqn);
    }

    // Pack Header into telemetry buffer
    let mut header_buffer = [0u8; HEADER_SIZE];
    try!(pack_header(RCSS_NAME, now, SIZE_OF_MESSAGE, &mut header_buffer));
    mem.telemetry_buffer.extend_from_slice(&header_buffer);

    // Pack message into telemetry buffer
    let mut message_buffer = [0u8; SIZE_OF_MESSAGE];
    try!(as_message(mem, &mut message_buffer));
    mem.telemetry_buffer.extend_from_slice(&message_buffer);

    Ok(0)
}
