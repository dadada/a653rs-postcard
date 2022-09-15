use apex_rs::bindings::*;
use arrayvec::ArrayVec;

pub enum QueuingRecvError<const MSG_SIZE: MessageSize>
where
    [u8; MSG_SIZE as usize]:,
{
    Apex(apex_rs::prelude::Error),
    Postcard(postcard::Error, ArrayVec<u8, { MSG_SIZE as usize }>),
}

impl<const MSG_SIZE: MessageSize> From<apex_rs::prelude::Error> for QueuingRecvError<MSG_SIZE>
where
    [u8; MSG_SIZE as usize]:,
{
    fn from(e: apex_rs::prelude::Error) -> Self {
        QueuingRecvError::Apex(e)
    }
}

pub enum SamplingRecvError<const MSG_SIZE: MessageSize>
where
    [u8; MSG_SIZE as usize]:,
{
    Apex(apex_rs::prelude::Error),
    Postcard(
        postcard::Error,
        Validity,
        ArrayVec<u8, { MSG_SIZE as usize }>,
    ),
}

impl<const MSG_SIZE: MessageSize> From<apex_rs::prelude::Error> for SamplingRecvError<MSG_SIZE>
where
    [u8; MSG_SIZE as usize]:,
{
    fn from(e: apex_rs::prelude::Error) -> Self {
        SamplingRecvError::Apex(e)
    }
}

pub enum SendError {
    Apex(apex_rs::prelude::Error),
    Postcard(postcard::Error),
}

impl From<apex_rs::prelude::Error> for SendError {
    fn from(e: apex_rs::prelude::Error) -> Self {
        SendError::Apex(e)
    }
}

impl From<postcard::Error> for SendError {
    fn from(e: postcard::Error) -> Self {
        SendError::Postcard(e)
    }
}