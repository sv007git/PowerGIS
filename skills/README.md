# Agent skills

Skills that teach an external AI agent to **use** PowerGIS, rather than change
PowerGIS's source code (see the repository's `AGENTS.md` for that).

| Skill | What it teaches |
| --- | --- |
| [`powergis/`](powergis/SKILL.md) | Install PowerGIS for the user, register `powergis-cli` as an MCP server, and manage the project already open in the desktop app through its MCP Relay. |

## Install the skill

Copy the entire `powergis` directory, including `references/`, into your agent's
skills directory. From a checkout of this repository, for Claude Code:

```sh
mkdir -p ~/.claude/skills
cp -R skills/powergis ~/.claude/skills/
```

For a project-local skill, use `.claude/skills/` instead. For another runtime,
use its documented skills directory. Start a new agent session so the skill's
frontmatter description can trigger it. If the runtime has no skill loader,
provide `SKILL.md` as instructions and make the reference document available.

Installing the skill does not install PowerGIS or start MCP. The companion
[install-from-agent guide](powergis/references/install-from-agent.md) teaches the
agent to do those steps on the user's machine, including the open-app relay.

## Keeping it accurate

The installation procedure follows [Installation](../docs/installation.md)
and <https://powergis.app/installation/>. MCP setup, tools, access policy and
open-app management follow [MCP Server](../docs/mcp.md), the `--attach` path in
`apps/powergis-cli/src/main.rs`, and the default port in
`crates/assistant/src/relay.rs`. Update the skill with those interfaces; a
private MCP project is not a substitute for managing the open desktop app.
