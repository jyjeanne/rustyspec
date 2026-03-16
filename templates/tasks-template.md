# Tasks: {{ feature_name }}

**Input**: Design documents from `specs/{{ branch_name }}/`
**Prerequisites**: plan.md (required), spec.md (required for user stories)

## Format: `- [ ] T### [P?] [Story?] Description with file path`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story (e.g., US1, US2) — only in story phases

## Phase 1: Setup

- [ ] T001 Create project structure per implementation plan

## Phase 2: Foundational

- [ ] T002 [Prerequisites from plan]

## Phase 3: User Story 1 (Priority: P1)

**Goal**: [From spec]

- [ ] T003 [US1] [Implementation task with file path]

## Phase N: Polish

- [ ] T999 Documentation updates
