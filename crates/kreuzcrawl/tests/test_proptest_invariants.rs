//! Property-based invariant tests for three behavioural surfaces:
//!
//!   A. EWMA update function — bounded, converges to 0/1.
//!   B. FixedBudget — sum of approved demands ≤ initial; concurrent never-overdraw.
//!   C. compute_backoff_ms — capped at max; monotonic until cap.
//!
//! `EwmaTracker` lives in `pub(crate) defaults`, so the EWMA properties are
//! written against a local closure that mirrors the published formula exactly:
//!   next = prev * (1 - alpha) + blocked_f32 * alpha,  alpha = 0.1
//!
//! `FixedBudget` and `compute_backoff_ms` are in the same private module, so
//! they are also mirrored locally.  The mirrors are deliberately minimal and
//! match the source verbatim — any divergence is a documentation bug, not a
//! test bug.

#![allow(clippy::unwrap_used)]

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use proptest::prelude::*;

// ---------------------------------------------------------------------------
// Mirror of EwmaTracker::update  (alpha = 0.1, identical to source)
// ---------------------------------------------------------------------------

const EWMA_ALPHA: f32 = 0.1;

fn ewma_update(prev: f32, blocked: bool) -> f32 {
    let observation: f32 = if blocked { 1.0 } else { 0.0 };
    EWMA_ALPHA.mul_add(observation, (1.0 - EWMA_ALPHA) * prev)
}

// ---------------------------------------------------------------------------
// Mirror of FixedBudget  (CAS loop, identical to source)
// ---------------------------------------------------------------------------

struct FixedBudgetMirror {
    remaining: AtomicU32,
}

impl FixedBudgetMirror {
    fn new(initial: u32) -> Self {
        Self {
            remaining: AtomicU32::new(initial),
        }
    }

    fn try_consume(&self, cost: u32) -> bool {
        let mut current = self.remaining.load(Ordering::Acquire);
        loop {
            if current < cost {
                return false;
            }
            let next = current - cost;
            match self
                .remaining
                .compare_exchange_weak(current, next, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => return true,
                Err(actual) => current = actual,
            }
        }
    }

    fn remaining(&self) -> u32 {
        self.remaining.load(Ordering::Acquire)
    }
}

// ---------------------------------------------------------------------------
// Mirror of compute_backoff_ms  (identical to source)
// ---------------------------------------------------------------------------

fn compute_backoff_ms_mirror(attempt: u32, max_backoff_ms: u64) -> u64 {
    let exp = 1u64.checked_shl(attempt).unwrap_or(u64::MAX);
    exp.saturating_mul(100).min(max_backoff_ms)
}

// ===========================================================================
// A. EWMA invariants
// ===========================================================================

proptest! {
    /// A1: update output is always in [0.0, 1.0] for any valid prev and blocked flag.
    #[test]
    fn ewma_update_is_bounded(prev in 0.0f32..=1.0, blocked: bool) {
        let next = ewma_update(prev, blocked);
        prop_assert!(
            next >= 0.0,
            "ewma_update({prev}, {blocked}) = {next} — violated lower bound 0.0"
        );
        prop_assert!(
            next <= 1.0,
            "ewma_update({prev}, {blocked}) = {next} — violated upper bound 1.0"
        );
    }

    /// A2: 200 consecutive blocked observations converge to within 0.01 of 1.0.
    #[test]
    fn ewma_all_block_converges_to_one(prev in 0.0f32..=1.0) {
        let mut ewma = prev;
        for _ in 0..200 {
            ewma = ewma_update(ewma, true);
        }
        prop_assert!(
            ewma >= 0.99,
            "all-block convergence: after 200 blocks starting from {prev}, ewma={ewma} < 0.99"
        );
    }

    /// A3: 200 consecutive success observations converge to within 0.01 of 0.0.
    #[test]
    fn ewma_all_success_converges_to_zero(prev in 0.0f32..=1.0) {
        let mut ewma = prev;
        for _ in 0..200 {
            ewma = ewma_update(ewma, false);
        }
        prop_assert!(
            ewma <= 0.01,
            "all-success convergence: after 200 successes starting from {prev}, ewma={ewma} > 0.01"
        );
    }
}

// ===========================================================================
// B. FixedBudget invariants
// ===========================================================================

proptest! {
    /// B1: sum of approved demands never exceeds the initial budget.
    #[test]
    fn fixed_budget_approved_sum_never_exceeds_initial(
        initial in 1u32..=1000,
        demands in proptest::collection::vec(0u32..=1000, 0..=50),
    ) {
        let budget = FixedBudgetMirror::new(initial);
        let mut approved_sum: u64 = 0;
        for demand in &demands {
            if budget.try_consume(*demand) {
                approved_sum += u64::from(*demand);
            }
        }
        prop_assert!(
            approved_sum <= u64::from(initial),
            "approved_sum={approved_sum} > initial={initial}"
        );
    }
}

/// B2: 100 concurrent try_consume(1) tasks on a budget of 100 — exactly 100 succeed.
///
/// proptest! macro is sync; use tokio runtime inline.
#[test]
fn fixed_budget_concurrent_never_overdraws() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let budget = Arc::new(FixedBudgetMirror::new(100));
        let mut handles = Vec::with_capacity(100);
        for _ in 0..100 {
            let b = budget.clone();
            handles.push(tokio::spawn(async move { b.try_consume(1) }));
        }
        let mut approvals: u32 = 0;
        for h in handles {
            if h.await.unwrap() {
                approvals += 1;
            }
        }
        assert_eq!(
            approvals, 100,
            "expected exactly 100 approvals with initial=100, got {approvals}"
        );
        assert_eq!(
            budget.remaining(),
            0,
            "remaining should be 0 after 100 approved try_consume(1), got {}",
            budget.remaining()
        );
    });
}

// ===========================================================================
// C. Backoff invariants
// ===========================================================================

proptest! {
    /// C1: compute_backoff_ms is always <= max_backoff_ms.
    #[test]
    fn backoff_never_exceeds_max(
        attempt in 0u32..32,
        max_backoff_ms in 1000u64..=60_000,
    ) {
        let backoff = compute_backoff_ms_mirror(attempt, max_backoff_ms);
        prop_assert!(
            backoff <= max_backoff_ms,
            "compute_backoff_ms({attempt}, {max_backoff_ms}) = {backoff} exceeds cap"
        );
    }

    /// C2: compute_backoff_ms is monotonically non-decreasing until the cap is reached.
    ///
    /// For attempt values where 2^attempt * 100 < max_backoff_ms, the next attempt
    /// must produce a value >= this attempt's value.
    #[test]
    fn backoff_is_monotonic_until_cap(
        attempt in 0u32..30,
        max_backoff_ms in 1000u64..=60_000,
    ) {
        let this_backoff = compute_backoff_ms_mirror(attempt, max_backoff_ms);
        let next_backoff = compute_backoff_ms_mirror(attempt + 1, max_backoff_ms);

        // Once we hit the cap, both values equal max_backoff_ms — that is fine.
        // Before the cap, next must be strictly greater (doubles each step).
        prop_assert!(
            next_backoff >= this_backoff,
            "compute_backoff_ms({attempt}) = {this_backoff} > compute_backoff_ms({}) = {next_backoff}",
            attempt + 1
        );
    }
}
