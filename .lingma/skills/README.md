# LingoMate AI-Driven Development Skills

## Overview
This directory contains specialized AI assistant skills for developing LingoMate - an AI-powered English tutor desktop application. These skills provide context-aware guidance, code templates, and best practices for efficient development.

## Available Skills

### 1. [lingomate-dev](./lingomate-dev/SKILL.md)
**Purpose**: General development skill for LingoMate project
**Use When**: 
- Starting new features
- Needing project context
- Following coding standards
- Understanding architecture

**Key Topics**:
- Project structure
- Technology stack (Tauri, Rust, React, TypeScript)
- Core principles (privacy, performance, accessibility)
- File organization

---

### 2. [tauri-command](./tauri-command/SKILL.md)
**Purpose**: Implementing Tauri commands in Rust backend
**Use When**:
- Adding new backend functionality
- Creating database operations
- Integrating system services (Ollama, TTS, STT)
- Exposing APIs to frontend

**Key Topics**:
- Command structure template
- Error handling patterns
- Database integration
- Event emission
- TypeScript interface generation
- Testing strategies

**Example Usage**:
```
"I need to create a Tauri command to export vocabulary data"
→ Follows step-by-step guide from this skill
```

---

### 3. [react-component](./react-component/SKILL.md)
**Purpose**: Creating React components with TypeScript and Tailwind
**Use When**:
- Building new UI components
- Implementing page layouts
- Adding interactive elements
- Creating reusable component libraries

**Key Topics**:
- Component structure template
- TypeScript interfaces
- Tailwind CSS styling
- Accessibility (ARIA, keyboard navigation)
- State management (useState, useEffect, custom hooks)
- Performance optimization (React.memo, useMemo)
- Testing with React Testing Library

**Example Usage**:
"Create a vocabulary card component with mastery level display"
→ Uses component template and best practices

---

### 4. [prompt-design](./prompt-design/SKILL.md)
**Purpose**: Designing and optimizing AI teaching prompts
**Use When**:
- Creating scenario-specific prompts
- Designing word teaching flows
- Adjusting difficulty levels
- Implementing review strategies
- A/B testing prompt variations

**Key Topics**:
- Heuristic teaching methodology
- Prompt structure templates
- Scenario prompt design (6 scenarios)
- Word teaching protocol
- Spaced repetition integration
- Difficulty adjustments (A1-C2)
- Testing and validation
- A/B testing framework

**Example Usage**:
"Design a prompt for the job interview scenario at B2 level"
→ Follows scenario prompt design guidelines

---

### 5. [database-operations](./database-operations/SKILL.md)
**Purpose**: SQLite database operations with Rust
**Use When**:
- Creating new tables
- Writing SQL queries
- Implementing CRUD operations
- Performing migrations
- Optimizing query performance

**Key Topics**:
- Connection setup
- CRUD operation patterns
- Advanced queries (JOINs, aggregations, FTS)
- Migration system
- Error handling
- Performance optimization (indexing, query plans)
- Security best practices
- Backup and recovery

**Example Usage**:
"Write a query to get learning statistics for a user"
→ Uses aggregation query patterns

---

### 6. [testing](./testing/SKILL.md)
**Purpose**: Comprehensive testing strategies
**Use When**:
- Writing unit tests (Rust/React)
- Creating E2E tests (Playwright)
- Setting up CI/CD pipelines
- Debugging failing tests
- Improving test coverage

**Key Topics**:
- Testing pyramid
- Rust unit testing (cargo test)
- React component testing (Testing Library)
- Custom hook testing
- Zustand store testing
- Playwright E2E tests
- CI/CD integration (GitHub Actions)
- Coverage goals
- Debugging techniques

**Example Usage**:
"Write E2E test for voice conversation flow"
→ Follows Playwright test patterns

---

## How to Use These Skills

### For AI Assistants
When working on LingoMate development tasks:

1. **Identify the task type** (backend, frontend, prompts, database, testing)
2. **Reference the appropriate skill** for context and templates
3. **Follow the step-by-step guides** provided in each skill
4. **Apply best practices** and checklists
5. **Use code templates** as starting points

### For Developers
When seeking AI assistance:

1. **Be specific about the task**: "Create a Tauri command for..."
2. **Mention relevant skill**: "Using tauri-command skill..."
3. **Provide context**: Reference existing code or documentation
4. **Ask for templates**: "Show me the component structure template"
5. **Request best practices**: "What are the security considerations?"

---

## Skill Integration Examples

### Example 1: Adding New Feature
**Task**: Add vocabulary export feature

**Skills Used**:
1. `tauri-command` - Create Rust backend command
2. `database-operations` - Query vocabulary data
3. `react-component` - Build export button UI
4. `testing` - Write tests for new feature

**Workflow**:
```
1. Design API (tauri-command skill)
2. Implement database query (database-operations skill)
3. Create UI component (react-component skill)
4. Write comprehensive tests (testing skill)
```

---

### Example 2: Improving AI Teaching
**Task**: Enhance word teaching quality

**Skills Used**:
1. `prompt-design` - Redesign teaching prompt
2. `testing` - A/B test new prompt
3. `lingomate-dev` - Integrate into system

**Workflow**:
```
1. Analyze current prompt effectiveness
2. Design improved prompt (prompt-design skill)
3. Implement A/B testing (testing skill)
4. Monitor metrics and iterate
```

---

## Project Documentation References

All skills reference these core documents:

- [Project Requirements](../../doc/LingoMate 项目需求文档.MD)
- [Technical Architecture](../../doc/LingoMate 一体化桌面应用 技术架构与实施方案2.0（综合版）.MD)
- [API Specification](../../doc/LingoMate-API接口规范.md)
- [Database Design](../../doc/LingoMate-数据库详细设计.md)
- [UI/UX Guidelines](../../doc/LingoMate-UI-UX设计规范.md)
- [Prompt Engineering](../../doc/LingoMate-Prompt工程规范.md)
- [Testing Plan](../../doc/LingoMate-测试计划与用例.md)
- [Deployment Guide](../../doc/LingoMate-部署与打包指南.md)
- [Error Handling](../../doc/LingoMate-错误处理与日志规范.md)
- [Security & Privacy](../../doc/LingoMate-安全与隐私合规.md)
- [Team Structure](../../doc/LingoMate-团队组织与技能要求.md)

---

## Contributing to Skills

### Adding New Skills
1. Create directory: `.lingma/skills/[skill-name]/`
2. Write `SKILL.md` following this structure:
   - Description
   - When to Use
   - Step-by-step guides
   - Code templates
   - Best practices
   - Examples
   - References
3. Update this README
4. Test with real development tasks

### Updating Existing Skills
1. Review feedback from usage
2. Add missing patterns or examples
3. Update outdated information
4. Improve clarity and organization
5. Keep templates current with codebase

---

## Quick Reference Card

| Task | Primary Skill | Secondary Skills |
|------|---------------|------------------|
| New backend feature | `tauri-command` | `database-operations`, `testing` |
| New UI component | `react-component` | `testing` |
| Prompt optimization | `prompt-design` | `testing` |
| Database schema change | `database-operations` | `tauri-command` |
| Bug fix | `testing` | Relevant domain skill |
| Performance issue | `lingomate-dev` | `database-operations` |
| Security review | `lingomate-dev` | All skills |

---

## Maintenance

**Last Updated**: 2026-05-17
**Maintained By**: LingoMate Development Team
**Review Frequency**: Monthly or after major releases

For questions or improvements, create an issue or PR in the project repository.
