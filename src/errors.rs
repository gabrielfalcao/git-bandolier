use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    IOError(String),
    RuntimeError(String),

    RegexError(String),

    SerdeJsonError(String),

    SerdeYamlError(String),

    TomlError(String),

    NomError(String),

    PestError(String),

    TokioError(String),
    Git2Error(String),

    AxumError(String),

    ZeromqError(String),

    SharedmarkError(String),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.variant(),
            match self {
                Error::IOError(e) => e.to_string(),
                Error::RuntimeError(e) => e.to_string(),

                Error::RegexError(e) => e.to_string(),

                Error::SerdeJsonError(e) => e.to_string(),

                Error::SerdeYamlError(e) => e.to_string(),

                Error::TomlError(e) => e.to_string(),

                Error::NomError(e) => e.to_string(),

                Error::PestError(e) => e.to_string(),

                Error::TokioError(e) => e.to_string(),
                Error::Git2Error(e) => e.to_string(),

                Error::AxumError(e) => e.to_string(),

                Error::ZeromqError(e) => e.to_string(),

                Error::SharedmarkError(e) => e.to_string(),
            }
        )
    }
}

impl Error {
    pub fn variant(&self) -> String {
        match self {
            Error::IOError(_) => "IOError",
            Error::RuntimeError(_) => "RuntimeError",

            Error::RegexError(_) => "RegexError",

            Error::SerdeJsonError(_) => "SerdeJsonError",

            Error::SerdeYamlError(_) => "SerdeYamlError",

            Error::TomlError(_) => "TomlError",

            Error::NomError(_) => "NomError",

            Error::PestError(_) => "PestError",

            Error::TokioError(_) => "TokioError",
            Error::Git2Error(_) => "Git2Error",

            Error::AxumError(_) => "AxumError",

            Error::ZeromqError(_) => "ZeromqError",

            Error::SharedmarkError(_) => "SharedmarkError",
        }
        .to_string()
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e.to_string())
    }
}
impl From<iocore::Error> for Error {
    fn from(e: iocore::Error) -> Self {
        Error::IOError(e.to_string())
    }
}
impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Self {
        Error::Git2Error(e.to_string())
    }
}
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Exit {
    Success,
    Error(Error),
}
impl std::process::Termination for Exit {
    fn report(self) -> std::process::ExitCode {
        match &self {
            Exit::Success => std::process::ExitCode::from(0),
            Exit::Error(error) => {
                eprintln!("{}", error);
                std::process::ExitCode::from(1)
            },
        }
    }
}
impl<T> From<std::result::Result<T, Error>> for Exit {
    fn from(result: std::result::Result<T, Error>) -> Exit {
        match result {
            Ok(_) => Exit::Success,
            Err(e) => Exit::Error(e),
        }
    }
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = name.strip_suffix("::f").unwrap();
        name
    }};
}
#[macro_export]
macro_rules! traceback {
    ($variant:ident, $error:expr ) => {{
        let name = $crate::function_name!();
        $crate::Error::$variant(format!("{} [{}:[{}:{}]]\n", $error, name, file!(), line!()))
    }};
    ($variant:ident, $format:literal, $arg:expr  ) => {{
        $crate::traceback!($variant, format!($format, $arg))
    }};
    ($variant:ident, $format:literal, $( $arg:expr ),* ) => {{
        $crate::traceback!($variant, format!($format, $($arg,)*))
    }};
}
