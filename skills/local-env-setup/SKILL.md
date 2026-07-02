---
name: local-env-setup
description: Configure the local wisp-science runtime when uv or Python is missing — first-run bootstrap, python tool, and bundled MCP servers. Use when the user installed wisp-science but python/uv checks fail, Capabilities shows Python or uv missing, bootstrap errors mention "uv not found" or "Python environment", or the user asks to 配置环境 / install Python / install uv / set up the local environment. Not for remote GPU/SSH compute (use compute-env-setup).
license: Apache-2.0
tags: bootstrap, uv, python, install, macos, windows, linux
---

# Local runtime setup (uv + Python)

wisp-science does **not** ship Python. It needs **`uv`** on PATH (or `UV_PATH`) so the app can create a managed venv and install MCP/kernel deps on first run.

Users do **not** need a separate Python installer if `uv` is present — `uv python install` is enough.

## What gets created automatically

After `uv` is visible to wisp-science:

1. `uv venv` → managed virtualenv under app data
2. `uv pip install -r …/python/requirements-mcp.txt` → MCP + kernel deps
3. Marker file `.wisp_deps_ok` inside the venv when deps succeed

**Desktop app venv** (default):

| OS | Path |
|---|---|
| Windows | `%APPDATA%\science.wisp-science\wisp-science\python\.venv` |
| macOS | `~/Library/Application Support/science.wisp-science/wisp-science/python/.venv` |
| Linux | `~/.local/share/science.wisp-science/wisp-science/python/.venv` (XDG; may vary) |

**CLI / dev checkout**: `<workspace>/.wisp/python/.venv`

Restart wisp-science after changing PATH or installing uv so bootstrap re-runs.

## Step 0 — Detect platform and current state

Read the **Environment** section in the system prompt (`Operating system`, `Working directory`).

Run checks with the **`shell`** tool (PowerShell on Windows, `sh -c` on macOS/Linux):

**Windows (PowerShell):**

```powershell
$PSVersionTable.PSVersion; Get-Command uv -ErrorAction SilentlyContinue | Select-Object Source; python --version 2>$null; uv --version 2>$null
```

**macOS / Linux (bash/zsh via sh):**

```sh
uname -srm; command -v uv; uv --version 2>/dev/null; command -v python3; python3 --version 2>/dev/null
```

In the desktop UI: **Capabilities** (能力) shows `Python: … · uv: …`. Both should be ready after a successful bootstrap.

## Step 1 — Install uv

Pick **one** method for the user's OS. Prefer non-admin user installs. After install, open a **new** terminal or restart wisp-science so PATH updates.

### Windows

**Option A — official installer (recommended):**

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

**Option B — winget:**

```powershell
winget install --id astral-sh.uv -e
```

Default location: `%USERPROFILE%\.local\bin\uv.exe`. Ensure that directory is on the user PATH.

### macOS

**Option A — official installer (recommended):**

```sh
curl -LsSf https://astral.sh/uv/install.sh | sh
```

Adds `uv` to `~/.local/bin` (installer prints the exact PATH line).

**Option B — Homebrew:**

```sh
brew install uv
```

Apple Silicon and Intel both use the same commands; verify with `uname -m` if you need to explain architecture.

### Linux

Same as macOS installer script, or distro package if the user prefers:

```sh
curl -LsSf https://astral.sh/uv/install.sh | sh
```

## Step 2 — Install Python via uv (if system Python is absent)

Only needed when bootstrap still fails after uv is on PATH:

```sh
uv python install 3.11
uv python list
```

On Windows via PowerShell, the same `uv` subcommands work once `uv` is on PATH.

wisp-science targets **Python 3.11+** for the managed venv.

## Step 3 — Manual bootstrap (when auto-setup failed)

Use when the app reports `Python environment: …` but uv is installed. Replace `APP_DATA` with the path from the table above.

**macOS / Linux:**

```sh
# macOS example — adjust APP_DATA on Linux (see table above)
APP_DATA="$HOME/Library/Application Support/science.wisp-science/wisp-science"
REQ="/path/to/wisp-science/python/requirements-mcp.txt"   # repo root or bundled resource
mkdir -p "$APP_DATA/python"
uv venv "$APP_DATA/python/.venv"
uv pip install -r "$REQ" --python "$APP_DATA/python/.venv/bin/python"
```

For a dev checkout, set `REQ` to `<repo>/python/requirements-mcp.txt`. For the installed app, the file is bundled in app resources — if missing, ask the user to reinstall.

**Windows (PowerShell):**

```powershell
$AppData = Join-Path $env:APPDATA "science.wisp-science\wisp-science"
New-Item -ItemType Directory -Force -Path (Join-Path $AppData "python") | Out-Null
$Venv = Join-Path $AppData "python\.venv"
uv venv $Venv
# Point -r at the bundled requirements-mcp.txt from the install or repo clone:
uv pip install -r "C:\path\to\wisp-science\python\requirements-mcp.txt" --python (Join-Path $Venv "Scripts\python.exe")
```

Then restart wisp-science and re-check Capabilities.

## Step 4 — Verify

**macOS / Linux:**

```sh
uv --version
"$APP_DATA/python/.venv/bin/python" -c "import mcp, pandas; print('ok')"
```

**Windows:**

```powershell
uv --version
& "$env:APPDATA\science.wisp-science\wisp-science\python\.venv\Scripts\python.exe" -c "import mcp, pandas; print('ok')"
```

Success: import prints `ok`, Capabilities shows Python and uv ready, `python` tool runs without "uv not found".

## Workarounds

| Issue | Fix |
|---|---|
| uv installed but app still says missing | Restart app; confirm `uv` in the **same user's** PATH the GUI inherits (macOS: launch from Dock after shell profile updated). |
| Cannot modify PATH | Set `UV_PATH` to the full path of the `uv` binary before launching wisp-science. |
| Corporate proxy / TLS errors | Configure `HTTPS_PROXY` / system trust store; retry `uv pip install`. |
| Old venv, corrupt deps | Delete `python/.venv` under app data and restart (bootstrap recreates). |
| User only has CLI checkout | Run from repo root; venv is `.wisp/python/.venv`; still requires uv on PATH. |

## Agent workflow

1. `use_skill` this file when bootstrap or python/MCP failures appear.
2. Detect OS — **do not** give PowerShell-only steps on macOS, or bash-only steps on Windows.
3. Install uv → optionally `uv python install 3.11` → verify → ask user to **restart wisp-science**.
4. If still failing, manual bootstrap with paths above, then verify imports.
5. Finish with **attempt_completion** listing what was installed, paths checked, and whether Capabilities should now show ready.

## Not in scope

- Remote GPU / SSH / Modal environments → `compute-env-setup`
- Installing conda, pyenv, or system-wide Python when `uv python install` suffices
- API keys or model provider settings
