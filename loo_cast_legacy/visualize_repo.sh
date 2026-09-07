#!/usr/bin/env bash

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$repo_root"

if [[ -x "$repo_root/gource.exe" ]]; then
  gource_bin="$repo_root/gource.exe"
elif command -v gource >/dev/null 2>&1; then
  gource_bin="$(command -v gource)"
else
  echo "Gource not found. Install it or place gource.exe in the repo root."
  exit 1
fi

# Core pacing knob: 1 year of repo time = 30 seconds of playback.
year_seconds="${YEAR_SECONDS:-30}"
seconds_per_day="$(awk -v y="$year_seconds" 'BEGIN { printf "%.6f", y/365.2425 }')"

# Defaults are tuned for a solo-dev history with long active periods.
"$gource_bin" \
  --title "Loo-Cast History" \
  --seconds-per-day "$seconds_per_day" \
  --auto-skip-seconds "${AUTO_SKIP_SECONDS:-1}" \
  --file-idle-time "${FILE_IDLE_TIME:-0}" \
  --max-file-lag "${MAX_FILE_LAG:-0.15}" \
  --hide mouse,progress,filenames,dirnames,usernames \
  --highlight-users \
  --highlight-dirs \
  --follow-user "${FOLLOW_USER:-$(git config user.name || true)}" \
  --camera-mode overview \
  --multi-sampling \
  --dont-stop \
  "${@}"
