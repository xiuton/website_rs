# Split original styles.scss into component partials (hardcoded markers)
$ErrorActionPreference = "Stop"
$enc = New-Object System.Text.UTF8Encoding($false)
$crlf = "`r`n"
$src = "d:\Code\Rust\website_rs\src\styles"

$blobPath = "$env:TEMP\blob_utf8.bin"
Start-Process -FilePath "git" -ArgumentList "cat-file blob HEAD:src/styles.scss" -NoNewWindow -RedirectStandardOutput $blobPath -Wait
$rawBytes = [System.IO.File]::ReadAllBytes($blobPath)
$text = [System.Text.Encoding]::UTF8.GetString($rawBytes)
$lines = $text -split "`n"
Write-Host "Total: $($lines.Count)"

function Write-Partial($name, $start, $end, $needImports) {
    $content = ($lines[$start..($end-1)] -join $crlf)
    if ($needImports) {
        $header = '@use "sass:map";' + $crlf + "@use 'variables' as *;" + $crlf + $crlf
        $content = $header + $content
    }
    $content = $content.TrimEnd("`r", "`n") + $crlf
    $path = "$src\$name"
    [System.IO.File]::WriteAllText($path, $content, $enc)
    Write-Host "Wrote $name : $($end - $start) lines"
}

# Markers found from proper UTF-8 decoding:
# L0-28  -> _variables.scss  (includes @use "sass:map";)
# L29-166 -> _theme.scss
# L167-199 -> _fonts.scss
# L200-284 -> _base.scss
# L285-2913 -> _layout.scss
# L2914-3021 -> _responsive.scss
# L3022-3340 -> _test.scss
# L3341-4193 -> _playground.scss
# L4194-4363 -> _knowledge_graph.scss
# L4364-end -> _ai_summary.scss

Write-Partial "_variables.scss" 0 29 $false
Write-Partial "_theme.scss" 29 167 $false
Write-Partial "_fonts.scss" 167 200 $false
Write-Partial "_base.scss" 200 285 $true
Write-Partial "_layout.scss" 285 2914 $true
Write-Partial "_responsive.scss" 2914 3022 $true
Write-Partial "_test.scss" 3022 3341 $true
Write-Partial "_playground.scss" 3341 4194 $true
Write-Partial "_knowledge_graph.scss" 4194 4364 $true
Write-Partial "_ai_summary.scss" 4364 4508 $true

Write-Host "All done."