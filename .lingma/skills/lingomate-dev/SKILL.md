# LingoMate Development Skill

## Description
AI-assisted development skill for LingoMate project - an AI-powered English tutor desktop application built with Tauri, Rust, and React.

## When to Use
- Implementing new features for LingoMate
- Debugging Rust/Tauri backend code
- Creating React components with TypeScript
- Writing database queries for SQLite
- Designing prompts for AI teaching engine
- Setting up CI/CD pipelines
- Writing tests (unit, integration, E2E)

## Project Context
LingoMate is a cross-platform desktop application that provides AI-powered English tutoring through natural conversation. Key technologies:
- **Frontend**: React 18 + TypeScript + Tailwind CSS + Zustand
- **Backend**: Rust + Tauri 2.x
- **AI**: Ollama (Qwen2.5 models)
- **Database**: SQLite (via tauri-plugin-sql)
- **Voice**: System STT + Edge TTS/System TTS

## Core Principles
1. **Privacy First**: All user data stays local, never upload to cloud
2. **Performance**: Must run smoothly on 8GB RAM devices
3. **Accessibility**: WCAG 2.1 AA compliant
4. **Type Safety**: Strict TypeScript and Rust type checking
5. **Error Handling**: Graceful degradation, user-friendly messages

## File Structure Awareness
```
lingomate/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/       # Tauri commands
│   │   ├── services/       # Business logic
│   │   ├── models/         # Data structures
│   │   └── errors.rs       # Error types
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                    # React frontend
│   ├── components/         # UI components
│   ├── pages/              # Page components
│   ├── hooks/              # Custom hooks
│   ├── stores/             # Zustand stores
│   └── utils/              # Utility functions
├── doc/                    # Documentation
└── package.json
```

## Code Generation Guidelines

### Rust Backend
- Use `thiserror` for error handling
- Always handle Result types explicitly
- Use async/await for I/O operations
- Follow Rust naming conventions (snake_case for functions/variables)
- Add comprehensive doc comments for public APIs
- Use `#[tauri::command]` for exposed functions

### React Frontend
- Use functional components with Hooks
- Prefer TypeScript interfaces over types
- Use Tailwind CSS for styling (no inline styles)
- Implement proper error boundaries
- Use semantic HTML elements
- Add ARIA labels for accessibility

### Database Operations
- Use parameterized queries (never string concatenation)
- Handle database errors gracefully
- Implement proper transaction management
- Add indexes for frequently queried fields

### Prompt Engineering
- Keep prompts concise and specific
- Use few-shot examples when needed
- Include clear constraints and rules
- Test with different difficulty levels

## Common Tasks

### Adding a New Tauri Command
1. Define command in `src-tauri/src/commands/`
2. Register in `main.rs`
3. Create TypeScript interface in `src/types/`
4. Add error handling
5. Write unit tests

### Creating a React Component
1. Create component file in `src/components/`
2. Define TypeScript props interface
3. Implement with Tailwind classes
4. Add storybook story (if applicable)
5. Write component tests

### Database Schema Changes
1. Update schema in documentation
2. Create migration script
3. Update Rust models
4. Update TypeScript interfaces
5. Test data integrity

## Testing Requirements
- Unit tests for all Rust functions
- Component tests for React components
- E2E tests for critical user flows
- Performance tests for 8GB devices
- Accessibility audits

## Documentation Standards
- All public APIs must have doc comments
- Complex logic needs inline comments
- Update relevant docs when making changes
- Follow existing documentation style

## Security Checklist
- [ ] No hardcoded secrets
- [ ] Input validation on all user inputs
- [ ] SQL injection prevention (parameterized queries)
- [ ] XSS prevention (proper escaping)
- [ ] Permission requests with clear explanations
- [ ] No sensitive data in logs

## Performance Guidelines
- Lazy load components when possible
- Use virtual scrolling for long lists
- Minimize re-renders with React.memo
- Optimize database queries with indexes
- Monitor memory usage on low-end devices

## References
- [Project Requirements](../../doc/LingoMate 项目需求文档.MD)
- [Technical Architecture](../../doc/LingoMate 一体化桌面应用 技术架构与实施方案2.0（综合版）.MD)
- [API Specification](../../doc/LingoMate-API接口规范.md)
- [Database Design](../../doc/LingoMate-数据库详细设计.md)
- [UI/UX Guidelines](../../doc/LingoMate-UI-UX设计规范.md)
- [Prompt Engineering](../../doc/LingoMate-Prompt工程规范.md)
