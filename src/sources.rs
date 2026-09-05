//! Generation of a project's source tree under `./src`.
//!
//! The C++ entry point (`main.cpp`) is always written and is backend-agnostic:
//! when a GPU backend is enabled it simply calls `run_kernel()`, which each
//! backend's kernel file (`kernel.cu` / `kernel.hip`) defines. This keeps the
//! entry point shareable across the two targets produced when both CUDA and HIP
//! are requested. Requested runtime dependencies (MPI, Kokkos) are initialized
//! and finalized around the body of `main` in a fixed order.

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::features::{Backend, Features};

const KERNEL_CU: &str = include_str!("../files/cuda/kernel.cu");
const KERNEL_HIP: &str = include_str!("../files/hip/kernel.hip");

/// Write the source files implied by `features` into `./src`, creating the
/// directory if needed. Existing files are never overwritten.
pub fn write_sources(features: &Features) {
    let src_dir = Path::new("./src");
    fs::create_dir_all(src_dir).expect("Failed to create src directory");

    write_if_absent(&src_dir.join("main.cpp"), &render_main_cpp(features));

    if features.has(Backend::Cuda) {
        write_if_absent(&src_dir.join("kernel.cu"), KERNEL_CU);
    }
    if features.has(Backend::Hip) {
        write_if_absent(&src_dir.join("kernel.hip"), KERNEL_HIP);
    }
}

/// Write `content` to `path` unless a file is already there, so a regenerated
/// project does not clobber the user's edits.
fn write_if_absent(path: &Path, content: &str) {
    if path.exists() {
        return;
    }
    let mut file = File::create(path).expect("Failed to create source file");
    file.write_all(content.as_bytes())
        .expect("Failed to write source file");
}

