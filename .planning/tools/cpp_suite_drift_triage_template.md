# C++ Suite Drift Triage Template

Use this template when:

```powershell
powershell -ExecutionPolicy Bypass -File .planning/tools/cpp_suite_drift_check.ps1
```

returns drift failure.

---

## 1. Drift Snapshot

- Date:
- Baseline file:
- Source root:
- Drift check command output:

### Changed Files

| File | Drift Type | Detail |
|------|------------|--------|
| | | |

---

## 2. Old-Suite Block Mapping

Map changed old C++ suite blocks to current FFI evidence.

| Old C++ file/block | Current FFI evidence | Initial status |
|--------------------|----------------------|----------------|
| | | covered / equivalent / gap |

---

## 3. Classification

For each changed block, classify:

- **covered**: existing tests already execute the changed behavior.
- **equivalent**: API evolved but behavior is already proven equivalent.
- **gap**: no executable evidence currently covers the changed behavior.

Record justification for each item:

| Block | Classification | Justification |
|-------|----------------|---------------|
| | | |

---

## 4. Action Decision

For each **gap**:

1. Name target Rust file/module.
2. Define test shape (property-level or vertex-level).
3. State required assertions.
4. Link to phase/plan where fix lands.

| Gap block | Rust target | Planned test | Plan/Phase |
|-----------|-------------|--------------|------------|
| | | | |

---

## 5. Closure Checklist

- [ ] Cross-suite checklist updated.
- [ ] New/updated tests added for all gap items.
- [ ] `cargo test --workspace -q` passed.
- [ ] `cargo fmt --all --check` passed.
- [ ] `cargo clippy --all-targets -- -D warnings` passed.
- [ ] `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` passed.
- [ ] `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` healthy.
- [ ] Baseline updated only after re-audit completion (if required).
