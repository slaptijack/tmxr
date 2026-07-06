# Architecture

tmxr is organized around a simple principle:

One responsibility per component.

The project favors small modules with explicit interfaces over large,
general-purpose frameworks.

When adding functionality:

1. Keep parsing separate from execution.
2. Keep business logic independent of the CLI.
3. Keep side effects isolated.
4. Make behavior easy to test.

New abstractions should only be introduced after multiple concrete use cases
demonstrate the need.
