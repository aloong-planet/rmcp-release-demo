# Documentation layout

Where this repository's documents live, which of them can be relied on, and which cannot.

## Conclusions and process are two different things

**Conclusions** — answer "what is right now", and any skill may rely on them:

| Location | Question it answers | Maintained by |
|---|---|---|
| `docs/CONTEXT.md` | The exact meaning of each term, and which synonyms to avoid | `/domain-modeling` |
| `docs/adr/` | Why something was decided, and what was rejected | `/domain-modeling` |
| `docs/specs/<slug>.md` | The feature's current behaviour, the boundaries set, and what is explicitly out of scope | `/to-spec` |
| `docs/prototypes/` | Design references for interfaces and state models | `/prototype` |
| `docs/postmortems/` | The causal chain of an incident | `/implement` |
| `docs/ops/` | This project's runbooks (the actual values for signing, releasing and the like) | Defined per project |

**Process** — answers "how it was done at the time":

| Location | Contents |
|---|---|
| `docs/.workings/<slug>/` | Local tickets, the Requirements alignment record, the feature checklist, and the delivery note of each delivery |

**`docs/.workings/` stays out of the repository (the repository's `.gitignore` ignores the whole
directory); its contents are kept, not deleted, but no skill may treat them as a reliable source.** Which
checkout to write into when working in a worktree is set by the "施工记录存储" section of the global
instructions. Read it only to answer "why was this done that way at the time", never to infer the present
state. When it conflicts with a conclusion document, the conclusion document wins, **and the process
record need not be corrected** — it is meant to stay as it was at the time. It is a hidden directory: it
takes no part in publishing the documentation and is not matched by globs such as `docs/**/*.md`.

## Read these first when exploring

Before changing code or proposing a plan, read:

- The table of contents of `docs/CONTEXT.md`, then fetch single entries for the terms you use (how to read
  it is set by the "读项目词汇表 `CONTEXT.md`" section of the global instructions); if `docs/CONTEXT-MAP.md`
  exists, use it to find the `CONTEXT.md` of the context relevant to the topic at hand (format and
  inference rules are in `/domain-modeling`'s `CONTEXT-FORMAT.md`)
- The ADRs in `docs/adr/` that relate to the area being changed; in a multi-context repository, also the
  `docs/adr/` under that context's directory

**When these files do not exist, carry on silently.** Do not point out that they are missing, and do not
suggest creating them in advance — `/domain-modeling` creates them lazily when a term or decision is
actually settled.

## Use the glossary's words

Whenever an output mentions a domain concept (issue titles, proposals, assumptions, test names), **it must
use the word defined in `CONTEXT.md`**, and must not drift to a synonym its `_Avoid_` explicitly lists.

A concept you need that is not in the glossary is a signal: either you are inventing a way of speaking this
project does not use (reconsider), or an entry is genuinely missing (note it down and hand it to
`/domain-modeling`).

## Name any conflict with an existing ADR

If an output contradicts an ADR, **it must say so by name, never override it silently**:

> _Contradicts ADR-0007 (event-sourced orders) — but worth reopening, because…_
