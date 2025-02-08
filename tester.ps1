$headers = @{
    "Content-Type" = "application/json"
}

$body = @{
    url = "https://example.com"
    platform = "mastodon"
} | ConvertTo-Json -Depth 10

$response = Invoke-WebRequest -Uri "http://localhost:8787/make-fedilink" `
    -Method Post `
    -Headers $headers `
    -Body $body

Write-Output $response.Content