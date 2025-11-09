use std::sync::Arc;

use axum::{
    Json,
    extract::{Extension, State},
    response::IntoResponse,
};

use super::State as APIState;
use crate::{model::build_req::BuildRequest, storage::Storage};

/// Principle for a build api request:
/// - receive POST request
/// - build hash of relevant fields
/// - lookup cached image build -> return
/// - lookup existing job by label -> return
/// - start new job -> return

/// Job execution:
/// - create configmap with
///     - signature pubkeys for repos
///     - repos
///     - defaults
/// - determine image
/// - schedule job
///     - `make manifest PROFILE={build_request.profile} PACKAGES={' '.join(build_cmd_packages)} STRIP_ABI=1`
///     - build command:
///         - `make image PROFILE={build_request.profile} PACKAGES={' '.join(build_cmd_packages)} EXTRA_IMAGE_NAME={packages_hash[:12]} BIN_DIR=/builder/{request_hash}`
///         - for defaults: `FILES={bin_dir}/files`
///         - for custom rootfs size: `ROOTFS_PARTSIZE={build_request.rootfs_size_mb}`
/// - copy built image to some storage

pub async fn create<S: Storage>(
    State(state): State<Arc<APIState<S>>>,
    Json(req): Json<BuildRequest>,
) -> impl IntoResponse {
    if let Err(_err) = req.validate() {
        // return bad request error
        todo!()
    }

    let hash = req.req_hash();
    let stored_build = state.storage.get(hash);

    // - lookup cached image build -> return
    // - lookup existing job by label -> return
    // - start new job -> return

    todo!();
    ""
}
