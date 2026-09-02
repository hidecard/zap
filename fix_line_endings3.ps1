$content = Get-Content scripts/bootstrap/aggregate_b1_parser_gates.sh -Raw
$content = $content -replace "`r", ""
Set-Content scripts/bootstrap/aggregate_b1_parser_gates.sh -Value $content -NoNewline

$content = Get-Content scripts/bootstrap/verify_b2_recursive_alias.sh -Raw
$content = $content -replace "`r", ""
Set-Content scripts/bootstrap/verify_b2_recursive_alias.sh -Value $content -NoNewline

$content = Get-Content scripts/bootstrap/verify_b1_boundary_fixtures.sh -Raw
$content = $content -replace "`r", ""
Set-Content scripts/bootstrap/verify_b1_boundary_fixtures.sh -Value $content -NoNewline
