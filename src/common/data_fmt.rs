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
use std::time;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use std::net::UdpSocket;
use SharedMemory;
use UpdateResult;


const P_LIMIT: usize = 1432;				// MAX size of packet: Default(1432) During testing reduce to 26
const HEADER_SIZE: usize = 12;				// PSAS header size (bytes)
const SIZE_OF_MESSAGE: usize = 14;			// Shared memory/message size (bytes)
const RCSS_NAME: [u8;4] = [82, 67, 83, 83];	// ASCII name for PSAS message format (RCSS)
const PSAS_TELEMETRY_UDP_PORT: u16 = 35001; // UDP packet target port


///////////////////////////////////////////////////////////////////////////////
// Function name: pack_header
//
// Purpose:	Packs header information into a byte array.
//	
// INPUTS: name         -- ASCII PSAS message definition name.
//         time         -- Time duration with respect to boot time.
//         message_size -- Size of message associated with this header type.
//
// RETURN: Byte array of header.
//
///////////////////////////////////////////////////////////////////////////////
fn pack_header(name: [u8; 4], time: time::Duration, message_size: usize) -> [u8; HEADER_SIZE] {

    let mut buffer = [0u8; HEADER_SIZE];
    {
        let mut header = Cursor::<&mut [u8]>::new(&mut buffer);

        // Fields:
        // ID (Four character code)
        header.write(&name).unwrap();

        // Timestamp, 6 bytes nanoseconds from boot
        let nanos: u64 = (time.as_secs() * 1000000000) + time.subsec_nanos() as u64;
        let mut time_buffer = [0u8; 8];
        {
            let mut t = Cursor::<&mut [u8]>::new(&mut time_buffer);
            t.write_u64::<BigEndian>(nanos).unwrap();
        }
        // Truncate to 6 least significant bytes
        header.write(&time_buffer[2..8]).unwrap();

        // Size:
        header.write_u16::<BigEndian>(message_size as u16).unwrap();
    }
    buffer
}


///////////////////////////////////////////////////////////////////////////////
// Function name: as_message
//
// Purpose:	Packs select pieces of data from SharedMemory into a byte array.
//	
// INPUTS: mem -- Reverence to SharedMemory
//
// RETURN: Byte array of message.
//
///////////////////////////////////////////////////////////////////////////////
// Pack PSAS data information into a byte array
pub fn as_message(mem: &mut SharedMemory) -> [u8; SIZE_OF_MESSAGE] {
	let mut buffer = [0u8; SIZE_OF_MESSAGE];
	{
		let mut message = Cursor::<&mut [u8]>::new(&mut buffer);

		//Write RCSS message
		message.write_f32::<BigEndian>(mem.gyro_x).unwrap();
		message.write_f32::<BigEndian>(mem.gyro_y).unwrap();
		message.write_f32::<BigEndian>(mem.gyro_z).unwrap();
		message.write_u8(mem.cw_state).unwrap();
		message.write_u8(mem.ccw_state).unwrap();
	}
	buffer
}


///////////////////////////////////////////////////////////////////////////////
// Function name: flush_telemetry
//
// Purpose:	Establishes an address for the UDP packet target, sends a UDP
// packet, then clear and prepares the "telemetry_buffer" for the next packet.
//	
// INPUTS: socket -- Reference to source UDP socket
//         mem -- Reference to SharedMemory
//
// RETURN: Ok()     -- all is well
//         Err(err) -- an error occurred
//
///////////////////////////////////////////////////////////////////////////////
fn flush_telemetry(socket: &UdpSocket, mem: &mut SharedMemory) {

	// Address for UDP packet target
	let telemetry_addr: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PSAS_TELEMETRY_UDP_PORT);

	//send UDP packet to UDP packet target
	socket.send_to(&mem.telemetry_buffer, telemetry_addr).unwrap();

	// Increment SEQN
	mem.sequence_number += 1;

	// Start telemetry buffer over
	mem.telemetry_buffer.clear();

	// Prepend with next sequence number
	let mut seqn = Vec::with_capacity(4);
	seqn.write_u32::<BigEndian>(mem.sequence_number).unwrap();
	mem.telemetry_buffer.extend_from_slice(&mut seqn);
}
     
	

///////////////////////////////////////////////////////////////////////////////
// Function name: send_packet
//
// Purpose:	Calls functions "pack_header" and "as_message" which produces 
// byte arrays. These byte arrays are stored in a buffer; if the contents of
// the buffer exceeds the size specified by "P_LIMIT" - this results in the 
// transmition of the UDP packet.
//	
// INPUTS: socket -- Reference to source UDP socket
//         mem -- Reference to SharedMemory
//
// RETURN: Ok()     -- all is well
//         Err(err) -- an error occurred
//
///////////////////////////////////////////////////////////////////////////////
pub fn send_packet(socket: &UdpSocket, mem: &mut SharedMemory) -> UpdateResult  {

	// Get timestamp for this header
	let now = time::Instant::now().duration_since(mem.boot_time);
	
	// Send UDP packet once packet size limit (P_LIMIT) has been reached
	if (mem.telemetry_buffer.len() + HEADER_SIZE + SIZE_OF_MESSAGE) > P_LIMIT {
		flush_telemetry(socket, mem);
	}

	// Pack Header into telemetry buffer
	let header = pack_header(RCSS_NAME, now, SIZE_OF_MESSAGE);
	mem.telemetry_buffer.extend_from_slice(&header);

	// Pack message into telemetry buffer
	let message = as_message(mem);
	mem.telemetry_buffer.extend_from_slice(&message);

	Ok(0)
}


