$content = Get-Content scripts/bootstrap/verify_b1_token_native_indentation.sh -Raw
$content = $content -replace "`r", ""
Set-Content scripts/bootstrap/verify_b1_token_native_indentation.sh -Value $content -NoNewline
