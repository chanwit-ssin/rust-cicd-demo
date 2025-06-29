# Contributing to rust-ping-pong

We welcome contributions to the `rust-ping-pong` project! To ensure a smooth development process and maintain code quality, please follow these guidelines:

## Development Workflow

1.  **Feature Branches:** All new features, bug fixes, or significant changes should be developed on separate branches. Please create a new branch from `main` for your work:
    ```bash
    git checkout main
    git pull origin main
    git checkout -b feature/your-feature-name
    ```

2.  **Code Changes:** Make your changes and commit them with clear, concise commit messages.

3.  **Pull Requests:** Once your changes are complete and tested locally, push your branch and open a Pull Request (PR) to the `main` branch. Ensure your PR description clearly explains the changes and their purpose.

4.  **Code Review:** Your PR will be reviewed by maintainers. Address any feedback or requested changes.

## CI/CD and Releases

Our Continuous Integration/Continuous Deployment (CI/CD) pipeline is managed via GitHub Actions.

-   **`main.yml` (CI):** This workflow runs automated tests and linting checks on every push to the `main` branch and on every Pull Request targeting `main`. All changes merged into `main` must pass these checks.

-   **`deploy.yml` (CD):** This workflow is responsible for deploying the application to Railway. **It is triggered only when a new Git tag is pushed to the repository.**

**Important: How to Create a Release Tag**

To ensure that deployed versions are stable and have passed all CI checks, release tags must *only* be created from the `main` branch after it has successfully passed its CI pipeline.

1.  **Ensure `main` is Up-to-Date and Tested:** Before creating a tag, make sure your local `main` branch is synchronized with the remote `main` and that the latest changes on `main` have passed all CI checks (as verified by the `main.yml` workflow).
    ```bash
    git checkout main
    git pull origin main
    ```

2.  **Create the Tag:** Create an annotated tag from the `main` branch. Use a semantic versioning scheme (e.g., `v1.0.0`, `v1.0.1`, `v1.1.0`).
    ```bash
    git tag -a vX.Y.Z -m "Release vX.Y.Z - Your release notes here"
    ```
    Replace `vX.Y.Z` with the actual version number and the message with relevant release notes.

3.  **Push the Tag:** Push the newly created tag to the remote repository. This action will trigger the `deploy.yml` workflow.
    ```bash
    git push origin vX.Y.Z
    ```

By following this process, we ensure that every deployed version of `rust-ping-pong` has undergone proper testing and validation.