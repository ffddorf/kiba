use serde::{Deserialize, Serialize};

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
    pub detail: GeneralStatus,
    pub imagebuilder_status: BuilderStatus,
    pub request: super::build_req::BuildRequest,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub build_cmd: Option<String>,
}
