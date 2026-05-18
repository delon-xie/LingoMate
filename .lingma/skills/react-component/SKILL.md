# React Component Development Skill

## Description
Specialized skill for creating React components in LingoMate frontend with TypeScript, Tailwind CSS, and accessibility best practices.

## When to Use
- Creating new UI components
- Implementing page layouts
- Adding interactive elements
- Building forms and inputs
- Creating reusable component libraries

## Component Structure Template

```tsx
// src/components/[ComponentName].tsx

import React from 'react';
import { cn } from '@/lib/utils';

// 1. Define TypeScript interfaces
export interface [ComponentName]Props {
  // Required props
  title: string;
  
  // Optional props with defaults
  variant?: 'default' | 'primary' | 'secondary';
  size?: 'sm' | 'md' | 'lg';
  
  // Event handlers
  onClick?: () => void;
  
  // Children
  children?: React.ReactNode;
  
  // Accessibility
  'aria-label'?: string;
}

// 2. Main component with destructured props
export function [ComponentName]({
  title,
  variant = 'default',
  size = 'md',
  onClick,
  children,
  'aria-label': ariaLabel,
}: [ComponentName]Props) {
  // 3. Component logic (hooks, state, effects)
  
  // 4. Render with semantic HTML and Tailwind classes
  return (
    <div
      className={cn(
        // Base styles
        'rounded-lg transition-all duration-200',
        
        // Variant styles
        {
          'bg-white border border-gray-200': variant === 'default',
          'bg-primary-500 text-white': variant === 'primary',
          'bg-gray-100': variant === 'secondary',
        },
        
        // Size styles
        {
          'p-2 text-sm': size === 'sm',
          'p-4 text-base': size === 'md',
          'p-6 text-lg': size === 'lg',
        }
      )}
      onClick={onClick}
      role="button"
      tabIndex={0}
      aria-label={ariaLabel || title}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          onClick?.();
        }
      }}
    >
      <h3 className="font-semibold mb-2">{title}</h3>
      {children}
    </div>
  );
}

// 5. Display name for debugging
[ComponentName].displayName = '[ComponentName]';
```

## Step-by-Step Implementation

### Step 1: Plan Component Structure
- Identify component purpose and responsibilities
- List required and optional props
- Determine state management needs
- Plan accessibility requirements
- Consider responsive behavior

### Step 2: Create Component File
```bash
# Create component file
touch src/components/[ComponentName].tsx

# Create test file
touch src/components/[ComponentName].test.tsx

# Create story file (optional)
touch src/components/[ComponentName].stories.tsx
```

### Step 3: Define TypeScript Interfaces
```typescript
// Be specific with types
interface User {
  id: string;
  name: string;
  email: string;
}

// Use union types for limited options
type ButtonVariant = 'primary' | 'secondary' | 'ghost';

// Use discriminated unions for complex props
type InputProps = 
  | { type: 'text'; value: string; onChange: (v: string) => void }
  | { type: 'number'; value: number; onChange: (v: number) => void };
```

### Step 4: Implement Component Logic
```tsx
import { useState, useEffect, useCallback } from 'react';

export function ChatMessage({ message }: ChatMessageProps) {
  // State management
  const [isExpanded, setIsExpanded] = useState(false);
  
  // Memoized callbacks
  const handleToggle = useCallback(() => {
    setIsExpanded(prev => !prev);
  }, []);
  
  // Side effects
  useEffect(() => {
    // Auto-scroll when new message arrives
    if (message.isNew) {
      scrollToBottom();
    }
  }, [message.isNew]);
  
  // Computed values
  const displayContent = isExpanded 
    ? message.content 
    : message.content.slice(0, 100) + '...';
  
  return (/* JSX */);
}
```

### Step 5: Style with Tailwind CSS
```tsx
// ✅ DO: Use Tailwind utility classes
<div className="flex items-center gap-4 p-4 bg-white rounded-lg shadow-sm">

// ❌ DON'T: Use inline styles
<div style={{ display: 'flex', padding: '16px' }}>

// ✅ DO: Use CSS variables for theme colors
<div className="text-primary-500 bg-primary-50">

// ❌ DON'T: Hardcode colors
<div className="text-blue-500 bg-blue-50">

// ✅ DO: Use responsive prefixes
<div className="w-full md:w-1/2 lg:w-1/3">

// ✅ DO: Use dark mode variants
<div className="bg-white dark:bg-gray-800">
```

### Step 6: Add Accessibility
```tsx
// Semantic HTML
<nav aria-label="Main navigation">
<main role="main">
<article aria-label="Blog post">

// ARIA attributes
<button
  aria-label="Close dialog"
  aria-expanded={isOpen}
  aria-controls="dialog-content"
>

// Keyboard navigation
<input
  onKeyDown={(e) => {
    if (e.key === 'Escape') {
      onClose();
    }
  }}
/>

// Focus management
useEffect(() => {
  if (isOpen) {
    firstInputRef.current?.focus();
  }
}, [isOpen]);

// Screen reader support
<span className="sr-only">Loading...</span>
```

