# Contributing to meteoslave 🌟

Thank you for considering to contribute to meteoslave! We value your efforts and aim to make the contribution process as smooth and enjoyable as possible. Below you'll find our guidelines which we kindly ask you to follow.

## Git Flow 🔄

We adhere to the Git Flow strategy to streamline our development process and maintain code quality. Here's an outline of our branch system:

- `main`: This is the production branch that remains stable at all times and corresponds to the live state of the project.
- `dev`: Our primary development branch where all the upcoming features are integrated, tested, and reviewed before being released.
- Feature branches: Named with a `feat/` prefix, these are used for specific feature developments or improvements. They branch off from and merge back into `dev`.
- Hotfix branches: With a `hotfix/` prefix, they address urgent issues found in the `main` branch that cannot wait until the next regular release.

Please initiate your work from the correct branch and use Pull Requests to merge changes into `dev`, ensuring a review process for maintaining code quality.

## Commit Message Style Guide ✉️

Clear and consistent commit messages enhance collaboration and tracking project history. Please structure your commit messages as follows:

### Format

```bash
:emoji: type(scope): title (issue ID)
\n
body(option)
\n
footer(option)
```

### Components

- **emoji** (optional): An emoji placed at the beginning of the commit message to provide an immediate visual cue about the commit's intent or nature. It can help to quickly identify the purpose or impact of the changes at a glance.
- **type**: The type of change you're committing. It helps in categorizing changes in the project's history and release notes.
- **scope** (optional): A keyword that specifies the part of the codebase your change affects.
- **title**: A concise description of the changes. Start with a capital letter, don't end with a period, and aim to capture the essence in fewer than 50 characters.
- **body** (optional): A detailed account of what changed and why. More complex changes should have an elaborate body.
- **footer** (optional): References to issue IDs or other relevant information.

### Emojis & Types

- ✨ `feat`: Introduces a new feature.
- 🐛 `fix`: Fixes a bug.
- 🎨 `design`: Changes related to UI/UX.
- 💥 `!BREAKING CHANGE`: Introduces changes that break backward compatibility.
- 🚑 `!HOTFIX`: A critical bug fix requiring immediate attention.
- 🔄 `style`: Cosmetic changes related to code formatting or comments.
- ♻️ `refactor`: Code changes that neither fix a bug nor add a feature.
- 🚀 `perf`: Performance improvements.
- 📝 `docs`: Documentation updates or improvements.
- ✅ `test`: Adding or updating tests.
- 🧹 `chore`: Maintenance tasks that don't modify src or test files.
- 🔀 `rename`: Renaming files, variables, or functions.
- 🗑️ `remove`: Removing code, files, or functionalities.
- ➕ `add`: Adding new files or code sections.

### Footer

- `fixes`: Identifies the issue that gets resolved by the commit.
- `resolves`: Another way to identify issue resolution.
- `ref`: Indicates additional context or resources.
