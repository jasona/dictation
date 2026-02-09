---
id: CODE-INTERNAL
name: Node.js, Next.js, Tailwind, shadcn/ui & Radix Code Standards
version: 1.0.0
owner: "Engineering"
last_updated: "2026-01-22"
---

## Purpose

These standards ensure our Next.js codebase is readable, maintainable, testable, and consistent across all team members and projects. This stack uses TypeScript, Tailwind CSS for styling, and shadcn/ui (built on Radix primitives) for components.

## Musts (Non-Negotiable)

### Code Quality
- [INT-1] **TypeScript Strict Mode**: Projects MUST use TypeScript with `strict: true`; avoid `any` types except when absolutely necessary with a comment explaining why.
- [INT-2] **Use Server Components by Default**: Components MUST be React Server Components unless they require client-side interactivity; add `'use client'` directive only when needed.
- [INT-3] **Small Components**: Components MUST be focused and single-purpose; extract sub-components when a file exceeds 150 lines.
- [INT-4] **Async/Await Consistency**: Async functions MUST use `async/await`; avoid mixing `.then()` chains with await in the same function.
- [INT-5] **No Hardcoded Secrets**: API keys, secrets, and environment-specific values MUST be in environment variables; never commit `.env` files.

### Component Architecture
- [INT-6] **Use shadcn/ui Components**: UI primitives MUST use shadcn/ui components when available; do not recreate buttons, dialogs, dropdowns, etc. from scratch.
- [INT-7] **Radix for Accessibility**: Interactive components MUST use Radix primitives (via shadcn/ui) for keyboard navigation and ARIA compliance.
- [INT-8] **Tailwind for Styling**: All styling MUST use Tailwind CSS utility classes; avoid inline styles and CSS modules except for third-party library overrides.

### Testing
- [INT-9] **Tests Must Pass**: All existing tests MUST pass before merging; do not use `.skip()` to make builds pass.

## Shoulds (Strong Recommendations)

### TypeScript Patterns
- [INT-10] **Zod for Validation**: SHOULD use Zod for runtime validation of API inputs, form data, and external data; infer types from schemas.
- [INT-11] **Discriminated Unions**: SHOULD use discriminated unions for state that has mutually exclusive variants (loading/success/error).
- [INT-12] **Prefer `satisfies`**: SHOULD use `satisfies` operator for type checking object literals while preserving literal types.
- [INT-13] **Avoid Enums**: SHOULD prefer `as const` objects or union types over TypeScript enums.

### Next.js Patterns
- [INT-14] **App Router**: SHOULD use the App Router (`app/` directory) for all new routes; avoid Pages Router for new features.
- [INT-15] **Server Actions**: SHOULD use Server Actions for form submissions and mutations instead of API routes when possible.
- [INT-16] **Route Handlers for APIs**: SHOULD use Route Handlers (`route.ts`) for REST API endpoints consumed by external clients.
- [INT-17] **Metadata API**: SHOULD use the Metadata API for SEO; define metadata in `layout.tsx` or `page.tsx` files.
- [INT-18] **Image Optimization**: SHOULD use `next/image` for all images; configure remote patterns in `next.config.js`.

### Styling & Components
- [INT-19] **CN Utility**: SHOULD use the `cn()` utility (clsx + tailwind-merge) for conditional class names.
- [INT-20] **Component Variants**: SHOULD use `cva` (class-variance-authority) for component variants with multiple visual states.
- [INT-21] **Composition over Props**: SHOULD compose components using children and slots rather than excessive prop drilling.
- [INT-22] **Consistent Spacing**: SHOULD use Tailwind's spacing scale consistently; prefer `gap` over margins for flex/grid layouts.

### Code Organization
- [INT-23] **Colocation**: SHOULD colocate related files (component, tests, types, utils) in the same directory.
- [INT-24] **Barrel Exports**: SHOULD use `index.ts` barrel exports sparingly; prefer direct imports to improve tree-shaking.

## Architecture Patterns

### Recommended Patterns
- **Feature-Based Structure**: Organize by feature (`features/auth/`, `features/dashboard/`) rather than by type (`components/`, `hooks/`)
- **Server-First Data Fetching**: Fetch data in Server Components; pass to Client Components as props
- **Optimistic Updates**: Use `useOptimistic` for instant UI feedback on mutations
- **Error Boundaries**: Wrap route segments with `error.tsx` for graceful error handling
- **Loading States**: Use `loading.tsx` and Suspense boundaries for streaming
- **Parallel Routes**: Use parallel routes (`@modal`, `@sidebar`) for complex layouts

### Patterns to Avoid
- **Client-Side Data Fetching for Initial Load**: Avoid `useEffect` + fetch for data that can be fetched on the server
- **Prop Drilling**: Avoid passing props through many levels; use composition or context
- **CSS-in-JS Runtime**: Avoid runtime CSS-in-JS libraries (styled-components, emotion) with Server Components
- **Barrel File Hell**: Avoid deep barrel exports that break tree-shaking and slow builds

## Project Structure

