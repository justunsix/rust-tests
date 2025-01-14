$Env:ATUIN_POWERSHELL_PROMPT_OFFSET=-3

$Env:Path += ";$Env:USERPROFILE\reference\atuin\target\release"
$env:ATUIN_DB_PATH = "$Env:USERPROFILE\tempatuin\temp_atuin_dev.db"
$env:ATUIN_RECORD_STORE_PATH = "$Env:USERPROFILE\tempatuin\temp_atuin_records.db"

atuin init powershell | Out-String | Invoke-Expression

# Example activateion of Starship prompt with 3 lines
## Using https://github.com/justunsix/dotfiles/blob/main/.config/starship.toml
if (Get-Command "starship" -ErrorAction SilentlyContinue) {
    Invoke-Expression (&starship init powershell)
}


