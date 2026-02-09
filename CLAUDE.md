# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **AI Tools** — a collection of AI prompt templates and a corporate standards system for structured software development, content creation, and design workflows. It is a text-based toolkit (Markdown + YAML), not a software application. There is no build system, package manager, or runtime.

All prompt templates guide AI assistants through a four-phase pipeline:

**Research** (`research.md`) → **Create** (`create-prd.md` / `create-crd.md` / `create-drd.md`) → **Generate** (`generate-tasks.md`) → **Execute** (`execute-tasks.md`)

All outputs are saved to `/tasks/` with versioned filenames (e.g., `rsd-auth-v1.md`, `prd-auth-v1.md`, `tasks-auth.md`).

## Architecture

### Prompt Templates (root directory)

Each `.md` file in the root is a self-contained prompt template that defines rules, clarifying question formats, and output structure for one workflow phase. The templates are designed to be pasted into (or `@`-referenced by) an AI assistant.

### Standards System (`/standards/`)

The standards system provides organizational guardrails applied during each phase:

- **`standards-manifest.yml`** — Central config mapping each phase to its applicable standards files. Version-controlled (currently v1.0.0).
- **`global/`** — Apply to ALL phases: principles, security/privacy, accessibility, terminology.
- **`domains/`** — Domain-specific, reused across phases: code architecture, content voice, design UI.
- **`phases/`** — Per-phase customizations.
- **`teams/`** — Optional team-specific overlays (currently empty).

Standards files use YAML frontmatter (`id`, `name`, `version`, `owner`, `last_updated`) and classify rules as **Musts** (numbered 1-9, non-negotiable) vs **Shoulds** (numbered 10+, recommendations). Rules use prefix codes for referencing (e.g., `[INT-1]`, `[VOICE-3]`).

### Key Interaction Patterns

- **Pause-and-approve**: During task execution, the AI completes one sub-task, marks it `[x]`, then stops and waits for user confirmation before proceeding.
- **Two-phase task generation**: First generate parent tasks and get user approval ("Go"), then break down into sub-tasks.
- **Clarifying questions**: Research phase asks 4-7 questions; Create phase asks 3-5 questions. Questions use numbered lists with lettered options (A, B, C).
- **Task 0.0**: Task generation always includes "Create feature branch" as the first task unless the user opts out.

## Recommended Tech Stack (from domain standards)

The code standards (`standards/domains/code-internal-architecture.md`) define a specific stack for projects built using this toolkit:

- Next.js (App Router) + TypeScript (strict mode)
- React Server Components by default; `'use client'` only when needed
- Tailwind CSS for styling; shadcn/ui + Radix UI for components
- Zod for validation; React Query for data fetching
- Vitest + React Testing Library for testing

## Known Issue

The `standards-manifest.yml` references `domains/code-architecture.md`, but the actual file is named `domains/code-internal-architecture.md`. If updating the manifest or the file, ensure these stay in sync.
