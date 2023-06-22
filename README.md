Linux ls Command Implementation in Rust
<img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" alt="rust logo">

This project aims to recreate the familiar ls command found in Linux using the Rust programming language. The goal is to provide a feature-rich and efficient alternative to the standard ls command while leveraging the power and safety of Rust.

Features
Directory listing: Display file and directory names in a specified directory.
Error handling: Provide meaningful error messages and handle exceptional cases gracefully.

Feature To Be Added
Color-coded output: Enhance readability and visual appeal by using colors to differentiate file types.
Sorting options: Sort files and directories based on various criteria such as name, size, or modification time.
Command-line flags: Support different options and flags to customize the behavior of the ls command.
Documentation: Thoroughly document the code to aid maintainability and future contributions.
Performance optimization: Optimize the implementation for efficient execution, even with large directory structures.

Getting Started
To get started with the project, follow these steps:

Clone the repository:

bash
Copy code
git clone https://github.com/truthixify/rust-ls.git
cd ls-rust
Build the project using Cargo:

bash
Copy code
cargo build
Run the ls command:

bash
Copy code
cargo run -- [directory_path]
Replace [directory_path] with the path of the directory you want to list. If no directory is specified, the current directory will be used.

Explore the available command-line options and flags:

bash
Copy code
cargo run -- --help
Contributing
Contributions to this project are welcome! If you'd like to contribute, please follow these steps:

Fork the repository.
Create a new branch for your feature or bug fix.
Make the necessary changes and commit them.
Push your branch to your forked repository.
Submit a pull request to the main repository.
Please ensure that your code follows the project's coding style and conventions. Additionally, consider writing tests for any new functionality or modifications.

License
This project is licensed under the MIT License.

Acknowledgements
This project was inspired by the original ls command found in Linux. We appreciate the efforts of the developers and contributors of the Linux operating system.

Contact
If you have any questions or suggestions regarding this project, please feel free to contact truthixify at <a>truthixify@gmail.com</a>.

Happy coding! 🚀
