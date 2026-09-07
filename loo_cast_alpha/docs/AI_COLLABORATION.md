# AI Collaboration

Use this prompt when starting a new AI-assisted repo task.

```text
I want to use the repo collaboration workflow for this task, with approval gates.

Issue/PR:
#<number>

Workflow:

1. I name the issue or PR.
2. You inspect it with `gh`.
3. You summarize what it asks for and wait for my approval.
4. After approval, you inspect the relevant code/docs locally.
5. You summarize the relevant local context and propose the scoped change.
6. You wait for my approval before editing.
7. After approval, you make the change on a branch.
8. You ask before running validation.
9. After approval, you run the relevant validation.
10. You give me a commit title/body and PR title/body.
11. You ask before opening or updating a PR.
12. The PR links the issue and records evidence.

Rules:

- Do not make code or docs edits before I approve the proposed change.
- Do not run validation before I approve it.
- Do not open, update, merge, or close PRs/issues without approval.
- Keep the work scoped to the named issue or PR.
- Use existing repo conventions.
- Prefer GitHub issue numbers and labels over title prefixes or planning IDs.
- For multi-line GitHub CLI content (issue/PR bodies, comments, code/text blocks), write content to a temporary file and use `--body-file`/`-F`; do not rely on escaped `\n` strings.

Task:

Let’s address #<number>.

Start by inspecting it with `gh`, then summarize what it asks for and wait.
```
