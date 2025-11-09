use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Deserialize, Serialize)]
pub struct BuildRequest {
    /// This parameter is currently optional since no other
    /// distributions are supported.
    pub distro: String, // default to "openwrt"

    /// It is recommended to always upgrade to the latest version,
    /// however it is possible to request older images for testing.
    pub version: String,

    /// It is possible to send the expected revision.  This allows to
    /// show the revision within clients before the request. If the
    /// resulting firmware is a different revision, the build results
    /// in an error.
    pub version_code: Option<String>,

    /// Target name
    pub target: String,

    /// The ImageBuilder `PROFILE`.  Can be found with `ubus call
    /// system board` as the `board_name` value.
    pub profile: String,

    /// List of packages, either *additional* or *absolute* depending
    /// of the `diff_packages` parameter.  This is augmented by the
    /// `packages_versions` field, which allow you to additionally
    /// specify the versions of the packages to be installed.
    pub packages: Vec<String>,

    /// A dictionary of package names and versions.  This is an
    /// alternate form of `packages`, in which the expected package
    /// versions are specified for verification after the build has
    /// completed.
    pub packages_versions: Option<HashMap<String, String>>,

    /// This parameter determines if requested packages are seen as
    /// *additional* or *absolute*. If set to `true` the packages are
    /// seen as *absolute* and all default packages outside the
    /// requested packages are removed. \n\n It is possible to brick
    /// devices when requesting an incomplete list with this parameter
    /// enabled since it may remove WiFi drivers or other essential
    /// packages.
    pub diff_packages: bool,

    /// Custom shell script embedded in firmware image to be run
    /// on first boot. This feature might be dropped in the future.
    ///
    /// Input file size is limited to {settings.max_defaults_length}
    /// bytes and cannot be exceeded.
    pub defaults: Option<String>,

    /// Ability to specify a custom `CONFIG_TARGET_ROOTFS_PARTSIZE`
    /// for the resulting image. Attaching this optional parameter
    /// will cause ImageBuilder to build a rootfs with that size
    /// in MB.
    pub rootfs_size_mb: Option<u64>,

    /// Additional repositories for user packages.
    pub repositories: HashMap<String, String>,

    /// Verfication keys for the additional repositories.
    pub repository_keys: Vec<String>,

    /// Client name and version that requests the image
    pub client: Option<String>,
}

impl Default for BuildRequest {
    fn default() -> Self {
        Self {
            distro: "openwrt".into(),
            version: String::new(),
            version_code: None,
            target: String::new(),
            profile: String::new(),
            packages: Vec::new(),
            packages_versions: None,
            diff_packages: false,
            defaults: None,
            rootfs_size_mb: None,
            repositories: HashMap::new(),
            repository_keys: Vec::new(),
            client: None,
        }
    }
}

impl BuildRequest {
    pub fn validate(&self) -> Result<(), ()> {
        todo!()
    }

    pub fn req_hash(&self) -> String {
        let mut h = Sha256::new();

        h.update(&self.distro);
        h.update(&self.version);
        if let Some(vc) = &self.version_code {
            h.update(vc);
        }
        h.update(&self.target);
        h.update(&self.profile);
        h.update(&self.diff_packages.to_string());
        if let Some(defaults) = &self.defaults {
            h.update(defaults);
        }
        if let Some(rootfs_size) = self.rootfs_size_mb {
            h.update(rootfs_size.to_string());
        }

        // add list of packages
        match &self.packages_versions {
            Some(pv) => write_set(&mut h, prep_hashmap(pv).iter(), true),
            _ => write_set(&mut h, self.packages.iter(), true),
        };
        write_set(&mut h, prep_hashmap(&self.repositories).iter(), false);
        write_set(&mut h, self.repository_keys.iter(), false);

        format!("{:x}", h.finalize())
    }
}

fn prep_hashmap(hm: &'_ HashMap<String, String>) -> Vec<String> {
    hm.iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<String>>()
}

fn write_set<'a, I: AsRef<str> + 'a>(
    hash: &mut Sha256,
    set: impl Iterator<Item = &'a I>,
    strip_prefix: bool,
) {
    let mut items: Vec<&str> = if strip_prefix {
        set.map(|i| i.as_ref().strip_prefix("+").unwrap_or_else(|| i.as_ref()))
            .collect()
    } else {
        set.map(AsRef::as_ref).collect()
    };
    items.sort();
    for item in items {
        hash.update(item);
    }
}
