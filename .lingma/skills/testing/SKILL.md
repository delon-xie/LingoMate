# Testing Skill for LingoMate

## Description
Comprehensive testing skill for LingoMate project covering unit tests, integration tests, and E2E tests with Playwright. Ensures code quality and prevents regressions.

## When to Use
- Writing unit tests for Rust functions
- Creating React component tests
- Implementing E2E test scenarios
- Setting up CI/CD test pipelines
- Debugging failing tests
- Adding test coverage

## Testing Pyramid for LingoMate

```
        /\
       /  \      E2E Tests (Playwright) - 10%
      /____\     Critical user flows only
     /      \    Integration Tests - 20%
    /________\   Tauri Commands, API calls
   /          \  Unit Tests - 70%
  /____________\ Rust functions, React components
```

## Rust Unit Testing

### Test Structure
```rust
// src-tauri/src/services/vocabulary_service.rs

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    
    // Helper function for test setup
    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        // Run migrations
        migrate_database(&conn).unwrap();
        conn
    }
    
    #[test]
    fn test_calculate_next_review_date_level_0() {
        let next_date = calculate_next_review(0);
        let expected = chrono::Utc::now().date_naive();
        
        assert_eq!(next_date, expected);
    }
    
    #[test]
    fn test_update_mastery_level_increase() {
        let mut word = VocabularyItem {
            mastery_level: 2,
            review_count: 3,
            ..Default::default()
        };
        
        update_mastery(&mut word, true);
        
        assert_eq!(word.mastery_level, 3);
        assert_eq!(word.review_count, 4);
    }
    
    #[test]
    fn test_insert_duplicate_word() {
        let db = setup_test_db();
        
        // Insert first time
        insert_vocabulary(&db, "test", None, None).unwrap();
        
        // Insert duplicate (should be ignored or error)
        let result = insert_vocabulary(&db, "test", None, None);
        
        assert!(result.is_err() || result.unwrap() == 0);
    }
}
```

### Running Rust Tests
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_calculate_next_review

# Run with output
cargo test -- --nocapture

# Run with coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Mock External Dependencies
```rust
use mockall::mock;

mock! {
    pub OllamaClient {
        async fn generate(&self, prompt: String) -> Result<String, Error>;
        async fn check_health(&self) -> bool;
    }
}

#[tokio::test]
async fn test_ai_response_with_mock() {
    let mut mock_client = MockOllamaClient::new();
    
    mock_client
        .expect_generate()
        .times(1)
        .returning(|_| Ok("Test response".to_string()));
    
    let result = mock_client.generate("test prompt".to_string()).await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Test response");
}
```

## React Component Testing

### Test Setup
```typescript
// src/components/__tests__/ChatBubble.test.tsx

import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import { ChatBubble } from '../ChatBubble';

describe('ChatBubble', () => {
  test('renders AI message correctly', () => {
    render(
      <ChatBubble 
        role="assistant" 
        content="Hello! How can I help you?" 
      />
    );
    
    expect(screen.getByText('Hello! How can I help you?')).toBeInTheDocument();
    expect(screen.getByRole('img')).toHaveAttribute('aria-label', 'AI avatar');
  });
  
  test('renders user message with different style', () => {
    const { container } = render(
      <ChatBubble 
        role="user" 
        content="I'd like a latte." 
      />
    );
    
    const bubble = container.querySelector('.message-user');
    expect(bubble).toBeInTheDocument();
  });
  
  test('handles word click for teaching', () => {
    const onWordClick = jest.fn();
    
    render(
      <ChatBubble 
        role="assistant" 
        content="That's fascinating!" 
        enableWordClick={true}
        onWordClick={onWordClick}
      />
    );
    
    const word = screen.getByText('fascinating');
    fireEvent.click(word);
    
    expect(onWordClick).toHaveBeenCalledWith('fascinating');
  });
});
```

### Testing Custom Hooks
```typescript
// src/hooks/__tests__/useConversation.test.ts

import { renderHook, act } from '@testing-library/react';
import { useConversation } from '../useConversation';

describe('useConversation', () => {
  test('initializes with empty messages', () => {
    const { result } = renderHook(() => useConversation());
    
    expect(result.current.messages).toEqual([]);
    expect(result.current.isLoading).toBe(false);
  });
  
  test('adds message to conversation', async () => {
    const { result } = renderHook(() => useConversation());
    
    await act(async () => {
      await result.current.addMessage('user', 'Hello');
    });
    
    expect(result.current.messages).toHaveLength(1);
    expect(result.current.messages[0].content).toBe('Hello');
  });
  
  test('handles error when sending message', async () => {
    const { result } = renderHook(() => useConversation());
    
    await act(async () => {
      // Mock failed API call
      jest.spyOn(global, 'fetch').mockRejectedValue(new Error('Network error'));
      
      await expect(result.current.sendMessage('Test'))
        .rejects.toThrow('Network error');
    });
    
    expect(result.current.error).toBeTruthy();
  });
});
```

