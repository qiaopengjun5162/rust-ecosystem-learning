# Contributing Guide

Thank you for your interest in contributing to **rust-ecosystem-learning**! We welcome contributions from everyone, whether you're fixing a typo, adding a new example, or improving documentation. Your help makes this repository better for everyone in the Rust community.

Please take a moment to review these guidelines to ensure a smooth and collaborative contribution process.

---

## 🛠️ How to Contribute

### 1. **Fork the Repository**

- Click the "Fork" button at the top right of the repository page to create your own copy of the project.

### 2. **Clone Your Fork**

   ```bash
   git clone https://github.com/your-username/rust-ecosystem-learning.git
   cd rust-ecosystem-learning
   ```

### 3. **Create a New Branch**

- Create a branch for your changes. Use a descriptive name that reflects the purpose of your contribution.

   ```bash
   git checkout -b your-branch-name
   ```

### 4. **Make Your Changes**

- Add your changes, whether it's fixing a bug, adding a new example, or improving documentation.
- Follow the repository's coding and formatting conventions (see below).

### 5. **Test Your Changes**

- Ensure your changes work as expected and don't introduce new issues.
- If applicable, add tests to verify your changes.

### 6. **Commit Your Changes**

- Write clear and concise commit messages. Use the present tense and follow the format:

     ```bash
     type(scope): brief description
     ```

     Example:

     ```bash
     docs(examples): add async programming example
     ```

- Common commit types include:
  - `feat`: A new feature or example.
  - `fix`: A bug fix.
  - `docs`: Documentation changes.
  - `chore`: Maintenance or tooling changes.

### 7. **Push Your Changes**

   ```bash
   git push origin your-branch-name
   ```

### 8. **Open a Pull Request (PR)**

- Go to the original repository and click "New Pull Request."
- Provide a clear title and description for your PR, explaining what changes you made and why.
- Reference any related issues (e.g., "Closes #123").

---

## 🧑‍💻 Contribution Guidelines

### Code Style

- Follow Rust's official formatting guidelines using `rustfmt`. Run the following command to format your code:

  ```bash
  cargo fmt
  ```

- Use `clippy` to ensure your code adheres to best practices:

  ```bash
  cargo clippy
  ```

### Documentation

- Add comments to your code where necessary to explain complex logic or decisions.
- Update the README or relevant documentation files if your changes introduce new features or examples.

### Testing

- If your contribution includes code changes, ensure that existing tests pass and add new tests if applicable.
- Run tests using:

  ```bash
  cargo test
  ```

### Issues

- If you find a bug or have a feature request, open an issue on GitHub. Provide as much detail as possible, including steps to reproduce the issue or a description of the proposed feature.

---

## 🚀 First-Time Contributors

If you're new to open-source contributions, don't worry! We're here to help. Look for issues labeled **"good first issue"** to find beginner-friendly tasks. If you have any questions, feel free to ask in the issue comments or reach out to the maintainers.

---

## 📜 Code of Conduct

We are committed to fostering a welcoming and inclusive community. Please review our [Code of Conduct](CODE_OF_CONDUCT.md) to understand the standards of behavior we expect from all contributors.

---

## 🙏 Thank You

Your contributions help make this repository a valuable resource for the Rust community. We appreciate your time and effort! If you have any questions or need assistance, feel free to reach out.

Happy coding! 🦀
