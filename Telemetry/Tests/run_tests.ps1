<#
Run all tests for the Telemetry crate. Intended for local use or CI on Windows.
#>
Set-Location -Path (Split-Path -Parent $MyInvocation.MyCommand.Definition)
# Move up to the crate root
Set-Location -Path ".."

Write-Host "Running cargo test for Telemetry crate..."
cargo test --manifest-path .\Cargo.toml
Param()

Write-Host "Running Telemetry crate tests..."
Push-Location -Path (Join-Path $PSScriptRoot "..")
try {
    # Run cargo test for the telemetry crate
    & cargo test --manifest-path Telemetry/Cargo.toml --verbose
    if ($LASTEXITCODE -ne 0) { throw "cargo test failed with exit $LASTEXITCODE" }
} finally {
    Pop-Location
}

Write-Host "Telemetry tests finished."
