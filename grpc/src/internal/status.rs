/*!
<https://github.com/grpc/grpc-go/blob/master/internal/status/status.go>
*/

import (
    "github.com/golang/protobuf/proto"
    "github.com/golang/protobuf/ptypes"
)

// use std::error::Error;

use google::rpc::status::Status as StatusProto;
use protobuf::{
    runtime::protoiface::Message
};

use crate::{
    codes::Code,
    internal::
};

/// Status represents an RPC status code, message, and details. It is immutable and should be created with New, Newf, or from_proto.
pub struct Status {
    s: StatusProto
}

impl Status {
    /// Returns a Status representing c and msg.
    pub fn new(code: Code, message: String) -> Self {
        Self { s: StatusProto { code: code as i32, message } }
    }

    /// Returns New(c, fmt.Sprintf(format, a...)).
    pub fn newf(code: Code, format: String, a: ...interface{}) -> Self {
        return New(c, format!(format, a...))
    }

    /// Returns a Status representing s.
    pub fn from_proto(status: &StatusProto) -> Self {
        Self { s: status.clone() }
    }

    /// Returns an error representing c and msg.  If c is OK, returns nil.
    pub fn err(code: Code, message: String) -> error {
        return Self::new(code, message).Err()
    }

    /// Errorf returns Error(c, fmt.Sprintf(format, a...)).
    pub fn Errorf(code: Code, format: String, a: Vec<interface{}>) -> error {
        Self::err(code, format!(format, a...))
    }

    /// Code returns the status code contained in self.
    pub fn code(&self) -> Code {
        self.s.code as Code
    }

    /// Message returns the message contained in self.
    pub fn message(&self) -> String {
        self.s.message
    }

    /// Returns s's status as an spb.Status proto message.
    pub fn proto(&self) -> StatusProto {
        self.s.clone()
    }

    /// Returns an immutable error representing s; returns nil if s.Code() is OK.
    pub fn Err(&self) -> Result<(), Error> {
        match self.code() {
            Code::OK => Ok(()),
            _ => Err(Error { status: self })
        }
    }

    /// Returns a new status with the provided details messages appended to the status.
    /// If any errors are encountered, it returns nil and the first error encountered.
    pub fn with_details(&self, details: Vec<Message>) -> Result<Status> {
        if self.code() == Code::OK {
            return Err("No error details for status with code OK.")
        }
        // self.Code() != OK implies that self.Proto() != nil.
        let p = self.proto();
        for detail in details.iter() {
            match ptypes.MarshalAny(detail) {
                Ok(any) => {
                    p.details = append(p.details, any);
                },
                Err(err) => return Err(err)
            }
            
        }
        return Ok(Status { s: p })
    }

    /// Details returns a slice of details messages attached to the status.
    /// If a detail cannot be decoded, the error is returned in place of the detail.
    pub fn details(&self) -> Vec<interface{}> {
        let details = make([]interface{}, 0, self.s.details.len());
        for any in self.s.details.iter() {
            let mut detail = ptypes.DynamicAny {};
            if let Err(err) = ptypes.UnmarshalAny(any, detail) {
                details = append(details, err);
                continue
            }
            details = append(details, detail.message);
        }
        details
    }

    pub fn string(&self) -> String {
        format!("RPC error: code = {} desc = {}", self.code(), self.message())
    }
}

/// Error wraps a pointer of a status proto. It implements error and Status, and a nil *Error should never be returned by this package.
pub struct Error {
    status: Status
}

impl Error {
    pub fn error(&self) -> String {
        self.status.string()
    }

    /// Returns the Status represented by se.
    pub fn grpc_status(&self) -> &Status {
        &self.status
    }

    /// Is implements future error.Is functionality.
    /// A Error is equivalent if the code and message are identical.
    pub fn is(&self, target: Error) -> bool {
        proto.Equal(self.status.s, target.s.s)
    }
}

/// Returns whether the status includes a code restricted for control plane usage as defined by gRFC A54.
pub fn is_restricted_control_plane_code(status: &Status) -> bool {
    match status.code() {
          Code::InvalidArgument
        | Code::NotFound
        | Code::AlreadyExists
        | Code::FailedPrecondition
        | Code::Aborted
        | Code::OutOfRange
        | Code::DataLoss => true,
        _ => false
    }
}
