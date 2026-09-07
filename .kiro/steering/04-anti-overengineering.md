---
inclusion: always
---
# Anti-Overengineering (lazy senior dev mode)

Lazy means efficient, not careless. The best code is the code never written.
Understand the problem first — read the task and the code it touches, trace the
real flow end to end — then climb the ladder and stop at the first rung that
holds:

1. Does this need to be built at all? (YAGNI)
2. Does it already exist in this codebase? Reuse it.
3. Does the standard library do it? Use it.
4. Does a native platform feature (Tauri, OS, browser) cover it? Use it.
5. Does an already-installed dependency solve it? Use it.
6. Can it be one line? Make it one line.
7. Only then: write the minimum code that works.

Bug fix = root cause, not symptom. Grep every caller of the function you touch
and fix the shared function once, not each call site.

## Rules

- No abstraction (interface/factory/abstract class) for a single implementation
  — add it when a second one actually exists.
- No new dependency if avoidable. No config option for a value that never
  changed. No boilerplate nobody asked for.
- Deletion over addition. Boring over clever. Fewest files possible.
- Shortest working diff wins — but only once you understand the problem. A
  small change in the wrong place is a second bug.
- Two same-size stdlib options: pick the edge-case-correct one.
- Mark a deliberate corner-cut with a known ceiling (global lock, O(n²) scan)
  using a `// ponytail:` comment naming the ceiling and upgrade path.

## Not lazy about

Understanding the problem, input validation at trust boundaries, error handling
that prevents data loss, security, accessibility, anything explicitly
requested. Never overrides the security (`01`) or TDD (`02`) policies.
