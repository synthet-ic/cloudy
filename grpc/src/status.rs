/*!
<https://github.com/grpc/grpc-go/blob/master/status/status.go>
*/

use google::rpc::status as spb;

use crate::{
    codes::Code,
    internal::status
};

/**
Status references google.golang.org/grpc/internal/status. It represents an
RPC status code, message, and details.  It is immutable and should be
created with New, Newf, or from_proto.
<https://godoc.org/google.golang.org/grpc/internal/status>
*/
pub type Status = status.Status;

impl Status {
    /// Returns a Status representing c and msg.
    pub fn new(code: Code, msg: String) -> Self {
        status.New(code, msg)
    }

    /// Returns New(c, fmt.Sprintf(format, a...)).
    pub fn newf(code: Code, format: String, a: Vec<interface{}>) -> Self {
        Self::new(code, format!(format, a...))
    }

    /// Rreturns an error representing c and msg.  If c is OK, returns nil.
    pub fn error(code: Code, msg: String) -> error {
        Self::new(code, msg).Err()
    }

    /// Returns Error(c, fmt.Sprintf(format, a...)).
    pub fn Errorf(code: Code, format: string, a: Vec<interface{}>) -> error {
        return Self::error(code, fmt.Sprintf(format, a...))
    }

    /// Returns an error representing s.  If s.Code is OK, returns nil.
    pub fn error_proto(s: *spb.Status)- > error {
        return from_proto(s).Err()
    }

    // from_proto returns a Status representing s.
    pub fn from_proto(s: *spb.Status) -> Self {
        return status.from_proto(s)
    }

    /**
    from_error returns a Status representation of err.

    - If err was produced by this package or implements the method `GRPCStatus()
    *Status`, the appropriate Status is returned.

    - If err is nil, a Status is returned with codes.OK and no message.

    - Otherwise, err is an error not compatible with this package.  In this
    case, a Status is returned with codes.Unknown and err's Error() message,
    and ok is false.
    */
    pub fn from_error(err: error) -> (s *Status, ok bool) {
        if err == nil {
            return nil, true
        }
        if se, ok = err.(interface {
            GRPCStatus() *Status
        }); ok {
            return se.GRPCStatus(), true
        }
        return New(codes.Unknown, err.Error()), false
    }

    /// convert is a convenience function which removes the need to handle the boolean return value from from_error.
    pub fn convert(err: error) -> Self {
        let s, _ = from_error(err);
        s
    }

    /// Code returns the Code of the error if it is a Status error, codes.OK if err is nil, or codes.Unknown otherwise.
    pub fn code(err: error) -> Code {
        // Don't use from_error to avoid allocation of OK status.
        if err == nil {
            return Code::OK
        }
        if se, ok = err.(interface {
            GRPCStatus() *Status
        }); ok {
            return se.GRPCStatus().Code()
        }
        return Code::Unknown
    }

    /**
    Converts a context error or wrapped context error into a Status. It returns a Status with codes.OK if err is nil, or a Status with codes.Unknown if err is non-nil and not a context error.
    */
    pub fn from_context_error(err: error) -> Self {
        if err == nil {
            return nil
        }
        if errors.Is(err, context.DeadlineExceeded) {
            return New(Code::DeadlineExceeded, err.Error())
        }
        if errors.Is(err, context.Canceled) {
            return New(Code::Cancelled, err.Error())
        }
        return New(Code::Unknown, err.Error())
    }
}
