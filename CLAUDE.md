# CLAUDE.md - AI Assistant Guide for VoxAI

**Last Updated**: 2025-11-14
**Repository**: theo-stocchetti/voxai
**Status**: New Repository - Initial Setup Phase

---

## Table of Contents

1. [Project Overview](#project-overview)
2. [Repository Structure](#repository-structure)
3. [Development Workflows](#development-workflows)
4. [Code Conventions & Standards](#code-conventions--standards)
5. [AI Assistant Guidelines](#ai-assistant-guidelines)
6. [Testing Strategy](#testing-strategy)
7. [Deployment & Infrastructure](#deployment--infrastructure)
8. [Common Tasks & Patterns](#common-tasks--patterns)
9. [Troubleshooting](#troubleshooting)

---

## Project Overview

### About VoxAI

**Status**: This is a new repository currently in the initial setup phase.

**Purpose**: [To be documented as the project develops]

**Key Technologies**: [To be documented]

**Architecture**: [To be documented]

### Project Goals

- [Goal 1 - To be defined]
- [Goal 2 - To be defined]
- [Goal 3 - To be defined]

---

## Repository Structure

Currently, the repository is in its initial state. The following structure is recommended for future development:

```
voxai/
├── src/                    # Source code
│   ├── api/               # API endpoints and routes
│   ├── core/              # Core business logic
│   ├── models/            # Data models and schemas
│   ├── services/          # Service layer
│   ├── utils/             # Utility functions
│   └── config/            # Configuration files
├── tests/                 # Test files
│   ├── unit/             # Unit tests
│   ├── integration/      # Integration tests
│   └── e2e/              # End-to-end tests
├── docs/                  # Documentation
├── scripts/               # Build and deployment scripts
├── .github/               # GitHub workflows and templates
├── CLAUDE.md             # This file
├── README.md             # Project README
└── package.json          # Project dependencies (if Node.js)
```

### Key Directories

**To be updated as the project structure develops**

---

## Development Workflows

### Branch Strategy

**Main Branch**: [To be determined - typically `main` or `master`]

**Feature Branches**: Use the following naming convention:
- `claude/[feature-name]-[session-id]` - For AI-assisted development
- `feature/[feature-name]` - For human-led feature development
- `bugfix/[bug-description]` - For bug fixes
- `hotfix/[issue]` - For urgent production fixes

**Current Working Branch**: `claude/claude-md-mhy5cd8x53r818fa-011Pm5UebLDmzNzS3y7Djcpx`

### Git Workflow

1. **Create Branch**: Always create feature branches from the main branch
2. **Develop**: Make atomic commits with clear, descriptive messages
3. **Test**: Ensure all tests pass before committing
4. **Push**: Use `git push -u origin <branch-name>` for first push
5. **Pull Request**: Create PR with comprehensive description and test plan

### Commit Message Convention

Follow conventional commits format:

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(api): add user authentication endpoint

Implements JWT-based authentication with refresh tokens.
Includes middleware for protected routes.

Closes #123
```

---

## Code Conventions & Standards

### General Principles

1. **Clarity over Cleverness**: Write code that is easy to understand
2. **DRY (Don't Repeat Yourself)**: Extract common patterns into reusable functions
3. **SOLID Principles**: Follow SOLID design principles
4. **Security First**: Never commit secrets, API keys, or sensitive data
5. **Error Handling**: Always handle errors gracefully with appropriate logging

### Naming Conventions

**To be defined based on project language/framework**

- **Variables**: `camelCase` or `snake_case`
- **Functions**: `camelCase` or `snake_case`
- **Classes**: `PascalCase`
- **Constants**: `UPPER_SNAKE_CASE`
- **Files**: `kebab-case.ext` or `snake_case.ext`

### Code Style

**To be defined - common options**:
- Use a linter (ESLint, Pylint, etc.)
- Use a formatter (Prettier, Black, etc.)
- Maximum line length: 80-120 characters
- Use meaningful variable names
- Add comments for complex logic

### Security Guidelines

**CRITICAL - Always Follow These Rules**:

1. **Never commit**:
   - API keys, tokens, or credentials
   - `.env` files with real secrets
   - Private keys or certificates
   - Database passwords
   - OAuth client secrets

2. **Use environment variables** for all configuration
3. **Validate all inputs** to prevent injection attacks
4. **Sanitize user data** before display (prevent XSS)
5. **Use parameterized queries** to prevent SQL injection
6. **Keep dependencies updated** to patch security vulnerabilities
7. **Implement proper authentication and authorization**
8. **Use HTTPS** for all production endpoints

---

## AI Assistant Guidelines

### When Working on This Codebase

1. **Always Read First**: Use the `Read` tool to examine existing files before making changes
2. **Prefer Editing**: Always edit existing files rather than creating new ones when possible
3. **Use TodoWrite**: For complex tasks, use the TodoWrite tool to plan and track progress
4. **Parallel Operations**: Run independent operations in parallel for efficiency
5. **Context Awareness**: Understand the full context before making changes
6. **Test Changes**: Verify changes work before committing

### Task Planning

For complex tasks:
1. Use TodoWrite to create a task list
2. Break down large tasks into smaller, manageable steps
3. Mark tasks as `in_progress` before starting
4. Mark as `completed` immediately after finishing
5. Only one task should be `in_progress` at a time

### Code Exploration

When exploring the codebase:
- Use the `Task` tool with `subagent_type=Explore` for open-ended searches
- Use `Grep` for specific pattern searches
- Use `Glob` for finding files by name pattern
- Use `Read` for examining specific files

### Making Changes

1. **Research**: Understand existing patterns in the codebase
2. **Plan**: Use TodoWrite for multi-step changes
3. **Implement**: Make changes following established conventions
4. **Test**: Run tests to verify changes
5. **Review**: Check for security issues, code quality
6. **Commit**: Create atomic commits with clear messages
7. **Push**: Push to the designated branch

### Communication

- **Be concise**: Keep responses short and to the point
- **No emojis**: Unless explicitly requested by the user
- **Professional tone**: Focus on technical accuracy
- **Show progress**: Use TodoWrite to give visibility into progress
- **Reference code**: Use `file:line` format for code references

### Common Pitfalls to Avoid

1. **Don't** create files unnecessarily
2. **Don't** use bash commands for file operations (use Read/Edit/Write tools)
3. **Don't** guess at URLs or make assumptions
4. **Don't** commit without testing
5. **Don't** push to main/master directly
6. **Don't** include secrets in code or commits
7. **Don't** forget to update documentation when making changes

---

## Testing Strategy

### Test Philosophy

**To be defined based on project needs**

- Aim for high test coverage (80%+ recommended)
- Write tests before fixing bugs (TDD for bug fixes)
- Test edge cases and error conditions
- Keep tests fast and independent

### Test Types

1. **Unit Tests**: Test individual functions/methods
2. **Integration Tests**: Test component interactions
3. **End-to-End Tests**: Test complete user workflows
4. **Performance Tests**: Test system performance under load

### Running Tests

**To be documented when test framework is chosen**

```bash
# Example commands (to be updated)
# Run all tests
npm test

# Run specific test file
npm test path/to/test

# Run with coverage
npm test -- --coverage
```

---

## Deployment & Infrastructure

### Environments

**To be defined**:
- **Development**: Local development environment
- **Staging**: Pre-production testing environment
- **Production**: Live production environment

### CI/CD Pipeline

**To be configured in .github/workflows/**

Recommended pipeline stages:
1. Lint and format check
2. Run tests
3. Build application
4. Security scanning
5. Deploy to staging
6. Deploy to production (manual approval)

### Configuration Management

- Use environment variables for configuration
- Store secrets in secure secret management system
- Document all required environment variables
- Provide `.env.example` template

---

## Common Tasks & Patterns

### Adding a New Feature

1. Create feature branch: `git checkout -b feature/feature-name`
2. Plan the feature using TodoWrite
3. Implement following code conventions
4. Write tests for the new feature
5. Update documentation
6. Create pull request

### Fixing a Bug

1. Write a failing test that reproduces the bug
2. Fix the bug
3. Verify the test passes
4. Check for similar bugs elsewhere
5. Commit with clear description

### Refactoring Code

1. Ensure existing tests pass
2. Make refactoring changes
3. Verify tests still pass
4. Update documentation if needed
5. Commit with `refactor:` prefix

### Adding Dependencies

1. Research the dependency (license, maintenance, security)
2. Add to package.json/requirements.txt/etc.
3. Document why the dependency is needed
4. Lock to specific version
5. Update documentation

---

## Troubleshooting

### Common Issues

**To be documented as issues are encountered**

#### Issue: [Problem Description]
**Symptoms**: [What you see]
**Solution**: [How to fix]

---

## Project-Specific Notes

### Technology Stack

**To be documented**:
- Programming Language: [TBD]
- Framework: [TBD]
- Database: [TBD]
- Key Libraries: [TBD]

### External Services

**To be documented**:
- APIs: [TBD]
- Third-party services: [TBD]

### Performance Considerations

**To be documented**:
- Performance targets
- Known bottlenecks
- Optimization strategies

---

## Additional Resources

### Documentation Links

- Project README: [To be created]
- API Documentation: [To be created]
- Architecture Decision Records: [To be created]

### External Resources

- [Project-specific resources to be added]

---

## Updating This Document

This document should be updated whenever:
- Project structure changes significantly
- New conventions are adopted
- New patterns emerge in the codebase
- Technology stack changes
- Common issues and solutions are discovered

**How to Update**:
1. Make changes to CLAUDE.md
2. Update the "Last Updated" date at the top
3. Add a note in git commit describing what was updated
4. Ensure changes are reviewed as part of PR process

---

## Version History

| Date       | Changes                                    | Updated By |
|------------|-------------------------------------------|------------|
| 2025-11-14 | Initial CLAUDE.md creation for new repository | AI Assistant |

---

**Note**: This is a living document that will evolve as the VoxAI project develops. AI assistants should consult this file before beginning work and update it as the project grows.