### Testing with Zustand Store
```typescript
// src/stores/__tests__/appStore.test.ts

import { act } from '@testing-library/react';
import { useAppStore } from '../appStore';

describe('AppStore', () => {
  beforeEach(() => {
    // Reset store before each test
    useAppStore.setState({
      currentSession: null,
      messages: [],
      isRecording: false,
    });
  });
  
  it('updates recording state', () => {
    act(() => {
      useAppStore.getState().setRecording(true);
    });
    
    expect(useAppStore.getState().isRecording).toBe(true);
  });
  
  it('adds message to current session', () => {
    const message = {
      id: 1,
      role: 'user' as const,
      content: 'Test message',
      created_at: new Date().toISOString(),
    };
    
    act(() => {
      useAppStore.getState().addMessage(message);
    });
    
    expect(useAppStore.getState().messages).toContainEqual(message);
  });
});
```

### Running React Tests
```bash
# Run all tests
npm test

# Run in watch mode
npm run test:watch

# Run with coverage
npm run test:coverage

# Run specific test file
npm test -- ChatBubble.test.tsx
```

## E2E Testing with Playwright

### Test Configuration
```typescript
// playwright.config.ts

import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  timeout: 30000,
  retries: process.env.CI ? 2 : 0,
  use: {
    baseURL: 'http://localhost:1420',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { browserName: 'chromium' },
    },
    {
      name: 'webkit',
      use: { browserName: 'webkit' },
    },
  ],
});
```

### Basic E2E Test
```typescript
// tests/e2e/scenario-selection.spec.ts

import { test, expect } from '@playwright/test';

test.describe('Scenario Selection', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });
  
  test('displays all 6 scenarios', async ({ page }) => {
    const scenarios = [
      'Coffee Shop',
      'Restaurant',
      'Hotel Check-in',
      'Job Interview',
      'Airport',
      'Social Gathering',
    ];
    
    for (const scenario of scenarios) {
      await expect(page.getByText(scenario)).toBeVisible();
    }
  });
  
  test('clicking scenario starts conversation', async ({ page }) => {
    await page.click('[data-testid="scenario-coffee-shop"]');
    
    // Should navigate to chat page
    await expect(page).toHaveURL(/\/chat\/\d+/);
    
    // AI should send greeting
    await expect(page.locator('.message-ai').first())
      .toContainText('What can I get for you');
  });
});
```

### Voice Conversation Test
```typescript
// tests/e2e/voice-conversation.spec.ts

import { test, expect } from '@playwright/test';

test('complete voice conversation flow', async ({ page }) => {
  // Select scenario
  await page.goto('/');
  await page.click('[data-testid="scenario-coffee-shop"]');
  
  // Mock STT result
  await page.evaluate(() => {
    (window as any).__MOCK_STT_RESULT__ = "I'd like a latte, please.";
  });
  
  // Simulate recording
  await page.click('.record-button');
  await page.waitForTimeout(1000);
  await page.dispatchEvent('.record-button', 'mouseup');
  
  // Verify recognized text appears
  await expect(page.locator('.message-user'))
    .toContainText("I'd like a latte");
  
  // Wait for AI response
  await page.waitForSelector('.message-ai:last-child');
  
  // Verify AI response contains relevant content
  const aiMessage = await page.locator('.message-ai:last-child').textContent();
  expect(aiMessage).toMatch(/latte|coffee|hot|iced/i);
  
  // Verify audio play button exists
  await expect(page.locator('.audio-play-btn').last()).toBeVisible();
});
```

### Word Teaching Test
```typescript
// tests/e2e/word-teaching.spec.ts

import { test, expect } from '@playwright/test';

test('double-click word triggers teaching', async ({ page }) => {
  // Start conversation
  await page.goto('/chat/1');
  await page.waitForSelector('.message-ai');
  
  // Double-click word
  const word = page.locator('.clickable-word', { hasText: 'fascinating' });
  await word.dblclick();
  
  // Verify word highlighted
  await expect(word).toHaveClass(/selected/);
  
  // Wait for teaching response
  await page.waitForSelector('.message-ai:last-child');
  
  // Verify teaching content
  const teaching = await page.locator('.message-ai:last-child').textContent();
  expect(teaching).toMatch(/means|example|question/i);
  
  // Verify word added to vocabulary
  await page.goto('/vocabulary');
  await expect(page.getByText('fascinating')).toBeVisible();
});
```

