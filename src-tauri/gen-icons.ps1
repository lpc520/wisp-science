$ErrorActionPreference = "Stop"
Push-Location $PSScriptRoot
try {
    cargo tauri icon "../ui/logo.svg"
} finally {
    Pop-Location
}
