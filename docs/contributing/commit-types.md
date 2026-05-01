# Commit Wizard Gitmoji Reference

A reference for Commit Wizard commit prefixes using Gitmoji-style commit types.

## Commit Message Format

```text
<emoji> <prefix>: <message>
```

Example:

```text
✨ feat: add user profile settings
```

---

## Features

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`feat`|✨|A new feature|Minor|Add a new dashboard page, API endpoint, CLI command, or user-facing capability.|
|`flags`|🚩|Feature flags or rollout controls|Patch|Add a feature toggle, rollout percentage, kill switch, or gated experiment.|
|`easter`|🥚|Add an easter egg or hidden feature|Patch|Add a hidden command, joke interaction, or non-obvious bonus behavior.|

---

## Fixes

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`fix`|🐛|A bug fix|Patch|Fix incorrect behavior, broken validation, failing edge cases, or runtime defects.|
|`hotfix`|🚑️|A critical production fix|Patch|Patch an outage, production crash, data-loss bug, or urgent live incident.|
|`patch`|🩹|Simple fix for a non-critical issue|Patch|Fix a small layout issue, minor typo in logic, or low-risk defect.|
|`revert`|⏪️|Revert a previous change|Patch|Undo a commit that caused a regression or broke expected behavior.|

---

## Breaking Changes

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`break`|💥|A breaking change|Major|Remove a public API, change a contract, rename required config, or alter behavior incompatibly.|

---

## Engineering

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`refactor`|♻️|A code change that improves structure without changing behavior|Patch|Simplify services, split functions, rename internals, or reorganize logic without behavior changes.|
|`fmt`|🎨|Code formatting or stylistic cleanup|Patch|Run formatter, normalize spacing, reorder imports, or clean style-only issues.|
|`perf`|⚡️|A performance improvement|Minor|Improve query speed, reduce memory usage, cache expensive work, or optimize hot paths.|
|`remove`|🔥|Removal of code, files, or other dead artifacts|Patch|Delete unused files, remove obsolete modules, or drop abandoned assets.|
|`move`|🚚|Move or rename files, folders, or symbols|Patch|Rename a package, move components, or reorganize directory structure.|
|`arch`|🏗️|Architecture or design boundary changes|Patch|Introduce layers, change module boundaries, split services, or adjust system structure.|
|`infra`|🧱|Infrastructure or environment changes|Patch|Change Docker, Terraform, hosting, service wiring, or environment setup.|
|`async`|🧵|Concurrency, async, or multithreading changes|Patch|Add worker pools, async handlers, mutex changes, queues, or task scheduling.|
|`valid`|🦺|Validation and defensive checks|Patch|Add input validation, guards, bounds checks, or safer failure paths.|
|`offline`|✈️|Improve offline support|Patch|Add local caching, offline queueing, sync recovery, or no-network fallbacks.|
|`compat`|🦖|Add backwards compatibility|Patch|Support old config keys, legacy payloads, previous clients, or deprecated behavior.|
|`spike`|⚗️|Experimental or spike work|Patch|Explore an approach, prototype an integration, or commit throwaway research code.|
|`types`|🏷️|Type definitions or type system improvements|Patch|Add TypeScript types, refine Rust traits, improve generics, or tighten type contracts.|
|`deadcode`|⚰️|Remove dead code that is no longer used|Patch|Delete unused branches, unreachable functions, stale interfaces, or obsolete abstractions.|
|`trash`|🗑️|Discard obsolete code, files, or temporary artifacts|Patch|Remove temp scripts, old migration leftovers, obsolete generated files, or stale notes.|
|`wip`|🚧|Work in progress changes|Patch|Commit incomplete work while checkpointing a branch.|
|`hack`|💩|Temporary hack, messy workaround, or intentionally rough code|Patch|Add a short-term workaround that should be cleaned up later.|
|`chaos`|🍻|Experimental chaotic changes or unserious spike work|Patch|Try risky or playful experiments that are not intended as polished work.|

---

## Documentation

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`docs`|📝|Documentation changes|Patch|Update README, API docs, architecture notes, or usage guides.|
|`text`|💬|User-facing text, messages, or literals|Patch|Change labels, error messages, empty states, notifications, or UI copy.|
|`typo`|✏️|Typo fixes|Patch|Fix spelling, grammar, punctuation, or minor wording mistakes.|
|`notes`|💡|Code comments or internal notes|Patch|Add comments, clarify implementation notes, or document internal assumptions.|

---

## User Experience

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`ui`|💄|Visual or UI styling updates|Patch|Change colors, spacing, typography, layout polish, or component styling.|
|`mobile`|📱|Mobile or responsive design changes|Patch|Improve breakpoints, mobile nav, touch targets, or responsive layout.|
|`a11y`|♿️|Accessibility improvements|Patch|Add ARIA labels, keyboard navigation, contrast fixes, or screen reader support.|
|`ux`|🚸|Improve user experience or usability|Patch|Simplify flows, reduce friction, improve onboarding, or clarify interactions.|
|`motion`|💫|Animations or transitions|Patch|Add page transitions, loading animations, hover motion, or micro-interactions.|
|`snap`|📸|Snapshot updates|Patch|Update Jest snapshots, visual regression baselines, or UI golden files.|
|`seo`|🔍️|SEO improvements|Patch|Add metadata, improve headings, update sitemap, or optimize crawlable content.|

