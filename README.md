
# RSCAN: The Fast and Furious Rust Secret Scanner 🔍
---
**RSCAN** is a lightweight, blazing-fast Rust tool that recursively scans your directories and files to uncover sensitive secrets like AWS keys, JWT tokens, API keys, and passwords. Don't let your secrets go rogue -- RSCAN helps you find and secure them!

---
## Features

* **Recursive Scanning:** Dives deep into directories and files to leave no secret unturned.
* **Comprehensive Pattern Matching:** Comes packed with multiple regex patterns to catch common secrets right out of the box.
* **Intuitive CLI:** A command-line interface that's super easy to use.
* **Rust-Powered Speed:** Built with Rust for unparalleled performance and efficiency.
* **Sleek Banner:** A cool startup banner to kick off your scanning.
  
![rscan](/rscan/rscan.png)
---
## Installation

Getting RSCAN up and running is a breeze!

1.  **Install Rust and Cargo:** If you don't have them yet, head over to the [official Rust installation guide](https://rust-lang.org/tools/install) and follow the instructions.
2.  **Clone the Repository:** Grab the latest version of RSCAN from GitHub:

    ```bash
    git clone https://github.com/jagdishtripathy/rscan.git
    cd rscan
    ```

3.  **Build the Release Version:** Compile RSCAN for optimal performance:

    ```bash
    cargo build --release
    ```

---
## Usage

Once installed, you can start scanning with a simple command:

```bash
./target/release/rscan <directory_or_file_path_to_scan>

```

**Example:**

Bash

```
./target/release/rscan /home/jagdish/projects/

```

* * * * *

Supported Secret Patterns
-------------------------

RSCAN is pre-configured to detect a variety of common secrets:

-   AWS Access Keys
-   AWS Secret Keys
-   JWT Tokens
-   Generic API Keys
-   Passwords embedded in text

*(Need to find something specific? You can easily customize the regex patterns to suit your needs!)*

* * * * *

Sample Output
-------------

Here's a peek at what RSCAN looks like in action:

```
██████╗ ███████╗ ██████╗ █████╗ ███╗  ██╗
██╔══██╗██╔════╝██╔════╝██╔══██╗████╗ ██║
██████╔╝█████╗  ██║     ███████║██╔██╗██║
██╔═══╝ ██╔══╝  ██║     ██╔══██║██║╚██╗██║
██║     ███████╗╚██████╗██║ ██║██║ ╚████║
╚═╝     ╚══════╝ ╚═════╝╚═╝ ╚═╝╚═╝  ╚═══╝
          🔍  Rust Secret Scanner - RSCAN

Found [AWS Access Key] in file /path/to/file.txt at line 42: AKIAIOSFODNN7EXAMPLE

```

* * * * *

License
-------

RSCAN is open-source and distributed under the **MIT License** © Jagadish Tripathy.

* * * * *

Contact
-------

For any questions, feedback, or contributions, please visit the [RSCAN GitHub repository](https://github.com/jagdishtripathy).

* * * * *

Happy scanning!
