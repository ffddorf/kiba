use std::hash::Hash;

use axum::{Json, response::IntoResponse};
use sha2::{Digest, Sha256};

use super::build_req::BuildRequest;

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

#[axum::debug_handler]
pub async fn create(Json(req): Json<BuildRequest>) -> impl IntoResponse {
    if let Err(_err) = req.validate() {
        // return bad request error
        todo!()
    }

    let hash = req.req_hash();

    // - lookup cached image build -> return
    // - lookup existing job by label -> return
    // - start new job -> return

    todo!();
    ""
}
