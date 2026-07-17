#!/bin/bash
# Validate Vue component structure after Claude edits/writes.
# BLOCKS writes that violate the template-before-script rule.
# Runs as PostToolUse hook for Edit|Write operations.
# Ported from cnctd.world.

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

TEMPLATE_LINE=$(grep -n '<template' "$FILE" 2>/dev/null | head -1 | cut -d: -f1)
SCRIPT_LINE=$(grep -n '<script' "$FILE" 2>/dev/null | head -1 | cut -d: -f1)

if [[ -n "$TEMPLATE_LINE" && -n "$SCRIPT_LINE" ]]; then
    if [[ "$SCRIPT_LINE" -lt "$TEMPLATE_LINE" ]]; then
        echo ""
        echo "BLOCKED: Vue component structure violation in $FILE"
        echo ""
        echo "<script> at line $SCRIPT_LINE precedes <template> at line $TEMPLATE_LINE."
        echo "Required order: 1. <template>  2. <script setup lang=\"ts\">  3. <style scoped>"
        echo "See .claude/rules/frontend.md."
        exit 2
    fi
fi

exit 0
