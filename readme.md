# **Mind Like Water (MLW) \- A Rust-Powered GTD Application**

## **🚀 Project Overview**

This project, "Mind Like Water" (MLW), is an ongoing endeavor to build a personal productivity application inspired by David Allen's **Getting Things Done (GTD)** methodology. Its primary purpose is to serve as a **hands-on learning vehicle for Rust**, allowing me to explore and apply Rust's powerful features in a practical, domain-centric context.

## **✨ Core Concepts & Features**

* **Domain-Centric Model:** The application is built around a rich domain model. The Stake is the core entity, representing any actionable item or area of focus (e.g., areas of responsibility, projects, or individual tasks). StakeId is a strongly-typed identifier. The StakesCollection provides encapsulated, domain-specific operations (like filtering and retrieval) on these entities. The MLW struct acts as the central application aggregate, encapsulating and exposing a clean API for managing distinct collections of areas, projects, and tasks.  
* **Test-Driven Development (TDD):** This project strictly adheres to a TDD workflow. Every new feature or behavior begins with writing a failing unit test, which is then made to pass with minimal code, followed by refactoring. This disciplined approach ensures high code quality, robust design, and confidence in the application's behavior.  
* **Leveraging Rust's Strengths:**  
  * **Performance:** Rust's zero-cost abstractions, fine-grained control over memory, and lack of a garbage collector are being leveraged to build a highly performant application.  
  * **Memory Safety:** Rust's ownership and borrowing system ensures memory safety at compile time, preventing common bugs like null pointer dereferences and data races without runtime overhead.  
  * **Strong Type System:** Rust's powerful type system is used extensively (e.g., newtype patterns for StakeId) to enforce domain invariants, improve code clarity, and catch errors early in the development cycle.  
* **GTD Methodology:** The application's structure and operations are designed to align with GTD principles, focusing on capturing, clarifying, organizing, reflecting, and engaging with work.

## **📐 Architectural Principles**

This project aims to follow principles of **Clean Code** and **Clean Architecture**:

* **Clean Code:** Emphasis is placed on writing code that is readable, maintainable, and understandable. This includes clear naming, consistent formatting, and well-defined responsibilities for each module, struct, and function. The goal is for the code to be its own best documentation.  
* **Clean Architecture:** The domain model (entities like Stake and MLW) is kept independent of external concerns such as specific database implementations, user interface frameworks, or network protocols. This strict separation of concerns ensures the core business logic remains pure, highly testable, and adaptable to changes in external infrastructure without impacting the core domain.

## **🔮 Future Plans**

* **Current State:** The application currently manages all data in-memory within the MLW object.  
* **Persistence:** Implement robust saving and loading mechanisms. Initial plans include using JSON for simple backups, with future exploration into a Rust-native relational database for primary, structured storage.  
* **Networking:** Integrate TCP connections for potential synchronization capabilities or client-server interactions.  
* **CLI/UI:** Develop a command-line interface or a simple graphical user interface to interact with the MLW application.

## **🚀 Getting Started**

To get this project up and running on your local machine:

### **Prerequisites**

* **Rust and Cargo:** If you don't have Rust installed, the recommended way is via rustup:  
  curl \--proto '=https' \--tlsv1.2 \-sSf https://sh.rustup.rs | sh

  Follow the on-screen instructions.

### **Cloning the Repository**

git clone https://github.com/markbroski/mlw-rust
cd mlw-rust

### **Building the Project**

To build the project in debug mode (faster compilation, useful during development):

cargo build

To build the project in release mode (optimized for performance, for benchmarks or deployment):

cargo build \--release

### **Running the Application**

To run the main application (which currently contains demonstration code in src/main.rs):

cargo run

To run the application in release mode:

cargo run \--release

### **Running Tests**

To run all unit tests:

cargo test

To run all unit tests and see println\! or dbg\! output from passing tests (useful for debugging performance tests):

cargo test \-- \--nocapture

To run specific tests (e.g., all tests within the project\_tests module):

cargo test project\_tests::

To run a specific performance test (in release mode, with output):

cargo test \--release test\_performance\_search\_by\_name \-- \--nocapture

To run lint checks (highly recommended for code quality):

cargo clippy

## **📂 Project Structure**

my-life-s-work/  
├── Cargo.toml                  \# Project manifest and dependencies  
├── src/  
│   ├── main.rs                 \# Main application entry point  
│   ├── mlw.rs                  \# The core MLW application entity and its public API  
│   └── entities/  
│       ├── mod.rs              \# Module declarations and re-exports for domain entities  
│       ├── stake.rs            \# Defines the Stake entity (ID, attributes, behavior)  
│       └── stakes\_collection.rs \# Defines the StakesCollection (collection logic, ID generation, serialization)  
└── README.md                   \# This file

## **🤝 Contributing**

Contributions, questions, and feedback are welcome\! Feel free to open issues or submit pull requests.

## **📄 License**

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT). (Please ensure you create a LICENSE file in the root of your repository with the full license text).
