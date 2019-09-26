use std::ffi::CString;

use libmilter_sys::smfi_setconn;
use std::os::raw::c_int;

enum MilterFlags {
    AddHeaders = 1,
    ChangeBody = 2,
    AddRecipient = 4,
    DeleteRecipient = 8,
    ChangeHeaders = 16,
    Quarantine = 32,
    ChangeFrom = 64,
    AddRecipientEsmtp = 128,
    SetSymbolsList = 256,
}

enum MilterProtocolSteps {
    NoConnect = 1,
    NoHelo = 2,
    NoMail = 4,
    NoRcpt = 8,
    NoBody = 16,
    NoHeaders = 32,
    NoEndOfHeaders = 64,
    NoReplyHeaders = 128,
    NoUnknown = 256,
    NoData = 512,
    Skip = 1024,
    RecipientRejects = 2048,
    NoReplyConnect = 4096,
    NoReplyHelo = 8192,
    NoReplyMail = 16384,
    NoReplyRcpt = 32768,
    NoReplyData = 65536,
    NoReplyUnknown = 131072,
    NoReplyEndOfHeaders = 262144,
    NoReplyBody = 524288,
    HeadersWithLeadingSpace = 1048576,
}

enum MilterNegotiateResponse {
    Continue = 0,
    Reject = 1,
    AllOptions = 10,
}

enum MilterResponse {
    Continue = 0,
    Reject = 1,
    Discard = 2,
    Accept = 3,
    TempFail = 4,
    NoReply = 7,
    Skip = 8,
}

enum MiResponse {
    Success = 0,
    Failure = -1,
}


pub fn set_socket(socket: &str) -> Result<(), &str> {
    let socket = CString::new(socket).unwrap();
    let socket = socket.into_raw();

    let result = unsafe { smfi_setconn(socket) };
    unsafe { CString::from_raw(socket); };

    if result == MiResponse::Failure as c_int {
        Err("Failed to set socket connection")
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
