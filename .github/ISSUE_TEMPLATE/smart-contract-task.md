---
name: Smart Contract Task
about: A scoped Soroban/Rust implementation task for GrantFox contributors
title: "[Smart Contracts] "
labels: "smart-contract, good first issue"
assignees: ""
---

## 🎯 Objective

<!-- Describe what function or feature needs to be implemented -->

## 🛠️ Technical Specifications

- **File to update:** `contracts/campaign/src/lib.rs`
- **Soroban SDK version:** `26.0.1`

<!-- Add specific implementation notes, storage keys to read/write, state checks required, etc. -->

## ✅ Acceptance Criteria

- [ ] Implementation compiles to wasm without errors
- [ ] All existing tests continue to pass (`cargo test --all`)
- [ ] New unit tests cover the happy path and at least one error/edge case
- [ ] Zero `cargo clippy` warnings introduced
- [ ] PR references this issue: `Closes #<issue-number>`

## 📎 Context

<!-- Link to relevant contract types (DataKey, CampaignState, Milestone) or Soroban docs -->
- [Soroban SDK docs](https://docs.rs/soroban-sdk/latest)
- [Stellar token interface](https://developers.stellar.org/docs/tokens/token-interface)
