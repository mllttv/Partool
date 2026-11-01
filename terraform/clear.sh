#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ALL_MODE=false
[[ " $* " =~ " --all " ]] && ALL_MODE=true

rm -f "$SCRIPT_DIR"/terraform.tfstate "$SCRIPT_DIR"/terraform.tfstate.*
rm -f "$SCRIPT_DIR"/crash.log "$SCRIPT_DIR"/crash.*.log
$ALL_MODE && rm -rf "$SCRIPT_DIR/.terraform" "$SCRIPT_DIR/.terraform.lock.hcl"
rm -f "$SCRIPT_DIR/../cloudbuild.yaml"

if ! $ALL_MODE && [ -f "$SCRIPT_DIR/terraform.tfvars" ] && [ -d "$SCRIPT_DIR/.terraform" ]; then
    PROJECT_ID=$(sed -n 's/^[[:space:]]*project_id[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$SCRIPT_DIR/terraform.tfvars")
    REGION=$(sed -n 's/^[[:space:]]*region[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$SCRIPT_DIR/terraform.tfvars")
    APP_NAME=$(sed -n 's/^[[:space:]]*app_name[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$SCRIPT_DIR/terraform.tfvars")
    GCS_BUCKET=$(sed -n 's/^[[:space:]]*gcs_bucket[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$SCRIPT_DIR/terraform.tfvars")
    : "${REGION:=us-central1}"
    : "${GCS_BUCKET:=${PROJECT_ID}-${APP_NAME}-var-state}"

    terraform -chdir="$SCRIPT_DIR" import google_storage_bucket.stateful_sync "$GCS_BUCKET" >/dev/null 2>&1 || true
    terraform -chdir="$SCRIPT_DIR" import google_artifact_registry_repository.repo "projects/${PROJECT_ID}/locations/${REGION}/repositories/${APP_NAME}" >/dev/null 2>&1 || true
    terraform -chdir="$SCRIPT_DIR" import google_cloud_run_v2_service.app "projects/${PROJECT_ID}/locations/${REGION}/services/${APP_NAME}" >/dev/null 2>&1 || true
fi
