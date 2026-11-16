# Specs

Each file here describes one feature as it behaves now: what it does, the boundaries it keeps and what it
explicitly does not do. A spec is rewritten in place whenever the feature changes; its history lives in
git, not in the file. A feature that is removed loses its spec and its row below.

- One spec per feature, named `<slug>.md`; requirement and acceptance identifiers take the form
  `<slug>::US-001/AC-01`.
- Specs state behaviour and refer to ADRs for the trade-offs behind it; they do not describe how the code
  is organised, how it is tested, or how a rule came about.
- Specs never point at tickets, branches or process records, which are cleaned up; a fact that is not true
  yet is written as a `> Pending: <condition>` note whose condition can be checked against the product.

| Spec | In one line |
|---|---|
| [Distribution](distribution.md) | How users install, upgrade and remove the server on each platform, and how a version tag turns into a checked release |
