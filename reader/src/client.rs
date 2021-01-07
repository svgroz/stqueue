use core::client::ClientType;
use std::marker::Unpin;
use tokio::io::AsyncReadExt;

pub enum ClientTypeRecivingError {
    IllegalLength(usize),
    ReaderClosed,
    UnknownClientType(u8),
    ReaderErrorWrapper(std::io::Error),
}

pub enum MessageHeaderRecivingError {
    IllegalLength(usize),
    ReaderClosed,
    ReaderErrorWrapper(std::io::Error),
}

pub enum MessagePartRecivingError {
    IllegalLength(usize),
    ReaderClosed,
    ReaderErrorWrapper(std::io::Error),
}

const CLIENT_TYPE_PART_SIZE: usize = 1;
const MESSAGE_SIZE_PART_SIZE: usize = 4;
const MESSAGE_PART_SIZE: usize = 64;

pub async fn read_type<AR: AsyncReadExt + Unpin>(
    reader: &mut AR,
) -> Result<ClientType, ClientTypeRecivingError> {
    let mut client_type_id_buffer: [u8; CLIENT_TYPE_PART_SIZE] = [u8::MAX; CLIENT_TYPE_PART_SIZE];

    let raw_client_type = match reader.read(&mut client_type_id_buffer).await {
        Ok(nbytes) if nbytes == CLIENT_TYPE_PART_SIZE => u8::from_be_bytes(client_type_id_buffer),
        Ok(nbytes) if nbytes == 0 => return Err(ClientTypeRecivingError::ReaderClosed),
        Ok(nbytes) => return Err(ClientTypeRecivingError::IllegalLength(nbytes)),

        Err(err) => return Err(ClientTypeRecivingError::ReaderErrorWrapper(err)),
    };

    match u8::from_be(raw_client_type) {
        0 => Ok(ClientType::Producer),
        1 => Ok(ClientType::Consumer),
        unknown_client_type_id => Err(ClientTypeRecivingError::UnknownClientType(
            unknown_client_type_id,
        )),
    }
}

pub async fn read_message_size_header<AR: AsyncReadExt + Unpin>(
    reader: &mut AR,
) -> Result<u32, MessageHeaderRecivingError> {
    let mut message_size_part_buffer: [u8; MESSAGE_SIZE_PART_SIZE] = [0u8; MESSAGE_SIZE_PART_SIZE];

    let raw_message_size = match reader.read(&mut message_size_part_buffer).await {
        Ok(nbytes) if nbytes == MESSAGE_SIZE_PART_SIZE => message_size_part_buffer,
        Ok(nbytes) if nbytes == 0 => return Err(MessageHeaderRecivingError::ReaderClosed),
        Ok(nbytes) => return Err(MessageHeaderRecivingError::IllegalLength(nbytes)),

        Err(err) => return Err(MessageHeaderRecivingError::ReaderErrorWrapper(err)),
    };

    return Ok(u32::from_be_bytes(raw_message_size));
}

pub async fn read_message_part<AR: AsyncReadExt + Unpin>(
    reader: &mut AR,
) -> Result<(usize, [u8; MESSAGE_PART_SIZE]), MessagePartRecivingError> {
    let mut message_part_buffer: [u8; MESSAGE_PART_SIZE] = [0u8; MESSAGE_PART_SIZE];

    match reader.read(&mut message_part_buffer).await {
        Ok(nbytes) if (0 < nbytes) && (nbytes <= MESSAGE_SIZE_PART_SIZE) => {
            return Ok((nbytes, message_part_buffer))
        }
        Ok(nbytes) if nbytes == 0 => return Err(MessagePartRecivingError::ReaderClosed),
        Ok(nbytes) => return Err(MessagePartRecivingError::IllegalLength(nbytes)),

        Err(err) => return Err(MessagePartRecivingError::ReaderErrorWrapper(err)),
    }
}
