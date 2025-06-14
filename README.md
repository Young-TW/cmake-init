# cmake-init

## Installation

```sh
git clone https://github.com/Young-TW/cmake-init.git
cd cmake-init
cargo install --path .
```

## Usage

```sh
cmake-init <PROJECT_NAME>
```

### Flags

| Flag                | Short  | Default | Description                                         |
|---------------------|--------|---------|-----------------------------------------------------|
| `--help`            | `-h`   |         | Show help message                                   |
| `--version`         | `-v`   |         | Show version information                            |
| `<PROJECT_NAME>`    |        | required| Project name                                        |
| `--cxx-std`         | `-s`   | `17`    | C++ standard (11, 14, 17, 20, 23, 26)               |
| `--gitignore`       | `-g`   | `true`  | Generate `.gitignore` file (`true`/`false`)         |
| `--cuda`            | `-c`   | `false` | Enable CUDA support (`true`/`false`)                |
| `--hip`             | `-i`   | `false` | Enable HIP support (`true`/`false`)                 |

### Example

```sh
cmake-init my_project -s 20 --gitignore false
```