### Step 7: Write Tests
```tsx
// src/components/[ComponentName].test.tsx

import { render, screen, fireEvent } from '@testing-library/react';
import { [ComponentName] } from './[ComponentName]';

describe('[ComponentName]', () => {
  test('renders correctly with required props', () => {
    render(<[ComponentName] title="Test Title" />);
    expect(screen.getByText('Test Title')).toBeInTheDocument();
  });
  
  test('handles click events', () => {
    const handleClick = jest.fn();
    render(<[ComponentName] title="Test" onClick={handleClick} />);
    
    fireEvent.click(screen.getByRole('button'));
    expect(handleClick).toHaveBeenCalledTimes(1);
  });
  
  test('applies variant styles', () => {
    render(<[ComponentName] title="Test" variant="primary" />);
    const element = screen.getByRole('button');
    expect(element).toHaveClass('bg-primary-500');
  });
  
  test('is accessible', () => {
    render(<[ComponentName] title="Test" aria-label="Test button" />);
    expect(screen.getByRole('button')).toHaveAttribute('aria-label', 'Test button');
  });
});
```

## Best Practices

### ✅ DO
- Use functional components with Hooks
- Define clear TypeScript interfaces
- Follow single responsibility principle
- Keep components small (< 200 lines)
- Use semantic HTML elements
- Implement keyboard navigation
- Add ARIA labels
- Write comprehensive tests
- Use Tailwind for styling
- Extract reusable logic into custom hooks

### ❌ DON'T
- Don't use class components
- Don't use `any` type
- Don't create deeply nested components
- Don't use inline styles
- Don't ignore accessibility
- Don't mutate props
- Don't over-use state
- Don't skip error boundaries
- Don't hardcode values
- Don't forget loading states

## Common Patterns

### Controlled Component Pattern
```tsx
interface ControlledInputProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
}

export function ControlledInput({
  value,
  onChange,
  placeholder,
}: ControlledInputProps) {
  return (
    <input
      type="text"
      value={value}
      onChange={(e) => onChange(e.target.value)}
      placeholder={placeholder}
      className="w-full px-4 py-2 border rounded-lg"
    />
  );
}
```

### Compound Component Pattern
```tsx
// Parent component
interface TabsProps {
  children: React.ReactNode;
  defaultValue?: string;
}

export function Tabs({ children, defaultValue }: TabsProps) {
  const [activeTab, setActiveTab] = useState(defaultValue);
  
  return (
    <TabsContext.Provider value={{ activeTab, setActiveTab }}>
      <div className="tabs-container">{children}</div>
    </TabsContext.Provider>
  );
}

// Child component
export function Tab({ value, label }: TabProps) {
  const { activeTab, setActiveTab } = useTabsContext();
  
  return (
    <button
      onClick={() => setActiveTab(value)}
      className={cn('tab', { active: activeTab === value })}
    >
      {label}
    </button>
  );
}
```

### Render Props Pattern
```tsx
interface DataFetcherProps<T> {
  url: string;
  children: (data: T | null, loading: boolean, error: Error | null) => React.ReactNode;
}

export function DataFetcher<T>({ url, children }: DataFetcherProps<T>) {
  const [data, setData] = useState<T | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);
  
  useEffect(() => {
    fetch(url)
      .then(res => res.json())
      .then(setData)
      .catch(setError)
      .finally(() => setLoading(false));
  }, [url]);
  
  return <>{children(data, loading, error)}</>;
}

// Usage
<DataFetcher<User[]> url="/api/users">
  {(users, loading, error) => {
    if (loading) return <Spinner />;
    if (error) return <ErrorMessage error={error} />;
    return <UserList users={users} />;
  }}
</DataFetcher>
```

## State Management Guidelines

### Local State (useState)
- Component-specific UI state
- Form inputs
- Toggle states
- Temporary data

### Global State (Zustand)
- User authentication
- Application settings
- Shared data across components
- Persistent state

### URL State (React Router)
- Page identification
- Filter parameters
- Search queries
- Shareable state

## Performance Optimization

### React.memo
```tsx
export const ExpensiveComponent = React.memo(({ data }: Props) => {
  // Only re-renders if props change
  return <div>{/* complex rendering */}</div>;
});
```

### useMemo
```tsx
const sortedItems = useMemo(() => {
  return items.sort((a, b) => a.name.localeCompare(b.name));
}, [items]);
```

### useCallback
```tsx
const handleClick = useCallback(() => {
  doSomething(id);
}, [id]);
```

### Virtual Scrolling
```tsx
import { FixedSizeList } from 'react-window';

export function LongList({ items }: LongListProps) {
  return (
    <FixedSizeList
      height={600}
      itemCount={items.length}
      itemSize={50}
      width="100%"
    >
      {({ index, style }) => (
        <ListItem item={items[index]} style={style} />
      )}
    </FixedSizeList>
  );
}
```

## Accessibility Checklist
- [ ] Semantic HTML used
- [ ] ARIA labels added
- [ ] Keyboard navigation works
- [ ] Focus indicators visible
- [ ] Color contrast meets WCAG AA
- [ ] Screen reader tested
- [ ] Error messages announced
- [ ] Loading states indicated
- [ ] Interactive elements focusable
- [ ] No keyboard traps

## Testing Checklist
- [ ] Renders with default props
- [ ] Handles all prop variations
- [ ] Event handlers work correctly
- [ ] State updates properly
- [ ] Edge cases handled
- [ ] Error states displayed
- [ ] Loading states shown
- [ ] Accessibility verified
- [ ] Responsive behavior tested
- [ ] Memory leaks checked

## Debugging Tips
- Use React DevTools for component tree
- Check console for warnings
- Use `debug()` from testing-library
- Monitor re-renders with Profiler
- Check network tab for API calls
- Verify Tailwind classes applied
- Test with screen readers
- Validate HTML structure

## References
- [React Documentation](https://react.dev)
- [Tailwind CSS](https://tailwindcss.com)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [WCAG Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [Project UI/UX Design](../../doc/LingoMate-UI-UX设计规范.md)
