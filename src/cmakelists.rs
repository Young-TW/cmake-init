//! Programmatic generation of a project's `CMakeLists.txt`.
//!
//! The capability combinations (`mpi` x `cuda` x `hip`) do not map onto a fixed
//! set of static templates, so the file is composed section by section from the
//! requested [`Features`].

use crate::features::{Backend, Features};

/// Render the full `CMakeLists.txt` contents for a project.
pub fn render(project_name: &str, cxx_std: i32, features: &Features) -> String {
    let mut out = String::new();

    // CUDA's `native` architecture detection requires 3.24; HIP as a
    // first-class CMake language requires 3.21; 3.20 is enough otherwise.
    let min_version = if features.has(Backend::Cuda) {
        "3.24.0"
    } else if features.has(Backend::Hip) {
        "3.21.0"
    } else {
        "3.20.0"
    };
    out.push_str(&format!("cmake_minimum_required(VERSION {min_version})\n"));

    let mut languages = vec!["CXX"];
    if features.has(Backend::Cuda) {
        languages.push("CUDA");
    }
    if features.has(Backend::Hip) {
        languages.push("HIP");
    }
    out.push_str(&format!(
        "project({project_name} VERSION 0.0.0 LANGUAGES {})\n\n",
        languages.join(" ")
    ));

    out.push_str(&format!("set(CMAKE_CXX_STANDARD {cxx_std})\n"));
    out.push_str("set(CMAKE_CXX_STANDARD_REQUIRED ON)\n");

    if features.has(Backend::Cuda) {
        out.push_str("\n# Detect the architecture(s) of the GPU(s) present at configure time.\n");
        out.push_str("set(CMAKE_CUDA_ARCHITECTURES native)\n");
    }
    if features.has(Backend::Hip) {
        out.push_str("\n# Set to your GPU's architecture (e.g. gfx1100).\n");
        out.push_str("set(CMAKE_HIP_ARCHITECTURES gfx906)\n");
    }

    if features.mpi {
        out.push_str("\nfind_package(MPI REQUIRED)\n");
    }

    // Glob each language's sources separately so targets can pick the kernel
    // flavour they need while sharing the C++ entry point.
    out.push_str("\nfile(GLOB_RECURSE CXX_SOURCES CONFIGURE_DEPENDS ./src/*.cpp)\n");
    if features.has(Backend::Cuda) {
        out.push_str("file(GLOB_RECURSE CUDA_SOURCES CONFIGURE_DEPENDS ./src/*.cu)\n");
    }
    if features.has(Backend::Hip) {
        out.push_str("file(GLOB_RECURSE HIP_SOURCES CONFIGURE_DEPENDS ./src/*.hip)\n");
    }

    for target in features.targets(project_name) {
        let sources = match target.backend {
            None => "${CXX_SOURCES}",
            Some(Backend::Cuda) => "${CXX_SOURCES} ${CUDA_SOURCES}",
            Some(Backend::Hip) => "${CXX_SOURCES} ${HIP_SOURCES}",
        };
        out.push('\n');
        out.push_str(&format!("add_executable({} {sources})\n", target.name));
        if target.mpi {
            out.push_str(&format!(
                "target_link_libraries({} PRIVATE MPI::MPI_CXX)\n",
                target.name
            ));
        }
        out.push_str(&format!("install(TARGETS {})\n", target.name));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feats(mpi: bool, cuda: bool, hip: bool) -> Features {
        let mut backends = Vec::new();
        if cuda {
            backends.push(Backend::Cuda);
        }
        if hip {
            backends.push(Backend::Hip);
        }
        Features { mpi, backends }
    }

    #[test]
    fn plain_cpp_single_target() {
        let out = render("proj", 17, &feats(false, false, false));
        assert!(out.contains("cmake_minimum_required(VERSION 3.20.0)"));
        assert!(out.contains("project(proj VERSION 0.0.0 LANGUAGES CXX)"));
        assert!(out.contains("set(CMAKE_CXX_STANDARD 17)"));
        assert!(out.contains("add_executable(proj ${CXX_SOURCES})"));
        assert!(out.contains("install(TARGETS proj)"));
        assert!(!out.contains("MPI"));
        assert!(!out.contains("CUDA"));
        assert!(!out.contains("HIP"));
        // No template placeholders survive in generated output.
        assert!(!out.contains("{{"));
    }

    #[test]
    fn mpi_links_the_target() {
        let out = render("proj", 20, &feats(true, false, false));
        assert!(out.contains("find_package(MPI REQUIRED)"));
        assert!(out.contains("target_link_libraries(proj PRIVATE MPI::MPI_CXX)"));
    }

    #[test]
    fn cuda_enables_language_and_globs_kernels() {
        let out = render("proj", 17, &feats(false, true, false));
        assert!(out.contains("LANGUAGES CXX CUDA)"));
        assert!(out.contains("set(CMAKE_CUDA_ARCHITECTURES native)"));
        assert!(out.contains("file(GLOB_RECURSE CUDA_SOURCES CONFIGURE_DEPENDS ./src/*.cu)"));
        assert!(out.contains("add_executable(proj ${CXX_SOURCES} ${CUDA_SOURCES})"));
    }

    // Regression test for GitHub issue #8: CUDA projects hard-coded
    // CMAKE_CUDA_ARCHITECTURES to 80 (Ampere), silently miscompiling on other
    // GPUs. The expected behaviour is `native`, which queries the installed GPU
    // at configure time, with the minimum CMake version bumped to 3.24 (the
    // first release that supports `native`).
    #[test]
    fn cuda_uses_native_architectures() {
        let out = render("proj", 17, &feats(false, true, false));
        assert!(
            out.contains("set(CMAKE_CUDA_ARCHITECTURES native)"),
            "CUDA architectures should default to `native`, not a hard-coded \
             value; generated output was:\n{out}"
        );
        assert!(
            !out.contains("set(CMAKE_CUDA_ARCHITECTURES 80)"),
            "CUDA architectures should not be hard-coded to 80 (Ampere); \
             generated output was:\n{out}"
        );
        assert!(
            out.contains("cmake_minimum_required(VERSION 3.24.0)"),
            "CUDA projects require CMake 3.24+ for `native`; generated output \
             was:\n{out}"
        );
    }

    #[test]
    fn hip_bumps_min_version() {
        let out = render("proj", 17, &feats(false, false, true));
        assert!(out.contains("cmake_minimum_required(VERSION 3.21.0)"));
        assert!(out.contains("LANGUAGES CXX HIP)"));
        assert!(out.contains("set(CMAKE_HIP_ARCHITECTURES gfx906)"));
        assert!(out.contains("add_executable(proj ${CXX_SOURCES} ${HIP_SOURCES})"));
    }

    #[test]
    fn cuda_and_hip_emit_two_suffixed_targets() {
        let out = render("proj", 17, &feats(false, true, true));
        assert!(out.contains("cmake_minimum_required(VERSION 3.24.0)"));
        assert!(out.contains("LANGUAGES CXX CUDA HIP)"));
        assert!(out.contains("add_executable(proj_cuda ${CXX_SOURCES} ${CUDA_SOURCES})"));
        assert!(out.contains("add_executable(proj_hip ${CXX_SOURCES} ${HIP_SOURCES})"));
        assert!(out.contains("install(TARGETS proj_cuda)"));
        assert!(out.contains("install(TARGETS proj_hip)"));
    }

    #[test]
    fn mpi_cuda_hip_links_both_targets() {
        let out = render("proj", 17, &feats(true, true, true));
        assert!(out.contains("target_link_libraries(proj_cuda PRIVATE MPI::MPI_CXX)"));
        assert!(out.contains("target_link_libraries(proj_hip PRIVATE MPI::MPI_CXX)"));
    }

    #[test]
    fn project_name_is_substituted() {
        let out = render("my_app", 23, &feats(false, false, false));
        assert!(out.contains("project(my_app "));
    }
}
