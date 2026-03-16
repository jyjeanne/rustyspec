# Setup plan supporting files for a feature
param([string]$Feature)

. "$PSScriptRoot\common.ps1"

$root = Get-RepoRoot
if (-not $Feature) { $Feature = Get-CurrentBranch }

$featureDir = Join-Path $root "specs\$Feature"
if (-not (Test-Path $featureDir)) {
    throw "Feature directory not found: $featureDir"
}

$date = Get-Date -Format "yyyy-MM-dd"

# Create research.md if missing
$researchPath = Join-Path $featureDir "research.md"
if (-not (Test-Path $researchPath)) {
    @"
# Research: $Feature

**Date**: $date

## Technology Investigation

[Research findings to be filled]
"@ | Set-Content $researchPath
    Write-Host "Created research.md"
}

# Create data-model.md if missing
$dataModelPath = Join-Path $featureDir "data-model.md"
if (-not (Test-Path $dataModelPath)) {
    @"
# Data Model: $Feature

## Entities

[Entities to be defined based on spec]
"@ | Set-Content $dataModelPath
    Write-Host "Created data-model.md"
}

# Create quickstart.md if missing
$quickstartPath = Join-Path $featureDir "quickstart.md"
if (-not (Test-Path $quickstartPath)) {
    @"
# Quickstart: $Feature

## Key Validation Scenarios

[Validation scenarios to be defined]
"@ | Set-Content $quickstartPath
    Write-Host "Created quickstart.md"
}

# Create contracts
$contractsDir = Join-Path $featureDir "contracts"
New-Item -ItemType Directory -Path $contractsDir -Force | Out-Null
$apiPath = Join-Path $contractsDir "api.md"
if (-not (Test-Path $apiPath)) {
    @"
# API Contracts: $Feature

[To be defined based on plan]
"@ | Set-Content $apiPath
    Write-Host "Created contracts/api.md"
}

Write-Host "Plan setup complete for $Feature"