### Running E2E Tests
```bash
# Install browsers
npx playwright install

# Run tests in UI mode (interactive)
npx playwright test --ui

# Run tests headless
npx playwright test

# Run specific test file
npx playwright test voice-conversation.spec.ts

# Run on specific browser
npx playwright test --project=chromium

# Generate test report
npx playwright show-report
```

## CI/CD Integration

### GitHub Actions Workflow
```yaml
# .github/workflows/test.yml

name: Test

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test-rust:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run Rust tests
        run: cargo test --verbose
      
      - name: Check code formatting
        run: cargo fmt --check
      
      - name: Run clippy lints
        run: cargo clippy -- -D warnings
  
  test-frontend:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Run unit tests
        run: npm test -- --coverage
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./coverage/lcov.info
  
  test-e2e:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node
        uses: actions/setup-node@v3
        with:
          node-version: 18
      
      - name: Install dependencies
        run: npm ci
      
      - name: Install Playwright browsers
        run: npx playwright install --with-deps
      
      - name: Build application
        run: npm run build
      
      - name: Run E2E tests
        run: npx playwright test
      
      - name: Upload test results
        if: failure()
        uses: actions/upload-artifact@v3
        with:
          name: playwright-report
          path: playwright-report/
```

## Test Coverage Goals

### Minimum Coverage Requirements
- **Rust Backend**: 80% line coverage
- **React Components**: 70% line coverage
- **Critical Paths**: 100% coverage (authentication, payments, data integrity)

### Checking Coverage
```bash
# Rust coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage

# Frontend coverage
npm test -- --coverage
# View report: open coverage/index.html
```

## Common Testing Patterns

### Arrange-Act-Assert Pattern
```typescript
test('adds item to list', () => {
  // Arrange
  const list = new TodoList();
  
  // Act
  list.add('Buy milk');
  
  // Assert
  expect(list.items).toContain('Buy milk');
});
```

### Test Data Builders
```typescript
// tests/fixtures/userBuilder.ts

export function createUser(overrides: Partial<User> = {}): User {
  return {
    id: 'user-123',
    name: 'John Doe',
    email: 'john@example.com',
    level: 'intermediate',
    ...overrides,
  };
}

// Usage in tests
const user = createUser({ name: 'Jane Smith' });
```

### Mocking Tauri APIs
```typescript
// tests/mocks/tauri.ts

import { vi } from 'vitest';

vi.mock('@tauri-apps/api', () => ({
  invoke: vi.fn((command: string, params?: any) => {
    switch (command) {
      case 'get_sessions':
        return Promise.resolve(mockSessions);
      case 'send_message':
        return Promise.resolve({ success: true });
      default:
        return Promise.reject(new Error(`Unknown command: ${command}`));
    }
  }),
  listen: vi.fn(),
}));
```

## Debugging Failing Tests

### Rust Tests
```bash
# Show full output
cargo test -- --nocapture --test-threads=1

# Run single test with debug output
cargo test test_name -- --nocapture

# Use debugger
rust-gdb target/debug/your_binary
```

### React Tests
```typescript
// Add debug output
import { debug } from '@testing-library/react';

test('debug example', () => {
  const { container } = render(<MyComponent />);
  debug(container); // Prints DOM structure
});
```

### Playwright Tests
```bash
# Run with debug mode
PWDEBUG=1 npx playwright test

# Show browser during test
npx playwright test --headed

# Pause execution
await page.pause(); # In test code
```

## Best Practices

### ✅ DO
- Write tests before fixing bugs (regression prevention)
- Use descriptive test names
- Keep tests independent and isolated
- Mock external dependencies
- Test edge cases and error conditions
- Update tests when requirements change
- Run tests locally before pushing

### ❌ DON'T
- Don't test implementation details
- Don't write tests that depend on execution order
- Don't skip testing error paths
- Don't hardcode test data (use builders)
- Don't ignore flaky tests
- Don't commit failing tests
- Don't test third-party libraries

## Test Checklist Before Release

- [ ] All unit tests passing
- [ ] All integration tests passing
- [ ] All E2E tests passing
- [ ] Code coverage meets minimum threshold
- [ ] No linting errors
- [ ] Performance tests within acceptable range
- [ ] Accessibility tests passing
- [ ] Cross-browser tests passing
- [ ] Manual smoke test completed

## References
- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [React Testing Library](https://testing-library.com/docs/react-testing-library/intro/)
- [Playwright Documentation](https://playwright.dev/)
- [Project Test Plan](../../doc/LingoMate-测试计划与用例.md)
