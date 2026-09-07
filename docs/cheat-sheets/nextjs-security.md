# Next.js Security Cheat Sheet

Actionable checklist for Next.js (App Router) apps. Maps to OWASP Top 10.
Scan locally with: `make security` (Semgrep + Trivy + Gitleaks — zero token).

## 1. Injection (SQL / command / OWASP A03)

- Parameterize every DB query. Never build SQL by string concatenation.
  - Prisma/Drizzle: use query builders / tagged templates, not `$queryRawUnsafe`.
  - `pg`/`mysql2`: pass values as parameter array, never interpolate.
- Never pass user input to `child_process` / shell. If unavoidable, use
  `execFile` with an argument array, never `exec` with a string.
- Validate all input at the boundary with a schema (Zod). Reject, don't coerce.

## 2. Server/Client boundary (Next-specific, highest-impact)

- Server Actions and Route Handlers are public HTTP endpoints. Re-check
  auth + authorization inside each one — never trust the caller.
- Never import server-only secrets into a Client Component. Guard server
  modules with `import 'server-only'`.
- `NEXT_PUBLIC_*` env vars are shipped to the browser. Keep secrets in
  non-public vars, read only in server code.
- Don't leak internal data through props serialized to the client — only
  send what the component renders.

## 3. Auth & sessions (A01 / A07)

- Cookies: `httpOnly`, `secure`, `sameSite=lax` (or `strict`). Never store
  tokens in `localStorage`.
- Verify the session in `middleware.ts` AND in each protected handler
  (middleware alone is bypassable on some routes).
- CSRF: Server Actions have built-in protection; custom POST route handlers
  need an explicit CSRF token or strict `sameSite` + origin check.

## 4. XSS (A03)

- Avoid `dangerouslySetInnerHTML`. If required, sanitize with DOMPurify first.
- Never build URLs/HTML from unsanitized input. Validate `redirect()` targets
  against an allowlist (open-redirect).
- Set a strict Content-Security-Policy via `next.config.js` headers; no
  `unsafe-inline`/`unsafe-eval` without a documented reason.

## 5. Dependencies (A06)

- `npm audit` on every install; fix HIGH/CRITICAL before merge.
- Pin versions (commit the lockfile). Trivy flags known-vulnerable versions.
- Watch for typosquatted package names on install.

## 6. Secrets & config (A05 / A02)

- No secrets in source, `next.config.js`, or `NEXT_PUBLIC_*`. Use env + a
  secret manager. Gitleaks blocks commits containing credentials.
- Security headers: HSTS, `X-Content-Type-Options: nosniff`,
  `Referrer-Policy`, `X-Frame-Options` (or CSP `frame-ancestors`).
- Disable the `X-Powered-By` header (`poweredByHeader: false`).

## 7. SSRF & data exposure (A10 / A02)

- Validate/allowlist any server-side `fetch` target built from user input.
- Rate-limit route handlers and Server Actions.
- Return generic errors to the client; log details server-side only.

## Quick gate
```
make security          # full local scan, no tokens, no data leaves machine
npm audit --audit-level=high
```
