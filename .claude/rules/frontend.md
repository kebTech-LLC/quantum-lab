# Frontend Code Standards — quantum-lab web

> Ported and slimmed from cnctd.world. These patterns are MANDATORY and
> enforced by hooks in `.claude/hooks/`. Vue files are reactive remotes and
> display; ALL business logic lives in TypeScript files.

## HARD RULES (enforced by hooks — violations BLOCKED)

### 1. Template before script (`validate-vue.sh`)
Every `.vue` file: `<template>` FIRST, `<script setup lang="ts">` SECOND,
`<style scoped>` LAST. No exceptions.

### 2. No business logic in components (`validate-vue-logic.sh`)
Forbidden inside `<script>` of any `.vue` file:

| Forbidden | Where it belongs |
|---|---|
| `fetch()` calls | a module in `modules/` |
| `import ... from '@/modules/sim/pkg'` | only `modules/sim/` touches the wasm boundary |
| `async onMounted` | module method, called from a non-async `onMounted` |
| `try/catch` around module/api calls | error handling lives in `modules/` |

## Architecture — four layers

```
COMPONENTS (src/components/)  template bindings, minimal local UI state
    ↓
VIEWS (src/views/)            UI layout state: panels, selections, modes
    ↓
MODELS (src/models/)          data classes: computed getters + behavior methods
    ↓
STATE (src/state/) + MODULES (src/modules/)
                              reactive app data + business logic/infrastructure
```

Components ONLY: import reactive singletons (`sim`, `app`, `views`), bind to
their properties in the template, call methods on modules/models, and hold
minimal local UI state (hover, scroll, toggle refs).

## Patterns

- Reactive singletons: `export const sim = reactive(new Sim())` from a module
  index. Components import them directly — no props drilling, no Pinia.
- Avoid emits for cross-component communication; update shared reactive state
  instead. Exception: tightly-coupled 1:1 parent-child (form input → form).
- Models separate data from behavior: plain interface for the contract, class
  wrapper with getters/methods, `static fromPlainObject()` factory.
- Models delegate UI state to `views`, never store it internally.
- Component size limits: simple display ~150 lines, interactive ~350,
  main/container ~500. Past the limit: extract sub-components or push logic
  down to models/views.

## quantum-lab-specific rules

- The wasm boundary belongs to `modules/sim/` exclusively. Nothing else
  imports from `modules/sim/pkg/` (generated, gitignored).
- Amplitude data crosses the boundary as Float64Array views into wasm memory:
  re-acquired every read, never stored, never serialized per frame.
- The dependency arrow points one way: web depends on the crates. No physics
  in TypeScript — if the frontend needs a quantum computation, it goes in
  `qsv` and gets exposed through the shim.
