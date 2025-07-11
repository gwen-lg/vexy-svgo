#!/usr/bin/env bash
DIRNAME="$(dirname "$1")"
FILENAME="$(basename "$1")"
PROMPT="Read 'PLUGIN_MIGRATE.md' and then check if the plugin '$FILENAME' has already been migrated from './vexy_svgo/src/plugins/' to './crates/plugin-sdk/src/plugins/'. If not, migrate it."
echo $PROMPT
echo $FILENAME
export GOOGLE_CLOUD_PROJECT="powerful-tree-444423-k8"
gemi -p "$PROMPT"
