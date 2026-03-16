# RustySpec prerequisites check
. "$PSScriptRoot\common.ps1"

Write-Host "RustySpec Prerequisites Check"
Write-Host "=============================="

$errors = 0

# Check git
try {
    $gitVersion = git --version 2>$null
    Write-Host "[OK] git: $gitVersion"
} catch {
    Write-Host "[!!] git: not found"
    $errors++
}

# Check project structure
try {
    $root = Get-RepoRoot
    Write-Host "[OK] Project root: $root"

    if (Test-Path "$root\.rustyspec\constitution.md") {
        Write-Host "[OK] Constitution file present"
    } else {
        Write-Host "[!!] Constitution file missing"
        $errors++
    }

    if (Test-Path "$root\rustyspec.toml") {
        Write-Host "[OK] rustyspec.toml found"
    } else {
        Write-Host "[!!] rustyspec.toml missing"
        $errors++
    }
} catch {
    Write-Host "[!!] Not inside a RustySpec project"
    $errors++
}

Write-Host ""
if ($errors -eq 0) {
    Write-Host "All checks passed."
} else {
    Write-Host "$errors issue(s) found."
    exit 1
}
