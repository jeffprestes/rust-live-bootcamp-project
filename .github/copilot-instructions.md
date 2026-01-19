# GitHub Copilot Instructions

This file contains instructions for GitHub Copilot to follow when generating code for this project.

## Project Context
- **Project Name:** Live Bootcamp Project
- **Languages:** Rust
- **Frameworks:** Axum, Tokio, Askama
- **Docker:** Used for containerization

## Coding Style
- Use idiomatic Rust code.
- Prefer `unwrap()` only in examples or main functions; handle errors properly in library code.
- Follow standard formatting (rustfmt).

## Documentation
- Add comments for complex logic.
- Update README.md when adding new features.

## Commit Messages
- Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.
- Format: `<type>[optional scope]: <description>`
- Common types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`.
