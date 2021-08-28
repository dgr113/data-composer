use std::io::{ Error as IOError };
use std::fmt::{ Display, Formatter, Result as FmtResult };

use serde::{ Serialize, Deserialize };
use serde_json::Error as SerdeJsonError;
use serde_yaml::Error as SerdeYamlError;
use data_finder::errors::ApiError as FinderApiError;
use data_getter::errors::ApiError as GetterApiError;




#[derive(Debug, Serialize, Deserialize)]
pub enum ApiError {
    EnvVarError( String ),
    SimpleMsgError( String ),
    SerdeError( String ),
    IOError( String ),
    ConfigError( String ),
    FinderApiError( String ),
    GetterApiError( String ),
    IndexError
}

impl From<SerdeJsonError> for ApiError {
    fn from( err: SerdeJsonError ) -> ApiError {
        ApiError::SerdeError( err.to_string() )
    }
}
impl From<SerdeYamlError> for ApiError {
    fn from( err: SerdeYamlError ) -> ApiError {
        ApiError::SerdeError( err.to_string() )
    }
}
impl From<IOError> for ApiError {
    fn from( err: IOError ) -> ApiError {
        ApiError::IOError( err.to_string() )
    }
}
impl From<FinderApiError> for ApiError {
    fn from( err: FinderApiError ) -> ApiError {
        ApiError::FinderApiError( err.to_string() )
    }
}
impl From<GetterApiError> for ApiError {
    fn from( err: GetterApiError ) -> ApiError {
        ApiError::GetterApiError( err.to_string() )
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let msg = match self {
            Self::EnvVarError( t ) => format!("EnvVarError: {}", t.to_string()),
            Self::SimpleMsgError( t ) => format!("SimpleMsgError: {}", t.to_string()),
            Self::IOError( t ) => format!("IOError: {}", t.to_string()),
            Self::ConfigError( t ) => format!("ConfigError: {}", t.to_string()),
            Self::SerdeError( t ) => format!("SessionDependencyError: {}", t.to_string()),
            Self::FinderApiError( t ) => format!("FinderApiError: {}", t.to_string()),
            Self::GetterApiError( t ) => format!("GetterApiError: {}", t.to_string()),
            Self::IndexError => format!( "IndexGettingError" )
        };
        write!(f, "{}", msg)
    }
}
