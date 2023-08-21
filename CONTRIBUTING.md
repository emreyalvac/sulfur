# Contributing Guide

This document is designed to guide developers who wish to contribute to our project. By contributing, you can help us improve and support our community.

## How to Contribute

1. **Fork the Project**: Start by forking the project to your GitHub account.

2. **Code Edits**: Apply the changes you want to make to the code. This could involve adding features, fixing bugs, or enhancing existing code.

3. **Configuring Changes**: Add a descriptive explanation of your changes, explaining what they accomplish and how they work.

4. **Pull Request (PR)**: Open a Pull Request to integrate your changes into the main project. Remember to provide a clear explanation of your changes.

5. **Feedback and Review**: Project maintainers and other developers will review your changes and provide feedback. Make any necessary adjustments.

6. **Approval and Merging**: Once your changes are approved, they will be merged into the project.

## Setting Up the Development Environment

1. Clone the project:

    ```bash
    git clone https://github.com/emreyalvac/sulfur.git
    ```

2. Install necessary dependencies. For example:

    ```bash
    cd sulfur
    cargo build
    ```

3. Start editing your code!

## Help and Communication

For any questions, suggestions, or assistance, you can email us at [emre.yalvac@outlook.com](mailto:emre.yalvac@outlook.com).

## Adding an Engine

If you want to add a new engine to the project, you can follow these steps:

1. Navigate to the `engine` folder in the project directory.

2. Create a new engine module or update an existing one. Below is an example engine code:

    ```rust
    use async_trait::async_trait;
    // Add other required dependencies
   
    // Example engine module
    pub struct MyEngine {
        // Define engine fields
    }

    // Implement the TEngine trait for MyEngine
    #[async_trait]
    impl TEngine for MyEngine {
        // Implement method definitions
    }
    ```

3. Edit the relevant file in the `engine` folder or create a new one to add your engine to the project.

4. Include examples that explain how to use and configure the new engine.

5. Don't forget to provide explanations for your changes.

6. Create a pull request and submit it for review.

Thank you for your contributions!
