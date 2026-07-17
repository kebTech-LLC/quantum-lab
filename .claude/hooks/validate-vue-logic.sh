#!/bin/bash
# Validate Vue components don't contain business logic.
# BLOCKS writes that put fetch, wasm-boundary imports, or async data loading
# in components. Runs as PostToolUse hook for Edit|Write operations.
# Ported from cnctd.world, adapted for quantum-lab.

INPUT=$(cat)
FILE=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ -z "$FILE" || "$FILE" == "null" ]]; then
    exit 0
fi

if [[ ! "$FILE" =~ \.vue$ ]]; then
    exit 0
fi

if [[ ! -f "$FILE" ]]; then
    exit 0
fi

SCRIPT_CONTENT=$(sed -n '/<script/,/<\/script>/p' "$FILE" 2>/dev/null)

if [[ -z "$SCRIPT_CONTENT" ]]; then
    exit 0
fi

VIOLATIONS=""

if echo "$SCRIPT_CONTENT" | grep -qE '\bfetch\s*\('; then
    VIOLATIONS="${VIOLATIONS}\n  - Direct fetch() call (network access belongs in modules/)"
fi

if echo "$SCRIPT_CONTENT" | grep -qE "modules/sim/pkg"; then
    VIOLATIONS="${VIOLATIONS}\n  - Direct wasm pkg import (only modules/sim/ touches the wasm boundary)"
fi

if echo "$SCRIPT_CONTENT" | grep -qE 'onMounted\s*\(\s*async'; then
    VIOLATIONS="${VIOLATIONS}\n  - async onMounted (move async work to a module method, call it from component)"
fi

if echo "$SCRIPT_CONTENT" | grep -qE 'try\s*\{' && echo "$SCRIPT_CONTENT" | grep -qE '(fetch\(|sim\.|\.init\(|await )'; then
    VIOLATIONS="${VIOLATIONS}\n  - try/catch around module calls (error handling belongs in modules/)"
fi

if [[ -n "$VIOLATIONS" ]]; then
    echo ""
    echo "BLOCKED: Business logic in Vue component: $FILE"
    echo ""
    echo "Violations found:"
    echo -e "$VIOLATIONS"
    echo ""
    echo "Components must be THIN: import reactive singletons (sim, app, views),"
    echo "bind properties in the template, call methods on modules/models, keep"
    echo "only minimal local UI state. See .claude/rules/frontend.md."
    exit 2
fi

exit 0
