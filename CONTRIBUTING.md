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
- ⚗️ `perf`: Performance improvements.
- 📝 `docs`: Documentation updates or improvements.
- ✅ `test`: Adding or updating tests.
- 🧹 `chore`: Maintenance tasks that don't modify src or test files.
- 🔀 `rename`: Renaming files, variables, or functions.
- 🔥 `remove`: Removing code, files, or functionalities.
- ➕ `add`: Adding new dependencies.
- 🔖 `release`: Release or add version tags.
- 🚀 `deploy`: Deploy :)
  
### Footer

- `fixes`: Identifies the issue that gets resolved by the commit.
- `resolves`: Another way to identify issue resolution.
- `ref`: Indicates additional context or resources.

## Code Review Conventions 🕵️

Code reviews are a vital part of our development process, ensuring high-quality code and promoting collaborative learning. Here are our conventions for conducting effective code reviews:

- **Be Respectful**: Offer feedback in a kind, constructive manner. Remember that the goal is to improve the code while fostering a positive team environment.
- **Be Specific**: Refer to specific lines of code where changes are suggested. Provide context and reasoning behind your suggestions to make them more actionable.
- **Ask Questions**: Instead of dictating changes, ask clarifying questions to understand the developer's perspective. This approach can lead to insightful discussions and more informed decisions.
- **Provide Examples**: When suggesting changes, include code snippets or links to relevant resources to illustrate your points clearly.
- **Respond Promptly**: Aim to review pull requests within a designated time frame (e.g., 24-48 hours). Prompt feedback keeps the development process moving and demonstrates respect for your colleagues' time.
- **Check for Consistency**: Ensure that the code adheres to the project's coding standards and conventions. Consistency is key to maintainability and readability.
- **Test Locally**: If possible, test the changes locally to verify that they behave as expected. This can prevent potential issues from being merged into the main branch.
- **Acknowledge Good Work**: Positive reinforcement is as important as constructive feedback. Recognize and commend good practices and solutions in the code review process.

### Key Abbreviations

- `NIT`: Suggests a minor change that could improve the code but isn't mandatory.
- `LGTM` (👍): "Looks Good To Me" indicates approval of the changes.
- `ACK` (👌): "Acknowledgment", often used to signify agreement or approval.
- `NACK`/`NAK` (🚫): "Negative Acknowledgement", used to show disapproval or to reject changes.
- `RFC` (💬): "Request For Comments" invites others to provide feedback.
- `WIP` (🚧): "Work In Progress" indicates that the work is not yet complete.
- `AFAIK`/`AFAICT` (🤔): "As Far As I Know" / "As Far As I Can Tell".
- `IIRC` (🧠): "If I Recall Correctly".
- `IANAL` (⚖️): "I Am Not A Lawyer", used to clarify that an opinion is not legal advice.
- `IMO`/`IMHO` (🤷): "In My Opinion" / "In My Humble Opinion".
- `FYI` (ℹ️): "For Your Information".
- `PTAL` (🔍): "Please Take A Look".
- `SSIA` (📌): "Subject Says It All".
- `TBD` (🤷‍♂️): "To Be Determined".
- `TL;DR` (📝): "Too Long; Didn't Read", often used to summarize lengthy content.
