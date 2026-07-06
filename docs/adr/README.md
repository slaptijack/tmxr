# Architecture Decision Records

This directory records significant architectural decisions made in tmxr,
along with the context and rationale behind them. As an opinionated CLI,
tmxr accumulates intentional design decisions (session naming, configuration
discovery, shell integration, plugin philosophy, and similar topics). Writing
these down helps contributors understand why the project works the way it
does, and avoids repeatedly revisiting settled questions.

Not every change needs an ADR. Reach for one when a decision is significant,
hard to reverse, or likely to be questioned again later.

## Adding a new ADR

1. Copy [`0000-template.md`](0000-template.md) to `NNNN-kebab-case-title.md`,
   using the next sequential number.
2. Fill in the sections and set `Status` to `Proposed`.
3. Open it in a normal PR per `CONTRIBUTING.md`; reviewers weigh in like any
   other doc change.
4. Once merged, update `Status` to `Accepted` (this can happen in the same
   PR).

## Numbering

- ADRs are numbered sequentially with zero-padded 4-digit prefixes
  (`0001`, `0002`, ...).
- Numbers are never reused or renumbered, even if an ADR is superseded or
  deprecated.

## Status values

- `Proposed` — under discussion, not yet in effect.
- `Accepted` — the decision is in effect.
- `Superseded by NNNN` — replaced by a later ADR; keep the original for
  history.
- `Deprecated` — no longer applicable, without a direct replacement.

## Format

Each ADR uses the following sections, as laid out in the template:

- `Status`
- `Context` — the problem and forces at play.
- `Decision` — what was decided.
- `Consequences` — resulting tradeoffs, both positive and negative.
