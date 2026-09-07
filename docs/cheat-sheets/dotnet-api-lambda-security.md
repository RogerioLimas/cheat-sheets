# .NET API / AWS Lambda Security Cheat Sheet

Actionable checklist for .NET (Minimal API / ASP.NET Core) and .NET AWS
Lambdas. Maps to OWASP Top 10. Local scan: `make security` (zero token).

## 1. Injection (SQL / command / OWASP A03)

- Parameterize every query. Never concatenate SQL.
  - EF Core: LINQ or `FromSqlInterpolated` (safe) — never `FromSqlRaw` with
    string interpolation.
  - Dapper/ADO.NET: pass `DbParameter`s, never build the command text from input.
- Validate input with DataAnnotations or FluentValidation at the edge.
- Never pass user input to `Process.Start`. Use argument arrays, not a
  concatenated command line.

## 2. Auth & authorization (A01 / A07)

- Enforce authorization per endpoint: `[Authorize]` / `.RequireAuthorization()`.
  Don't rely on client-side checks.
- Validate JWTs fully: `ValidateIssuer`, `ValidateAudience`,
  `ValidateLifetime`, `ValidateIssuerSigningKey` all true. Never disable
  signature validation.
- Use policy-based authorization for resource ownership checks (IDOR / A01) —
  confirm the caller owns the record, not just that they're logged in.

## 3. Secrets & config (A05 / A02)

- No secrets in `appsettings.json`, source, or env committed to git.
  - Local: user-secrets (`dotnet user-secrets`).
  - AWS: Secrets Manager / SSM Parameter Store, fetched at runtime.
- Never log secrets, tokens, or full request bodies. Scrub PII from logs.
- Gitleaks blocks credential commits; keep it in the gate.

## 4. Lambda-specific

- Least-privilege IAM: scope the execution role to the exact
  actions/resources needed. No wildcard `*` actions/resources.
- Treat every event source (API Gateway, SQS, S3) as untrusted input.
  Validate the payload shape before use.
- Keep secrets out of environment variables when possible; prefer Secrets
  Manager. If in env, they're visible to anyone with `GetFunction`.
- Set a function timeout and memory ceiling; validate payload size to limit
  abuse. Enable structured logging (no sensitive fields).
- Pin the runtime; patch dependencies — Lambda won't auto-update your libs.

## 5. Dependencies (A06)

- `dotnet list package --vulnerable --include-transitive` on every build.
- Fix HIGH/CRITICAL before merge. Trivy scans NuGet lockfiles.
- Enable `<Nullable>enable</Nullable>` and treat warnings as errors to catch
  whole bug classes at compile time.

## 6. Transport & data exposure (A02 / A04)

- HTTPS only: `UseHttpsRedirection()` + HSTS. TLS 1.2+.
- Return `ProblemDetails`, not raw exceptions/stack traces, to clients.
  Log the detail server-side only.
- Enable rate limiting (`AddRateLimiter`) on public endpoints.
- Set security headers (CSP, `X-Content-Type-Options`, etc.) via middleware.

## 7. Error handling & robustness

- No `throw`/unhandled exceptions leaking internals across the API boundary.
- Validate and bound all external input (size, type, range) before processing.
- Use `System.Text.Json` with explicit types; avoid deserializing to
  polymorphic/`object` from untrusted input (deserialization risks / A08).

## Quick gate
```
make security                                   # full local scan, zero token
dotnet list package --vulnerable --include-transitive
dotnet format --verify-no-changes
```
