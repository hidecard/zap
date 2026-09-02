$content = Get-Content scripts/bootstrap/aggregate_b1_parser_gates.sh -Raw
$content = $content -replace "`r", ""
Set-Content scripts/bootstrap/aggregate_b1_parser_gates.sh -Value $content -NoNewline
