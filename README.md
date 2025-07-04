# cmake-init

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

Currently, not supporting multiple flags in (`--cuda`, `--hip`, `--mpi`) at the same time.

| Flag                | Short  | Default | Description                                         |
|---------------------|--------|---------|-----------------------------------------------------|
| `--help`            | `-h`   |         | Show help message                                   |
| `--version`         | `-v`   |         | Show version information                            |
| `<PROJECT_NAME>`    |        | required| Project name                                        |
| `--cxx-std`         | `-s`   | `17`    | C++ standard (11, 14, 17, 20, 23, 26)               |
| `--gitignore`       | `-g`   | `true`  | Generate `.gitignore` file (`true`/`false`)         |
| `--cuda`            | `-c`   | `false` | Enable CUDA support (`true`/`false`)                |
| `--hip`             | `-i`   | `false` | Enable HIP support (`true`/`false`)                 |
| `--mpi`             | `-m`   | `false` | Enable OpenMPI support (`true`/`false`)             |

### Example

```sh
cmake-init my_project -s 20 --gitignore false
```

## Contributing

Please see the [Contributing](./CONTRIBUTING.md) document for details on how to contribute to this project.

## Stats

![Alt](https://repobeats.axiom.co/api/embed/cb91f9d845328a1a35e7c4581ac98e14fd2bb352.svg "Repobeats analytics image")
