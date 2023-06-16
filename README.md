# makelatex

makelatex is a command-line tool written in Rust that simplifies the creation of LaTeX projects. It allows you to quickly set up a new LaTeX project directory with the necessary folder structure and a Makefile for easy compilation of your LaTeX documents. It compiles your latex project twice to update the table of contents

## Features

- Creates a new LaTeX project directory with organized folder structure
- Generates a basic LaTeX source file with a predefined template
- Provides a Makefile for convenient compilation of the LaTeX document into a PDF
- Separates documentation source files, output logs, and images into dedicated folders

## Requirements

- Rust (version 1.0 or higher)
- pdflatex
- build-essentials

## Installation

1. Clone the repository:

```shell
git clone https://github.com/JustBobinAround/makelatex.git
```

2. Build the project:

```shell
cd makelatex
cargo build --release
```

3. Add the binary to your PATH:

```shell
export PATH="$PATH:/path/to/makelatex/target/release"
```

## Usage

To create a new LaTeX project, run the following command:

```shell
makelatex new <project_name>
```

Replace `<project_name>` with the desired name for your LaTeX project. This command will create a new directory with the specified project name and set up the necessary folder structure inside it.

Once the project is created, navigate to the project directory and use the provided Makefile to compile your LaTeX document. Run the following command:

```shell
make
```

This will compile the LaTeX source file into a PDF using `pdflatex`. The resulting PDF will be saved in the project's output directory.

## Folder Structure

The created LaTeX project directory will have the following structure:

```
<project_name>/
├── doc_out/
├── doc_logs/
├── doc_src/
│   └── <project_name>.tex
├── images/
└── Makefile
```

- `doc_out/`: Output directory for the generated PDF document.
- `doc_logs/`: Directory for storing compilation logs and auxiliary files.
- `doc_src/`: Directory for keeping the LaTeX source files. It contains a single file, `<project_name>.tex`, which serves as the main LaTeX document.
- `images/`: Directory for storing images or figures referenced in the LaTeX document.
- `Makefile`: Makefile for compiling the LaTeX document using `pdflatex`.

## Contribution

Contributions to the makelatex project are welcome! If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0). See the [LICENSE](LICENSE) file for more details.

---

**Disclaimer: This project is still under development. The functionality and features mentioned in this README are subject to changes.**
