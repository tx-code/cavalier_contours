param(
    [string]$BaselinePath = ".planning/tools/cpp_suite_drift_baseline.json",
    [string]$SourceRoot = ""
)

$ErrorActionPreference = "Stop"

if (!(Test-Path -LiteralPath $BaselinePath)) {
    Write-Error "Baseline file not found: $BaselinePath"
}

$baseline = Get-Content -Raw -LiteralPath $BaselinePath | ConvertFrom-Json
if (-not $baseline.files) {
    Write-Error "Baseline file has no 'files' entries: $BaselinePath"
}

if ([string]::IsNullOrWhiteSpace($SourceRoot)) {
    $SourceRoot = $baseline.source_root
}

if ([string]::IsNullOrWhiteSpace($SourceRoot)) {
    Write-Error "SourceRoot is empty and baseline.source_root is not set."
}

if (!(Test-Path -LiteralPath $SourceRoot)) {
    Write-Error "Source root not found: $SourceRoot"
}

$pattern = '^\s*TEST(?:_F|_P)?\s*\(\s*[^,]+,\s*([^)]+)\)'
$drifts = New-Object System.Collections.Generic.List[object]

function Get-TestBlocks {
    param([string]$FilePath, [string]$RegexPattern)

    $blocks = New-Object System.Collections.Generic.List[string]
    foreach ($line in Get-Content -LiteralPath $FilePath) {
        if ($line -match $RegexPattern) {
            $blocks.Add($matches[1].Trim())
        }
    }
    return $blocks
}

Write-Host "CPP suite drift check baseline: $BaselinePath"
Write-Host "CPP suite source root: $SourceRoot"

foreach ($entry in $baseline.files) {
    $relativePath = [string]$entry.relative_path
    $fullPath = Join-Path $SourceRoot $relativePath

    if (!(Test-Path -LiteralPath $fullPath)) {
        $drifts.Add([pscustomobject]@{
                file = $relativePath
                type = "missing_file"
                detail = "File not found"
            })
        continue
    }

    $actualHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $fullPath).Hash.ToLowerInvariant()
    $expectedHash = ([string]$entry.sha256).ToLowerInvariant()

    if ($actualHash -ne $expectedHash) {
        $drifts.Add([pscustomobject]@{
                file = $relativePath
                type = "hash_mismatch"
                detail = "expected=$expectedHash actual=$actualHash"
            })
    }

    $actualBlocks = @(Get-TestBlocks -FilePath $fullPath -RegexPattern $pattern)
    $expectedBlocks = @($entry.test_blocks)

    $sameCount = $actualBlocks.Count -eq $expectedBlocks.Count
    $sameOrder = $sameCount -and ((0..($expectedBlocks.Count - 1) | ForEach-Object { $expectedBlocks[$_] -eq $actualBlocks[$_] }) -notcontains $false)

    if (-not $sameOrder) {
        $missing = @($expectedBlocks | Where-Object { $_ -notin $actualBlocks })
        $added = @($actualBlocks | Where-Object { $_ -notin $expectedBlocks })
        $drifts.Add([pscustomobject]@{
                file = $relativePath
                type = "test_blocks_mismatch"
                detail = "expected_count=$($expectedBlocks.Count) actual_count=$($actualBlocks.Count); missing=[$($missing -join ', ')]; added=[$($added -join ', ')]"
            })
    }
}

if ($drifts.Count -gt 0) {
    Write-Host ""
    Write-Host "CPP suite drift detected:"
    foreach ($d in $drifts) {
        Write-Host " - $($d.file): $($d.type) :: $($d.detail)"
    }
    exit 1
}

Write-Host "CPP suite drift check passed: no file or test-block changes detected."
exit 0
