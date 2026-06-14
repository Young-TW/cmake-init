//! Generation of a project's source tree under `./src`.
//!
//! The C++ entry point (`main.cpp`) is always written and is backend-agnostic:
//! when a GPU backend is enabled it simply calls `run_kernel()`, which each
//! backend's kernel file (`kernel.cu` / `kernel.hip`) defines. This keeps the
//! entry point shareable across the two targets produced when both CUDA and HIP
//! are requested.

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::features::Features;

const KERNEL_CU: &str = include_str!("../files/cuda/kernel.cu");
const KERNEL_HIP: &str = include_str!("../files/hip/kernel.hip");

/// Write the source files implied by `features` into `./src`, creating the
/// directory if needed. Existing files are never overwritten.
pub fn write_sources(features: &Features) {
    let src_dir = Path::new("./src");
    fs::create_dir_all(src_dir).expect("Failed to create src directory");

    write_if_absent(&src_dir.join("main.cpp"), &render_main_cpp(features));

    if features.cuda {
        write_if_absent(&src_dir.join("kernel.cu"), KERNEL_CU);
    }
    if features.hip {
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

/// Build the contents of `main.cpp` for the requested features. The entry point
/// initializes MPI when requested and calls `run_kernel()` when a GPU backend
/// is enabled.
fn render_main_cpp(features: &Features) -> String {
    let has_backend = features.cuda || features.hip;

    let mut out = String::from("#include <iostream>\n");
    if features.mpi {
        out.push_str("#include <mpi.h>\n");
    }

    if has_backend {
        out.push_str("\n// Defined in the GPU kernel source (kernel.cu / kernel.hip).\n");
        out.push_str("void run_kernel();\n");
    }

    out.push_str("\nint main(int argc, char* argv[]) {\n");

    if features.mpi {
        out.push_str("    MPI_Init(&argc, &argv);\n\n");
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

    if features.mpi {
        out.push_str("\n    MPI_Finalize();\n");
    }

    out.push_str("    return 0;\n");
    out.push_str("}\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::in_temp_dir;

    fn feats(mpi: bool, cuda: bool, hip: bool) -> Features {
        Features { mpi, cuda, hip }
    }

    #[test]
    fn plain_writes_only_main_cpp() {
        in_temp_dir(|| {
            write_sources(&feats(false, false, false));
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
            write_sources(&feats(true, false, false));
            let main = fs::read_to_string("./src/main.cpp").unwrap();
            assert!(main.contains("MPI_Init"));
            assert!(main.contains("MPI_Finalize"));
            assert!(!main.contains("run_kernel"));
        });
    }

    #[test]
    fn cuda_writes_kernel_and_calls_it() {
        in_temp_dir(|| {
            write_sources(&feats(false, true, false));
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
            write_sources(&feats(false, false, true));
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
            write_sources(&feats(false, true, true));
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
            write_sources(&feats(false, true, false));
            fs::write("./src/main.cpp", "custom entry").unwrap();
            fs::write("./src/kernel.cu", "custom kernel").unwrap();
            write_sources(&feats(false, true, false));
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
        let main = render_main_cpp(&feats(true, true, false));
        assert!(main.contains("MPI_Init"));
        assert!(main.contains("run_kernel();"));
    }
}