```
src/
├── app/                      # Next.js App Router
│   ├── (auth)/               # Route group for auth pages
│   ├── (dashboard)/          # Route group for dashboard
│   ├── api/                  # Route Handlers (REST APIs)
│   ├── layout.tsx            # Root layout
│   └── page.tsx              # Home page
├── components/
│   ├── ui/                   # shadcn/ui components (auto-generated)
│   └── [feature]/            # Feature-specific components
├── features/                 # Feature modules
│   └── [feature]/
│       ├── components/       # Feature components
│       ├── hooks/            # Feature hooks
│       ├── actions.ts        # Server Actions
│       └── types.ts          # Feature types
├── lib/                      # Shared utilities
│   ├── utils.ts              # cn() and helpers
│   └── validations/          # Zod schemas
├── hooks/                    # Shared custom hooks
└── types/                    # Global type definitions
```

## Examples

### Good: Server Component with Data Fetching
```tsx
// app/users/page.tsx - Server Component (default)
import { UserList } from '@/features/users/components/user-list'
import { getUsers } from '@/features/users/actions'

export default async function UsersPage() {
  const users = await getUsers()

  return (
    <main className="container mx-auto py-8">
      <h1 className="text-3xl font-bold mb-6">Users</h1>
      <UserList users={users} />
    </main>
  )
}
```

### Bad: Unnecessary Client Component
```tsx
// Don't do this - fetching on client when server fetch works
'use client'
import { useEffect, useState } from 'react'

export default function UsersPage() {
  const [users, setUsers] = useState([])

  useEffect(() => {
    fetch('/api/users').then(r => r.json()).then(setUsers)
  }, [])

  return <UserList users={users} />
}
```

### Good: shadcn/ui with Tailwind
```tsx
'use client'
import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { cn } from '@/lib/utils'

interface Props {
  className?: string
  onConfirm: () => void
}

export function ConfirmDialog({ className, onConfirm }: Props) {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="destructive" className={cn('w-full', className)}>
          Delete Account
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Are you sure?</DialogTitle>
        </DialogHeader>
        <div className="flex gap-2 justify-end">
          <Button variant="outline">Cancel</Button>
          <Button variant="destructive" onClick={onConfirm}>
            Delete
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  )
}
```

### Good: Server Action with Zod Validation
```tsx
// features/users/actions.ts
'use server'
import { z } from 'zod'
import { revalidatePath } from 'next/cache'

const createUserSchema = z.object({
  email: z.string().email(),
  name: z.string().min(2).max(100),
})

export async function createUser(formData: FormData) {
  const parsed = createUserSchema.safeParse({
    email: formData.get('email'),
    name: formData.get('name'),
  })

  if (!parsed.success) {
    return { error: parsed.error.flatten().fieldErrors }
  }

  const user = await db.user.create({ data: parsed.data })
  revalidatePath('/users')
  return { success: true, userId: user.id }
}
```

### Good: Discriminated Union for State
```tsx
type AsyncState<T> =
  | { status: 'idle' }
  | { status: 'loading' }
  | { status: 'success'; data: T }
  | { status: 'error'; error: string }

function UserProfile({ state }: { state: AsyncState<User> }) {
  switch (state.status) {
    case 'idle':
      return null
    case 'loading':
      return <Skeleton className="h-20 w-full" />
    case 'error':
      return <Alert variant="destructive">{state.error}</Alert>
    case 'success':
      return <ProfileCard user={state.data} />
  }
}
```

### Good: Component Variants with CVA
```tsx
import { cva, type VariantProps } from 'class-variance-authority'
import { cn } from '@/lib/utils'

const badgeVariants = cva(
  'inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold',
  {
    variants: {
      variant: {
        default: 'bg-primary text-primary-foreground',
        secondary: 'bg-secondary text-secondary-foreground',
        destructive: 'bg-destructive text-destructive-foreground',
        outline: 'border border-input bg-background',
      },
    },
    defaultVariants: {
      variant: 'default',
    },
  }
)

interface BadgeProps
  extends React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof badgeVariants> {}

export function Badge({ className, variant, ...props }: BadgeProps) {
  return <div className={cn(badgeVariants({ variant }), className)} {...props} />
}
```

## NPM Packages (Recommended)

| Package | Purpose |
|---------|---------|
| zod | Schema validation and type inference |
| @tanstack/react-query | Client-side data fetching and caching |
| class-variance-authority | Component variant management |
| clsx + tailwind-merge | Conditional class name handling |
| lucide-react | Icon library (used by shadcn/ui) |
| react-hook-form | Form state management |
| @hookform/resolvers | Zod integration for react-hook-form |
| nuqs | Type-safe URL search params |
| vitest + @testing-library/react | Testing |

## References

- Next.js Documentation: https://nextjs.org/docs
- shadcn/ui Documentation: https://ui.shadcn.com
- Radix Primitives: https://www.radix-ui.com/primitives
- Tailwind CSS: https://tailwindcss.com/docs
- Zod Documentation: https://zod.dev
- Class Variance Authority: https://cva.style/docs
