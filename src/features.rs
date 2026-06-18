//! Composable project capabilities and the build targets they imply.
//!
//! The CLI exposes `--mpi`, `--cuda` and `--hip` as independent toggles. The
//! set of enabled GPU backends decides how many executables a project needs,
//! while MPI is an orthogonal modifier applied to every target.

/// A GPU backend that compiles its own kernel sources into a target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// NVIDIA CUDA backend.
    Cuda,
    /// AMD HIP backend.
    Hip,
}

impl Backend {
    /// Suffix appended to the executable name when more than one backend is
    /// built (e.g. `myproj_cuda`).
    pub fn suffix(self) -> &'static str {
        match self {
            Backend::Cuda => "cuda",
            Backend::Hip => "hip",
        }
    }

    /// File extension used for this backend's kernel sources.
    pub fn kernel_ext(self) -> &'static str {
        match self {
            Backend::Cuda => "cu",
            Backend::Hip => "hip",
        }
    }

    /// CMake language name enabled for this backend.
    pub fn cmake_lang(self) -> &'static str {
        match self {
            Backend::Cuda => "CUDA",
            Backend::Hip => "HIP",
        }
    }
}

/// The capabilities requested for a project.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Features {
    /// Whether every target links against MPI.
    pub mpi: bool,
    /// The GPU backends to build, in a stable canonical order (CUDA before
    /// HIP).
    pub backends: Vec<Backend>,
}

/// A single executable the generated CMake project should build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    /// Executable name. Equals the project name unless two backends are built,
    /// in which case each target is suffixed with its backend name.
    pub name: String,
    /// The GPU backend compiled into this target, or `None` for plain C++.
    pub backend: Option<Backend>,
    /// Whether this target links against MPI.
    pub mpi: bool,
}

impl Features {
    /// Whether `backend` is enabled for this project.
    pub fn has(&self, backend: Backend) -> bool {
        self.backends.contains(&backend)
    }

    /// Derive the executables to build for `project_name`.
    ///
    /// * no backend or exactly one backend → a single target named after the
    ///   project,
    /// * both CUDA and HIP → two targets, one per backend, each suffixed with
    ///   its backend name (CUDA and HIP cannot share a target).
    ///
    /// MPI, when enabled, applies to every target.
    pub fn targets(&self, project_name: &str) -> Vec<Target> {
        match self.backends.as_slice() {
            [] => vec![Target {
                name: project_name.to_string(),
                backend: None,
                mpi: self.mpi,
            }],
            [single] => vec![Target {
                name: project_name.to_string(),
                backend: Some(*single),
                mpi: self.mpi,
            }],
            _ => self
                .backends
                .iter()
                .map(|backend| Target {
                    name: format!("{project_name}_{}", backend.suffix()),
                    backend: Some(*backend),
                    mpi: self.mpi,
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(name: &str, backend: Option<Backend>, mpi: bool) -> Target {
        Target {
            name: name.to_string(),
            backend,
            mpi,
        }
    }

    #[test]
    fn plain_cpp_single_target() {
        let f = Features::default();
        assert_eq!(f.targets("proj"), vec![t("proj", None, false)]);
    }

    #[test]
    fn mpi_only_links_single_cpp_target() {
        let f = Features {
            mpi: true,
            ..Default::default()
        };
        assert_eq!(f.targets("proj"), vec![t("proj", None, true)]);
    }

    #[test]
    fn single_backend_keeps_project_name() {
        let cuda = Features {
            backends: vec![Backend::Cuda],
            ..Default::default()
        };
        assert_eq!(
            cuda.targets("proj"),
            vec![t("proj", Some(Backend::Cuda), false)]
        );

        let hip = Features {
            backends: vec![Backend::Hip],
            ..Default::default()
        };
        assert_eq!(
            hip.targets("proj"),
            vec![t("proj", Some(Backend::Hip), false)]
        );
    }

    #[test]
    fn mpi_modifies_single_backend_target() {
        let f = Features {
            mpi: true,
            backends: vec![Backend::Cuda],
        };
        assert_eq!(
            f.targets("proj"),
            vec![t("proj", Some(Backend::Cuda), true)]
        );
    }

    #[test]
    fn cuda_and_hip_split_into_two_suffixed_targets() {
        let f = Features {
            backends: vec![Backend::Cuda, Backend::Hip],
            ..Default::default()
        };
        assert_eq!(
            f.targets("proj"),
            vec![
                t("proj_cuda", Some(Backend::Cuda), false),
                t("proj_hip", Some(Backend::Hip), false),
            ]
        );
    }

    #[test]
    fn mpi_cuda_hip_yields_two_mpi_aware_targets() {
        let f = Features {
            mpi: true,
            backends: vec![Backend::Cuda, Backend::Hip],
        };
        assert_eq!(
            f.targets("proj"),
            vec![
                t("proj_cuda", Some(Backend::Cuda), true),
                t("proj_hip", Some(Backend::Hip), true),
            ]
        );
    }

    #[test]
    fn has_reports_enabled_backends() {
        let f = Features {
            backends: vec![Backend::Cuda],
            ..Default::default()
        };
        assert!(f.has(Backend::Cuda));
        assert!(!f.has(Backend::Hip));
    }
}