---

## Quality

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`test`|✅|Add or update tests|Patch|Add unit, integration, e2e, property, or regression tests.|
|`repro`|🧪|Add a failing test or reproduction case|Patch|Add a minimal failing case before fixing a bug.|
|`lint`|🚨|Lint, warnings, or static analysis cleanup|Patch|Fix Clippy, ESLint, compiler warnings, static analyzer findings, or style violations.|
|`errors`|🥅|Error handling improvements|Patch|Improve error messages, add fallback handling, wrap errors, or handle failure modes.|
|`mock`|🤡|Mocks, stubs, fakes, or parody changes|Patch|Add test doubles, stub services, fake clients, or mocked data.|

---

## Delivery

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`deploy`|🚀|Deployment changes|Patch|Change deployment scripts, release targets, hosting config, or rollout process.|
|`init`|🎉|Initial project setup|Minor|Create the first project scaffold, baseline app, or repository structure.|
|`cifix`|💚|Fix continuous integration|Patch|Fix broken CI jobs, missing secrets, failing workflow steps, or build agents.|
|`ci`|👷|CI/CD pipeline changes|Patch|Add workflows, change build matrices, add checks, or update pipeline automation.|
|`release`|🔖|Release or version tagging changes|Patch|Update changelog, tag versions, prepare release metadata, or publish version files.|
|`merge`|🔀|Merge branches or histories|Patch|Merge feature branches, resolve branch histories, or sync long-lived branches.|

---

## Security

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`secure`|🔒️|Security fixes or hardening|Patch|Fix vulnerabilities, harden headers, sanitize input, or improve secure defaults.|
|`secret`|🔐|Secrets, keys, or environment credential handling|Patch|Rotate keys, move secrets to env vars, update secret loading, or remove committed credentials.|
|`auth`|🛂|Authentication, authorization, roles, or permissions|Patch|Change login, permissions, role checks, session handling, or access control logic.|

---

## Dependencies

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`depadd`|➕|Add a dependency|Patch|Add a library, package, crate, module, or external dependency.|
|`depdel`|➖|Remove a dependency|Patch|Remove unused packages, libraries, crates, or external modules.|
|`depup`|⬆️|Upgrade a dependency|Patch|Upgrade a dependency to a newer version.|
|`depdown`|⬇️|Downgrade a dependency|Patch|Roll back a dependency to an older version due to compatibility or regressions.|
|`pin`|📌|Pin a dependency version|Patch|Lock a dependency version to prevent unexpected upgrades.|
|`pkg`|📦️|Add or update compiled files or packages|Patch|Update bundled builds, generated packages, vendored files, or distributable artifacts.|

---

## Tooling

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`config`|🔧|Configuration changes|Patch|Change app config, formatter config, compiler settings, or environment defaults.|
|`tooling`|🔨|Build scripts, developer tooling, or helper automation|Patch|Add Makefile targets, scripts, generators, codegen, or local automation.|
|`dx`|🧑‍💻|Improve developer experience|Patch|Improve local setup, docs for contributors, faster dev commands, or better errors for developers.|
|`ignore`|🙈|Ignore rules such as .gitignore|Patch|Update `.gitignore`, `.dockerignore`, npm ignore, or packaging exclude rules.|

---

## Data

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`db`|🗃️|Database, schema, or persistence changes|Patch|Add migrations, indexes, table changes, persistence logic, or schema updates.|
|`seed`|🌱|Seed or fixture data changes|Patch|Add seed data, fixtures, sample records, or test datasets.|
|`metrics`|📈|Metrics, telemetry, or analytics|Patch|Add counters, dashboards, analytics events, or tracking fields.|
|`inspect`|🧐|Inspection, tracing, or debugging instrumentation|Patch|Add traces, debug probes, diagnostic logs, or runtime inspection hooks.|

---

## Observability

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`logs`|🔊|Add or improve logging|Patch|Add useful logs, improve log context, or make troubleshooting easier.|
|`unclog`|🔇|Remove or reduce noisy logging|Patch|Remove spammy logs, lower log levels, or reduce log volume.|
|`health`|🩺|Healthchecks, readiness, or liveness changes|Patch|Add `/health`, readiness probes, liveness checks, or service heartbeat logic.|

---

## Integrations

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`api`|👽️|External API integration or contract changes|Patch|Add third-party API support, update webhook contracts, or change external payload handling.|

---

## Business

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`billing`|💸|Billing, pricing, or cost-related logic|Patch|Change invoices, plans, pricing rules, metering, payments, or subscription behavior.|
|`biz`|👔|Add or update business logic|Patch|Change domain rules, workflows, eligibility checks, or product-specific behavior.|

---

## Miscellaneous

|Prefix|Emoji|Description|Bump|Example when to use|
|---|--:|---|---|---|
|`i18n`|🌐|Localization or internationalization changes|Patch|Add translations, locale files, language fallback, or date/number formatting.|
|`legal`|📄|License or legal text updates|Patch|Update license files, legal notices, attribution, or compliance text.|
|`assets`|🍱|Static assets or bundled resources|Patch|Add images, icons, fonts, fixtures, media, or other bundled resources.|
|`team`|👥|Contributor or collaboration metadata|Patch|Update contributors, CODEOWNERS, maintainers, ownership, or team metadata.|