/// Build the contents of `main.cpp` for the requested features. The entry
/// point brings dependencies up in a fixed order — `MPI_Init`, then
/// `Kokkos::initialize` — calls `run_kernel()` when a GPU backend is enabled,
/// and tears everything down in reverse order (`Kokkos::finalize`, then
/// `MPI_Finalize`) so MPI stays alive underneath MPI-aware Kokkos paths.
fn render_main_cpp(features: &Features) -> String {
    let has_backend = !features.backends.is_empty();

    let mut out = String::from("#include <iostream>\n");
    if features.mpi {
        out.push_str("#include <mpi.h>\n");
    }
    if features.kokkos {
        out.push_str("#include <Kokkos_Core.hpp>\n");
    }

    if has_backend {
        out.push_str("\n// Defined in the GPU kernel source (kernel.cu / kernel.hip).\n");
        out.push_str("void run_kernel();\n");
    }

    out.push_str("\nint main(int argc, char* argv[]) {\n");

    if features.mpi {
        out.push_str("    MPI_Init(&argc, &argv);\n");
    }
    if features.kokkos {
        if features.mpi {
            out.push('\n');
        }
        out.push_str("    Kokkos::initialize(argc, argv);\n");
    }
    if features.mpi || features.kokkos {
        out.push('\n');
    }

    if features.mpi {
        out.push_str("    int rank = 0;\n");
        out.push_str("    int size = 0;\n");
        out.push_str("    MPI_Comm_rank(MPI_COMM_WORLD, &rank);\n");
        out.push_str("    MPI_Comm_size(MPI_COMM_WORLD, &size);\n\n");
        out.push_str(
            "    std::cout << \"Hello from rank \" << rank << \" of \" << size << std::endl;\n",
        );
    } else {
        out.push_str("    std::cout << \"Hello, World!\" << std::endl;\n");
    }

    if has_backend {
        out.push_str("    run_kernel();\n");
    }

    if features.mpi || features.kokkos {
        out.push('\n');
    }
    if features.kokkos {
        out.push_str("    Kokkos::finalize();\n");
    }
    if features.mpi {
        out.push_str("    MPI_Finalize();\n");
    }

    out.push_str("    return 0;\n");
    out.push_str("}\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::in_temp_dir;

    fn feats(mpi: bool, kokkos: bool, cuda: bool, hip: bool) -> Features {
        let mut backends = Vec::new();
        if cuda {
            backends.push(Backend::Cuda);
        }
        if hip {
            backends.push(Backend::Hip);
        }
        Features {
            mpi,
            kokkos,
            backends,
        }
    }

    #[test]
    fn plain_writes_only_main_cpp() {
        in_temp_dir(|| {
            write_sources(&feats(false, false, false, false));
            let main = fs::read_to_string("./src/main.cpp").unwrap();
            assert!(main.contains("Hello, World!"));
            assert!(!main.contains("run_kernel"));
            assert!(!Path::new("./src/kernel.cu").exists());
            assert!(!Path::new("./src/kernel.hip").exists());
        });
    }

    #[test]
    fn mpi_entry_initializes_mpi() {
        in_temp_dir(|| {
            write_sources(&feats(true, false, false, false));
            let main = fs::read_to_string("./src/main.cpp").unwrap();
            assert!(main.contains("MPI_Init"));
            assert!(main.contains("MPI_Finalize"));
            assert!(!main.contains("run_kernel"));
        });
    }

    #[test]
    fn cuda_writes_kernel_and_calls_it() {
        in_temp_dir(|| {
            write_sources(&feats(false, false, true, false));
            let main = fs::read_to_string("./src/main.cpp").unwrap();
            assert!(main.contains("run_kernel();"));
            let kernel = fs::read_to_string("./src/kernel.cu").unwrap();
            assert!(kernel.contains("__global__"));
            assert!(!Path::new("./src/kernel.hip").exists());
        });
    }

    #[test]
    fn hip_writes_kernel_and_calls_it() {
        in_temp_dir(|| {
            write_sources(&feats(false, false, false, true));
            let main = fs::read_to_string("./src/main.cpp").unwrap();
            assert!(main.contains("run_kernel();"));
            let kernel = fs::read_to_string("./src/kernel.hip").unwrap();
            assert!(kernel.contains("__global__"));
            assert!(!Path::new("./src/kernel.cu").exists());
        });
    }

    #[test]
    fn cuda_and_hip_write_both_kernels_with_shared_entry() {
        in_temp_dir(|| {
            write_sources(&feats(false, false, true, true));
            let main = fs::read_to_string("./src/main.cpp").unwrap();
            // The indented form is the call site (the declaration is unindented).
            assert_eq!(main.matches("    run_kernel();").count(), 1);
            assert!(Path::new("./src/kernel.cu").exists());
            assert!(Path::new("./src/kernel.hip").exists());
        });
    }

    #[test]
    fn existing_files_are_preserved() {
        in_temp_dir(|| {
            write_sources(&feats(false, false, true, false));
            fs::write("./src/main.cpp", "custom entry").unwrap();
            fs::write("./src/kernel.cu", "custom kernel").unwrap();
            write_sources(&feats(false, false, true, false));
            assert_eq!(
                fs::read_to_string("./src/main.cpp").unwrap(),
                "custom entry"
            );
            assert_eq!(
                fs::read_to_string("./src/kernel.cu").unwrap(),
                "custom kernel"
            );
        });
    }

    #[test]
    fn mpi_cuda_entry_does_both() {
        let main = render_main_cpp(&feats(true, false, true, false));
        assert!(main.contains("MPI_Init"));
        assert!(main.contains("run_kernel();"));
    }

    #[test]
    fn kokkos_entry_initializes_and_finalizes_kokkos() {
        let main = render_main_cpp(&feats(false, true, false, false));
        assert!(main.contains("#include <Kokkos_Core.hpp>"));
        assert!(main.contains("Kokkos::initialize(argc, argv);"));
        assert!(main.contains("Kokkos::finalize();"));
        assert!(!main.contains("MPI_Init"));
        assert!(!main.contains("MPI_Finalize"));
    }

    #[test]
    fn mpi_and_kokkos_lifecycle_is_properly_ordered() {
        // Mixed-dependency lifecycle: MPI_Init -> Kokkos::initialize -> ...
        // -> Kokkos::finalize -> MPI_Finalize, so MPI stays alive underneath
        // any MPI-aware Kokkos path in both directions.
        let main = render_main_cpp(&feats(true, true, true, false));
        let mpi_init = main.find("MPI_Init").expect("missing MPI_Init");
        let kokkos_init = main
            .find("Kokkos::initialize")
            .expect("missing Kokkos::initialize");
        let kokkos_finalize = main
            .find("Kokkos::finalize")
            .expect("missing Kokkos::finalize");
        let mpi_finalize = main.find("MPI_Finalize").expect("missing MPI_Finalize");
        assert!(mpi_init < kokkos_init);
        assert!(kokkos_init < kokkos_finalize);
        assert!(kokkos_finalize < mpi_finalize);
    }
}
