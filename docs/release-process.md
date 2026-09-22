# Open-source release process

This document describes how we release `c2patool` and the branching model
that supports it. It's adapted from
[c2pa-rs's release process](https://github.com/contentauth/c2pa-rs/blob/main/docs/release-process.md),
which this repo split off from -- the branching model, cadence, and
automation are deliberately kept the same for continuity, minus everything
that only made sense for coordinating multiple crates in one repo. It
applies to the **0.x (pre-1.0) phase**; we will revisit it as we approach a
1.0 stability commitment.

## Goals

We have two main goals:

* **Move fast on pre-1.0 refactors.** While we're below 1.0, we may make larger refactors and breaking changes.
* **Be stable and predictable as much as possible for users**: a steady stream of features, plus breaking changes that arrive on a pre-determined *known schedule*

## Core principle: split by breaking vs. non-breaking

What matters to someone depending on this tool isn't whether a change is "big": it's whether it breaks their scripts or workflows. An additive feature costs them nothing; a changed or removed flag/behavior forces them to adapt. So we govern those two kinds of change on two different tracks, which map directly onto pre-1.0 Cargo semantics:

| Change kind | Version slot | Cargo treats it as | Track |
| -- | -- | -- | -- |
| Additive / non-breaking | `0.x.y` (bump `y`) | compatible | **Track 1**: fast, on the current train |
| Breaking | `0.x.0` (bump `x`) | incompatible | **Track 2**: scheduled "train" |

Most changes never wait for the train: anything additive ships fairly quickly by being backported to the stable release train; only breaking changes are batched and scheduled.

## Branching model

| Branch | Role | Published to crates.io? |
| -- | -- | -- |
| `main` | Always green but unstable ("nightly-like"). It must always compile and pass tests, but its behavior is **not** guaranteed stable. It also tracks c2pa-rs's own `main` (see [Tracking c2pa-rs main](#tracking-c2pa-rs-main) below), so it may pick up upstream changes ahead of any c2pa release. | **No** |
| `stable` | Tracks the most-recent crates.io release and is the currently-active release line. Additive (`0.x.y`) releases, and the promoted breaking (`0.x.0`) release, are published from here. | **Yes** |
| `v0.x` (e.g. `v0.27`) | A long-lived branch for a **retired** release line, snapshotted from `stable` when that line is retired. A potential target if a security or other critical bug fix is made to a retired line. | Yes (rare backports) |
| `0.(x+1).0-rc` (release-candidate branch) | A transient breaking candidate, cut from `main`, that bakes before promotion. Its name ends in `-rc` so it is validated by CI but **never** matches a crates.io publish trigger. Individual **builds** cut from it (`-rc.1`, `-rc.2`, …) are tagged and get *prerelease* GitHub releases with binaries, but are never published to crates.io. | crates.io: **No**; GitHub-release binaries: **yes** |

The branch-name conventions are also the crates.io publish guard: the publish workflow only ever runs on `stable` and `v0.*`, and the `-rc`-named candidate branch matches neither, so a candidate can never be published to crates.io by construction.

Two rules keep this coherent:

* **Upstream-first.** Every change lands on `main` first. Release-line branches only ever *receive* changes (via cherry-pick); nothing originates on them. Two mechanisms enforce it: a proactive [upstream-first check](#upstream-first-check-proactive) on every PR to a release branch, and a scheduled [reconciliation check](#reconciliation-check-reactive) as a backstop.
* **`main` stays releasable.** Destabilizing work happens on feature branches off `main`, merged only once coherent.

> [!NOTE]
> Cherry-picking transfers individual fixes between branches without merging everything. Because actively-supported lines can diverge over time, a fix that applies cleanly on one branch may not cherry-pick directly onto another. In that case, the change may need to be adapted to compile, integrate, and pass tests on the target branch, or, if the branches have diverged enough, implemented separately for each supported branch.

## Tracking c2pa-rs main

`main`'s `Cargo.toml` depends on `c2pa` via a **git dependency** on c2pa-rs's own `main` branch, not a crates.io version -- so this repo continuously integration-tests against c2pa-rs's latest in-development code instead of only discovering a break once a new `c2pa` version publishes. A scheduled workflow, [`track-c2pa-rs-main.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/track-c2pa-rs-main.yml), keeps this fresh: it runs `cargo update -p c2pa` on weekday mornings and pushes directly to `main` if `Cargo.lock` changed. If the update ever breaks the build, CI on `main` goes red like any other main-breaking change.

`stable` (and any `v0.*` line) uses a real crates.io version instead, since crates.io requires every dependency to resolve to a published version. [`check-no-patch-deps.yml`](#patch-dependency-guard) mechanically enforces that this git dependency never reaches a release branch.

This means the [release train](#release-train-cut)'s cadence is deliberately staggered 30 minutes *after* c2pa-rs's own train cut (16:30 UTC vs. c2pa-rs's 16:00 UTC): cutting this train needs c2pa-rs's own cut to have already landed its dev-cycle bump on `main` first.

## Track 1: additive releases

Low-risk, non-breaking features and bug fixes ship quickly on the current release train:

1. The change lands on `main` (gated by Tier 1A CI like any PR).
2. It is cherry-picked onto the current release line (`stable`) by the [backport bot](#backport-bot) when you add a `backport-stable` label to the merged PR.
3. `release-plz` opens a release PR on `stable`; merging it publishes `0.x.y`.

Key points:

* **Short (about one day) bake.** Additive releases (`0.x.y`, y ≥ 1) don't need a full release-candidate stage, but we do hold a brief bake, approximately one business day, before the crates.io publish, to re-verify things are working as expected.

## Track 2: the breaking train

Breaking changes and larger refactors are batched onto a scheduled train:

1. On the scheduled date, a release-candidate branch `0.(x+1).0-rc` is cut from `main`, and its first build `0.(x+1).0-rc.1` is cut immediately (version set, tagged, prerelease binary built). **RC branches are not published to crates.io** (their name keeps them off every crates.io publish trigger), but each build **does** get a prerelease GitHub release with binaries so downstream consumers that depend on pre-built binaries can validate during the bake. See [RC builds](#release-candidate-builds).
2. **Bake period: minimum three business days.** Only bug fixes are accepted during the bake, and they follow upstream-first (fix on `main`, cherry-pick to the candidate). After fixes land, cut a fresh build (`-rc.2`, `-rc.3`, …) so downstream has updated binaries to test.
3. **Promote** (a deliberate, manual step): first snapshot the outgoing `stable` as `v0.<old>` so the retiring line is available for backports, then force-push the candidate onto `stable`. We force-push rather than merge so that `stable`'s history becomes exactly the coherent set of changes made on `main`, superseding whatever adaptations were needed while manually backporting Track 1 fixes onto the old `stable` line. `release-plz` then opens the `0.(x+1).0` version/changelog PR on `stable`; merging it publishes the breaking release.

### Cadence: scheduled, but not forced

* **Default rhythm: every two months**, on the **second Monday of each odd-numbered month at 16:30 UTC** (30 minutes after c2pa-rs's own train cut -- see [Tracking c2pa-rs main](#tracking-c2pa-rs-main)), published in advance so users can plan migrations.
* **Skip if empty.** If the date arrives with no breaking changes queued, we skip the train.
* **Don't hold the train.** If breaking changes are queued, the candidate is cut on the date regardless. An almost-finished breaking feature waits for the *next* train.
* **Anchor on the cut date,** not the release date, so the bake window absorbs slippage.

### Version numbering across a train

Every train advances the **minor** number by one, regardless of whether the change is actually breaking: a train is, by definition, a new minor line, and we want a clean, predictable number for it. Versions are **set by hand** (e.g. `cargo set-version`) rather than left to release-plz's semver detection.

The convention:

* **`main` always carries the *next* release's version with a `-dev` suffix**, e.g., `0.28.0-dev`. Because `main` is never published, the `-dev` prerelease is purely a label that says "work in progress toward 0.28.0."
* **Cutting the train** for `0.N.0` produces the release-candidate branch `0.N.0-rc` (dropping the numeric suffix from the branch name; the number belongs to each *build*). Its builds are versioned `0.N.0-rc.1`, `0.N.0-rc.2`, … and, while never published to crates.io, are tagged and get prerelease GitHub-release binaries. On promotion the line becomes `0.N.0`.
* **Right after the cut, `main` moves to `0.(N+1).0-dev`** so ongoing development is always numbered ahead of the line that's baking. This bump is committed to `main` automatically by [`release-train-cut.yml`](#release-train-cut) as part of the cut: no separate PR.

## Keeping additive changes additive

The model only works if we stay disciplined about keeping the fast lane non-breaking:

* **Review norm:** "Can this ship additively? If yes, it goes out now. If it requires a break, it waits for the next train."
* **Deprecate-then-remove:** when we must break, add the replacement behavior additively and mark the old one deprecated (a warning in the CLI, a note in the docs) before removing it, giving users a full train's window to migrate.

## Branch lifecycle and support

* A `0.x` release line is **retired when its successor `0.(x+1).0` ships**. By default we support only the latest stable line.
* **Backport exceptions** to a retired line are rare and reserved for a correctness or security issue with no reasonable upgrade path for the affected consumer. Such a backport targets that line's `v0.x` branch.

## Automation

Cutting a release is mostly a CI action rather than manual toil. The pieces:

### release-plz

We use [`release-plz`](https://release-plz.dev) (via the [GitHub Action wrapper](https://github.com/release-plz/action)), configured by [`release-plz.toml`](https://github.com/contentauth/c2patool/blob/main/release-plz.toml). Its two responsibilities are split across two workflows, both of which run on the **release-line and release-candidate branches**, never on `main`:

* [`release-pr.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/release-pr.yml) runs `release-plz release-pr`: it inspects commits since the last tag and opens/updates a **release PR** that bumps the version and updates the changelog.
* [`release.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/release.yml) runs `release-plz release`: when a release PR merges (a push to the release-line branch), it publishes to crates.io, creates a GitHub release, and tags it `v(version)`. That tag then drives the binary build ([`c2patool-release.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/c2patool-release.yml) on any `v*` tag): `release.yml` doesn't build binaries itself, which is what lets release-candidate builds produce the same binaries from the same tags (see [RC builds](#release-candidate-builds)). A push whose ref contains `-rc` never publishes to crates.io.

Binary builds are therefore entirely **tag-driven**, independent of how a tag was created. A tag whose name contains `-rc.` yields a **prerelease** GitHub release; nothing publishes to crates.io in that case.

How `release-plz` chooses a version:

* If only bug-fix commits are detected, bump the patch number (`y`).
* If additions or breaking changes are detected, bump the middle number (`x`). (Pre-1.0, Cargo treats a middle-number bump as incompatible; this becomes the major-number bump after 1.0.)

The set of commit types that trigger a release is configured by `release_commits` in [`release-plz.toml`](https://github.com/contentauth/c2patool/blob/main/release-plz.toml) (chore commits are ignored).

> [!IMPORTANT]
> You may manually edit a proposed changelog in the release PR, but those edits will be overwritten if another update is triggered: `release-plz` force-pushes to update an existing release PR.

### Backport bot

To bring a merged `main` PR onto a release line, add a `backport-<branch>` label to it (e.g. `backport-stable`). On merge, [`backport.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/backport.yml) (using [`korthout/backport-action`](https://github.com/korthout/backport-action)) cherry-picks the change and opens a PR against that branch. Because that PR targets a release-line branch, it must pass the full Tier 1A suite before it can merge (see [validation gating](#validation-gating)).

### Upstream-first check: proactive

[`upstream-first-check.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/upstream-first-check.yml) runs on every PR targeting a release-line or release-candidate branch and **blocks the merge** if the PR introduces a commit whose change is not already on `main` (compared by patch id via `git cherry`). Combined with [branch protection](#branch-protection) that requires PRs on these branches, it makes "nothing originates on a release branch" enforceable.

Two exemptions keep it practical: the `release-plz` release PR (labeled `release`) may legitimately originate version-bump/changelog commits on the release branch, and a maintainer can add the `upstream-first-verified` label to a PR whose cherry-pick had to be adapted to compile on the target branch (so its patch id no longer matches `main`).

### Reconciliation check: reactive

As a backstop to the proactive check above, a scheduled job, [`reconciliation.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/reconciliation.yml), runs `git cherry main <release-branch>`; anything present on the release branch but **not** on `main` means something originated on a release branch, violating upstream-first. The job opens (or updates) an issue so the change can be forward-ported. We deliberately do **not** auto-merge a release branch back into `main`.

### c2pa-rs release bump

[`c2pa-release-bump.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/c2pa-release-bump.yml) opens a PR against `stable` the moment c2pa-rs publishes a new version of the `c2pa` crate, rather than waiting for someone to notice or for a scheduled poll. c2pa-rs's own `release.yml`, right after a successful crates.io publish, dispatches this workflow cross-repo (`repository_dispatch`, authenticated with the `CROSS_ORG_PR_TOKEN` org token) with the new version. `RELEASE_PLZ_ORG_TOKEN` -- used everywhere else in this doc -- is not scoped for cross-repo calls (confirmed empirically: it gets a 403 against c2patool's dispatches endpoint), so this is the one piece of automation here that uses a different org token.

Because `stable`'s `Cargo.toml` pins `c2pa` to a real crates.io version (unlike `main`, which tracks c2pa-rs's `main` branch -- see [Tracking c2pa-rs main](#tracking-c2pa-rs-main)), Cargo's pre-1.0 caret rules mean most new c2pa-rs releases already satisfy the existing `c2pa = "0.x.y"` requirement, so the workflow just needs `cargo update -p c2pa --precise <version>` and opens a PR if `Cargo.lock` changed. The PR is labeled `c2pa-bump`, which exempts it from the [upstream-first check](#upstream-first-check-proactive): `main` never carries an equivalent version-bump commit for `git cherry` to match, since it depends on c2pa-rs via git rather than a version. If the new version falls **outside** stable's requirement (a breaking c2pa-rs release), the job fails loudly instead of widening the requirement unattended -- that case is handled by the [breaking train](#track-2-the-breaking-train) instead.

### Patch-dependency guard

[`check-no-patch-deps.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/check-no-patch-deps.yml) fails if a `[patch]` section or a git dependency is present. It runs on release-branch PRs and as a required prerequisite of `release.yml` -- note that `main`'s own `c2pa` git dependency (see [Tracking c2pa-rs main](#tracking-c2pa-rs-main)) is expected and fine there; this guard just makes sure it never reaches a release branch.

### Release-train cut

[`release-train-cut.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/release-train-cut.yml) runs every Monday and gates on a date check so it only acts on the second Monday of an odd-numbered month (or when dispatched manually with `force: true`). When it fires it computes the next breaking version, applies skip-if-empty, and, if there's breaking work to ship, pushes a new `0.(x+1).0-rc` candidate branch from `main`, kicks off its first build by dispatching [`release-rc.yml`](#release-candidate-builds), **advances `main` to the next `0.(x+2).0-dev` cycle** (committed directly, no PR), and opens a `release-train` tracking issue describing the bake and the manual-promotion step.

### Release-candidate builds

[`release-rc.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/release-rc.yml) cuts a numbered candidate **build** (`-rc.1`, `-rc.2`, …) from a candidate **branch** (`0.N.0-rc`). It sets the `-rc.N` version on the branch, commits, and pushes the `v…-rc.N` tag. That tag triggers the tag-driven binary build above, publishing a **prerelease** GitHub release with binaries. Nothing here reaches crates.io.

The first build (`-rc.1`) is cut automatically when the train is cut. A maintainer re-runs this workflow (via `workflow_dispatch` on the RC branch) to cut a fresh build after bugfixes have been cherry-picked onto the branch during the bake. Tags are pushed with a PAT (`RELEASE_PLZ_ORG_TOKEN`) so the tag-driven binary workflow actually runs: pushes made with the default `GITHUB_TOKEN` do not cascade into other workflows.

## Validation gating

* **Merging to `main`** requires **Tier 1A** (`ci.yml`): the merge gate for everyday development.
* **Any PR targeting a release-line (`stable`, `v0.x`) or release-candidate (`*-rc*`) branch** must pass Tier 1A before it can merge. This includes **backport PRs**, RC bake bugfix PRs, and the `release-plz` release PR: anything headed for a published (or soon-to-be-published) artifact gets the same validation.
* During a train's bake, Tier 1A also runs on every push to the `*-rc*` branch.

See [`docs/support-tiers.md`](support-tiers.md) for the build configurations Tier 1A actually covers.

## Commit lint used for PR title enforcement

Because `release-plz` uses [Conventional Commit syntax](https://www.conventionalcommits.org/en/v1.0.0/#summary) to generate changelogs, all commits to long-lived branches must follow it. We [squash-merge](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/configuring-commit-squashing-for-pull-requests) PRs, and [`pr_title.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/pr_title.yml) checks that each PR title conforms, as configured by [`.commitlintrc.yml`](https://github.com/contentauth/c2patool/blob/main/.commitlintrc.yml) (the definitive specification).

A quick, non-authoritative summary: the PR title must have this exact format:

```
type: description
```

The `type` must be one of (bold = preferred in most cases):

* **`feat`**: a new feature. Use a `!` immediately before the `:` to signal an API breaking change (which queues for the next train).
* **`fix`**: a bug fix.
* **`chore`**: maintenance; does not trigger a release PR and is omitted from the changelog.
* **`docs`**: documentation.
* `build`, `ci`, `perf`, `refactor`, `revert`, `style`, `test`, `update` (the last used by Dependabot).

Unlike c2pa-rs, `scope` is not allowed here: c2patool is a single crate, so `type(scope): description` would never carry information that `type: description` doesn't already. `description` is a short sentence, capitalized, no trailing period, preferably under 70 characters.

> [!NOTE]
> If these rules change, keep [`.github/workflows/pr_title.yml`](https://github.com/contentauth/c2patool/blob/main/.github/workflows/pr_title.yml) and [`.commitlintrc.yml`](https://github.com/contentauth/c2patool/blob/main/.commitlintrc.yml) in sync.

## Troubleshooting

### How to recover if the publish step fails or partially fails

Keep the core mental model in mind (see [`release-plz`](#release-plz)). The following usually works when `release-plz` fails to publish, though it may need adapting to the specific failure:

* **Read the logs** in the [Actions tab](https://github.com/contentauth/c2patool/actions/workflows/release.yml). (`cargo publish` uses a subtly different compilation environment than a normal build, which is a common root cause.)
* **Resolve the underlying issue.**
* **If the failure happened partway through, manually revert `Cargo.toml` and `CHANGELOG.md`** on the release-line branch. `release-plz` only generates a new release PR when `Cargo.toml`'s version exactly matches crates.io; delete the failed `CHANGELOG.md` section too, or `release-plz` will error on the next PR.
* **Wait for `release-plz` to open a fresh release PR** with the desired result, and otherwise **avoid manually editing `Cargo.toml`**: pushing `release-plz` outside its normal process tends to create more problems.

## Branch protection

The upstream-first guarantees rely on release-line and release-candidate branches only receiving changes through PRs. Configure branch protection (a repository setting, not something this repo can commit) on `main`, `stable`, and each `v0.*` / `*-rc*` branch to:

* **Require a pull request before merging**, so nothing is pushed directly, which is what makes the [upstream-first check](#upstream-first-check-proactive) an effective gate rather than an after-the-fact report.
* **Require status checks to pass**, including Tier 1A on `main`, and Tier 1A plus the upstream-first check on release-line/RC branches (see [validation gating](#validation-gating)).

Branch-name patterns (`v0.*`, `*-rc*`) can be covered with a single ruleset each so new release lines and candidates are protected automatically.

Some release automation pushes directly to protected branches and so must be on the ruleset **bypass list**: grant this to the identity behind `RELEASE_PLZ_ORG_TOKEN`:

* [`release-train-cut.yml`](#release-train-cut) commits the next-dev-cycle bump straight to `main`, [`release-rc.yml`](#release-candidate-builds) commits `-rc.N` version bumps straight to the RC branch, and [`track-c2pa-rs-main.yml`](#tracking-c2pa-rs-main) commits its `Cargo.lock` refresh straight to `main`. All three bypass the pull-request rule by design (they are mechanical, unreviewed-by-nature changes).

## One-time setup

The `stable` and `main` branches, and the full tag history, already exist from the repo split -- there's no equivalent of c2pa-rs's original "create `stable` from the latest tag" step to do here. What's still needed before any of this can actually run: the repo secrets, labels, and branch protection listed in the companion PR that added this document.
