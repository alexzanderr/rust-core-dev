//! core stuff for this submodule of core-dev

use lettre::{
    transport::smtp::authentication::Credentials,
    Message,
    SmtpTransport,
    Transport,
    transport::smtp::Error as SmtpError,
    transport::smtp::SmtpTransportBuilder
};
