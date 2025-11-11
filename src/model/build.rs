use serde::{Deserialize, Serialize};

use crate::utils::serde::chrono_human::CustomDateTime;

#[derive(Deserialize, Serialize)]
pub enum GeneralStatus {
    Init,
    Queued,
    Started,
    Failed,
    Done,
}

#[derive(Deserialize, Serialize)]
pub enum BuilderStatus {
    Init,
    ContainerSetup,
    ValidateRevision,
    ValidateManifest,
    BuildingImage,
    Done,
}

/// Represents a build that has started
#[derive(Deserialize, Serialize)]
pub struct Build {
    pub request: super::build_req::BuildRequest,
    pub request_hash: String,

    pub detail: GeneralStatus,
    pub imagebuilder_status: BuilderStatus,
    pub enqueued_at: Option<CustomDateTime>,

    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub build_cmd: Option<String>,
}
