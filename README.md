# rust-faas-serverless
A dynamic Function-as-a-Service (FaaS) solution built with Rust for learning purposes

Function As A Service (FaaS) in Rust

A dynamic Function-as-a-Service (FaaS) solution built with Rust, leveraging the power of the hyper framework to load and execute functions from user-uploaded dynamic libraries.

Overview

FaaS Server: An efficient and secure web service with endpoints for uploading dynamic libraries and invoking functions they encapsulate. With this architecture, users can run bespoke functions without modifying or recompiling the server.

FaaS Client: A robust command-line interface (CLI) designed to streamline interactions with the FaaS Server. The client simplifies tasks such as uploading libraries and invoking functions.
Getting Started
Prerequisites

   - Rust Programming Language
   - Cargo, Rust's built-in package manager

Installation

   1. Clone the repository:

          git clone https://github.com/[your_username]/faas-rust.git

   2. Change into the project directory and build the project:
      
          cd faas-rust
          cargo build --release

Usage
FaaS Server

Start the server using:

      cargo run --bin server

FaaS Client

To upload a dynamic library:

    cargo run --bin client upload <path_to_dynamic_library>

To invoke a function from the uploaded library:

    cargo run --bin client invoke <function_name> <optional_args>

Contribution

Contributions, issues, and feature requests are welcome! Feel free to check the issues page.
License

Distributed under the MIT License. See LICENSE for more information.

Made by Luis Soares 
