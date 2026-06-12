$orig = [System.IO.File]::ReadAllText("$env:TEMP\orig.scss")
$origLines = $orig -split "`r`n|`n"

function Read-Partial($filename, $stripHeader) {
    $content = [System.IO.File]::ReadAllText($filename)
    if ($stripHeader) {
        $content = $content -replace "@use.*\r?\n", ""
        $content = $content -replace "@use.*\r?\n", ""
    }
    # ensure trailing newline
    if (-not $content.EndsWith("`n")) { $content += "`n" }
    return $content
}

$combined = ""
$combined += (Read-Partial "d:\Code\Rust\website_rs\src\_variables.scss" $false)
$combined += (Read-Partial "d:\Code\Rust\website_rs\src\_theme.scss" $false)
$combined += (Read-Partial "d:\Code\Rust\website_rs\src\_fonts.scss" $false)
$combined += (Read-Partial "d:\Code\Rust\website_rs\src\_base.scss" $true)
$combined += (Read-Partial "d:\Code\Rust\website_rs\src\_layout.scss" $true)
$combined += (Read-Partial "d:\Code\Rust\website_rs\src\_responsive.scss" $true)
$combined += (Read-Partial "d:\Code\Rust\website_rs\src\_test.scss" $true)
$combined += (Read-Partial "d:\Code\Rust\website_rs\src\_playground.scss" $true)
$combined += (Read-Partial "d:\Code\Rust\website_rs\src\_knowledge_graph.scss" $true)
$combined += (Read-Partial "d:\Code\Rust\website_rs\src\_ai_summary.scss" $true)

$combLines = ($combined.TrimEnd("`r`n") -split "`r`n|`n")

Write-Host "Original lines: $($origLines.Count)"
Write-Host "Combined lines: $($combLines.Count)"

$maxLines = [Math]::Max($origLines.Count, $combLines.Count)
$diffCount = 0
for ($i = 0; $i -lt $maxLines; $i++) {
    $o = if ($i -lt $origLines.Count) { $origLines[$i] } else { "<MISSING>" }
    $c = if ($i -lt $combLines.Count) { $combLines[$i] } else { "<MISSING>" }
    if ($o -ne $c) {
        if ($diffCount -lt 20) {
            Write-Host "DIFF line $($i+1):"
            Write-Host "  ORIG: [$o]"
            Write-Host "  COMB: [$c]"
        }
        $diffCount++
    }
}
Write-Host "Total differences: $diffCount"