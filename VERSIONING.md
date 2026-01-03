# Versioning Policy

This document describes the versioning strategy used by the **Anvil** project.

Anvil follows **Semantic Versioning (SemVer)** with intentional flexibility during the `0.x` phase, as recommended for early-stage infrastructure libraries.

---

## Version Format

```
MAJOR.MINOR.PATCH
```

Examples:

* `0.1.0`
* `0.1.1`
* `0.2.0`
* `1.0.0`

---

## Current Phase: `0.x` (Pre-Stable)

Anvil is currently in the **`0.x` series**, which indicates:

* APIs are usable in production
* The design is intentional and conservative
* **Breaking changes are allowed between minor versions**

This phase exists to allow real-world feedback to shape the final stable API.

---

## Version Increment Rules

### PATCH (`0.x.Y`)

Increment the PATCH version when:

* Fixing bugs
* Improving error messages
* Internal refactoring with **no public API changes**
* Documentation-only changes (optional)

Example:

```
0.1.0 → 0.1.1
```

---

### MINOR (`0.X.0`)

Increment the MINOR version when:

* Adding new features
* Adding new public APIs
* Introducing **breaking changes** to existing APIs
* Changing behavior that may affect users

Example:

```
0.1.0 → 0.2.0
```

> During the `0.x` phase, **breaking changes must always bump the MINOR version**.

---

### MAJOR (`1.0.0`)

The MAJOR version will be incremented to `1.0.0` when:

* Core APIs are considered stable
* Real-world usage validates the design
* Backward compatibility becomes a strong guarantee

After `1.0.0`:

* Breaking changes will require a MAJOR bump
* Minor versions will be backward compatible

---

## Crate Coordination

Anvil consists of multiple crates:

* `anvil-core`
* `anvil-adapter-*`

Versioning rules:

* `anvil-core` defines the **source of truth** for versions
* Adapters should:

  * Match the core version when possible
  * Depend on an explicit core version (e.g. `anvil-core = "0.1"`)

Example:

```toml
anvil-core = "0.1"
```

---

## Publishing Rules

1. `anvil-core` **must always be published first**
2. Adapters may only depend on **published core versions**
3. Published versions are **immutable** and cannot be changed or removed

---

## Backward Compatibility Expectations

During `0.x`:

* Minor version bumps may include breaking changes
* Patch versions must not break existing code

After `1.0.0`:

* Breaking changes only in MAJOR versions
* Minor and patch releases must be backward compatible

---

## Deprecation Policy

When removing or changing APIs:

* Prefer deprecating first when feasible
* Clearly document breaking changes in `CHANGELOG.md`
* Provide migration guidance when possible

---

## Summary

* Anvil follows SemVer
* `0.x` allows iteration with discipline
* Stability increases as versions approach `1.0.0`
* Versioning decisions prioritize **clarity and trust** over speed
