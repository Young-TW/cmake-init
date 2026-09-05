# cmake-init - Initialize CMake project at speed

cmake-init is a command-line tool written in Rust that helps you quickly set up a CMake project with various options such as CUDA, HIP, MPI, and Kokkos support.

![CI/CD](https://github.com/Young-TW/cmake-init/actions/workflows/rust.yml/badge.svg)

## Installation

### Via Cargo

```sh
cargo install cmake-init
```

### Build from source

```sh
git clone https://github.com/Young-TW/cmake-init.git
cd cmake-init
cargo install --path .
```

## Usage

Initialize a simple CMake project with the specified name and options.

```sh
cmake-init <PROJECT_NAME>
```

Initialize a CMake project with CUDA/HIP support:

```sh
cmake-init <PROJECT_NAME> --cuda
```

or

```sh
cmake-init <PROJECT_NAME> --hip
```

If you use CUDA/HIP, you need to edit the `CMakeLists.txt` file to set the architecture code for your GPU.

### Flags

`--cuda`, `--hip` and `--mpi` are composable; see [Backend combinations](#backend-combinations) below.

| Flag             | Short  | Default  | Description                                       |
| ---------------- | ------ | -------- | ------------------------------------------------- |
| `--help`         | `-h`   |          | Show help message                                 |
| `--version`      | `-v`   |          | Show version information                          |
| `<PROJECT_NAME>` |        | required | Project name                                      |
| `--cxx-std`      | `-s`   | `17`     | C++ standard (11, 14, 17, 20, 23, 26)             |
| `--gitignore`    | `-g`   | `true`   | Generate `.gitignore` file (`true`/`false`)       |
| `--cuda`         | `-c`   | `false`  | Enable CUDA support (`true`/`false`)              |
| `--hip`          | `-i`   | `false`  | Enable HIP support (`true`/`false`)               |
| `--mpi`          | `-m`   | `false`  | Enable OpenMPI support (`true`/`false`)           |
| `--kokkos`       | `-k`   | `false`  | Enable Kokkos support (requires C++17+)           |
| `--git`          |        | `true`   | Initialize a Git repository (`true`/`false`)      |

### Backend combinations

`--cuda`, `--hip` and `--mpi` can be combined freely. The generated project
always has a backend-agnostic C++ entry point (`src/main.cpp`); each GPU backend
adds its own kernel file (`src/kernel.cu` / `src/kernel.hip`) that defines
`run_kernel()`. MPI links every target against `MPI::MPI_CXX`.

Because CUDA and HIP cannot share a single target, enabling both produces two
executables; otherwise a single executable named after the project is built.

| Flags                  | Targets                                            |
| ---------------------- | -------------------------------------------------- |
| *(none)*               | `name` (C++)                                        |
| `--cuda`               | `name` (C++ + CUDA kernel)                          |
| `--hip`                | `name` (C++ + HIP kernel)                           |
| `--mpi`                | `name` (C++, MPI-linked)                            |
| `--mpi --cuda`         | `name` (CUDA, MPI-linked)                           |
| `--mpi --hip`          | `name` (HIP, MPI-linked)                            |
| `--cuda --hip`         | `name_cuda`, `name_hip`                             |
| `--mpi --cuda --hip`   | `name_cuda`, `name_hip` (both MPI-linked)           |

Enabling HIP raises the generated `cmake_minimum_required` to 3.21 (the first
CMake release with first-class HIP language support).

### Kokkos

`--kokkos` (`-k`) is an orthogonal dependency like `--mpi`: it does not change
the number of executables. The generated `CMakeLists.txt` adds
`find_package(Kokkos REQUIRED)` and links `Kokkos::kokkos` into every target,
and the generated `main.cpp` brings dependencies up and down in a fixed order
so Kokkos always runs on top of a live MPI:

```cpp
MPI_Init(&argc, &argv);
Kokkos::initialize(argc, argv);
/* ... */
Kokkos::finalize();
MPI_Finalize();
```

Notes:

- Kokkos 4.x requires C++17 or later, so combining `--kokkos` with
  `--cxx-std 11`/`14` is rejected up front.
- Enabling Kokkos raises the generated `cmake_minimum_required` to 3.22
  (unless CUDA is also enabled, which needs 3.24).
- Which execution space Kokkos uses (Serial, OpenMP, CUDA, HIP, ...) is
  decided by your installed Kokkos build, not by cmake-init.

### Example

```sh
cmake-init my_project -s 20 --gitignore false
```

## Contributing

Please see the [Contributing](./CONTRIBUTING.md) document for details on how to contribute to this project.

## Stats

![Alt](https://repobeats.axiom.co/api/embed/cb91f9d845328a1a35e7c4581ac98e14fd2bb352.svg "Repobeats analytics image")
