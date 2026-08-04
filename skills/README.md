# Agent skills

Tool-agnostic reference documents for LLM coding agents working with this
project. Plain Markdown — no vendor-specific runtime is required to read them.

| Skill | For | Start at |
| ----- | --- | -------- |
| [`egui-shadcn`](egui-shadcn/) | **Using** the library in an application: dependency setup, bootstrap, the component catalog, theming, i18n. | [`SKILL.md`](egui-shadcn/SKILL.md) |
| [`egui-shadcn-contributing`](egui-shadcn-contributing/) | **Working on** the library: workspace layout, adding a component, design invariants, testing, build pipeline. | [`SKILL.md`](egui-shadcn-contributing/SKILL.md) |

Each `SKILL.md` is the entry point and stays short; the bulk lives in
`references/` next to it and is meant to be read on demand rather than up front.

## Wiring it into a tool

Every skill folder is self-contained, so "installing" it is a copy or a symlink.

**Claude Code** discovers skills in `.claude/skills/` (project) or
`~/.claude/skills/` (all projects). Symlink rather than copy, so there is one
source of truth:

```sh
# this project only
mkdir -p .claude/skills && ln -s ../../skills/egui-shadcn .claude/skills/egui-shadcn

# or every project on the machine
mkdir -p ~/.claude/skills
ln -s "$PWD/skills/egui-shadcn" ~/.claude/skills/egui-shadcn
```

**Other agents** (Codex, Cursor, Kimi, GLM, Gemini, …) generally read an
instruction file at the repository root. [`AGENTS.md`](../AGENTS.md) points here,
so an agent that honours that convention finds these documents without any extra
configuration. For anything else, pass the relevant `SKILL.md` as context.

## Frontmatter

Each `SKILL.md` opens with YAML frontmatter:

```yaml
---
name: egui-shadcn
description: <when this skill applies, with trigger terms>
---
```

Claude Code uses `description` to decide when to load the skill. Other tools
ignore the block harmlessly — it is valid Markdown front matter, not a runtime
dependency.

## Keeping them true

These documents state real signatures and real constraints. When a public API
changes, update the skill in the same commit — a skill that lies is worse than
no skill, because an agent will trust it over reading the source.
