use anyhow::anyhow;
use serde::Deserialize;
use tonic::{include_proto, Status};

include_proto!("tvm");

pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("tvm_descriptor");

#[derive(Deserialize)]
pub struct TvmResult<T> {
    pub success: bool,
    pub error: Option<String>,
    #[serde(flatten)]
    pub data: Option<T>,
}

impl<T> From<TvmResult<T>> for anyhow::Result<T>
where
    T: Default,
{
    fn from(value: TvmResult<T>) -> Self {
        if value.success {
            Ok(value.data.unwrap_or_default())
        } else {
            Err(anyhow!(value
                .error
                .unwrap_or("ambiguous response".to_owned())))
        }
    }
}

impl<T> From<TvmResult<T>> for Result<T, Status>
where
    T: Default,
{
    fn from(value: TvmResult<T>) -> Self {
        if value.success {
            Ok(value.data.unwrap_or_default())
        } else {
            Err(Status::internal(
                value.error.unwrap_or("ambiguous response".to_owned()),
            ))
        }
    }
}
