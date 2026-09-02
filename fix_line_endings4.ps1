$content = Get-Content scripts/bootstrap/verify_b1_lexer.sh -Raw
$content = $content -replace "`r", ""
Set-Content scripts/bootstrap/verify_b1_lexer.sh -Value $content -NoNewline
