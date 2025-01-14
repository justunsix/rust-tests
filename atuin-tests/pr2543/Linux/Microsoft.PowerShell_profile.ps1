$Env:ATUIN_POWERSHELL_PROMPT_OFFSET=-3

$Env:PATH += ":/home/user1/usr/reference/atuin/target/release"
# Temporarily change where atuin stores it's database
# Set environment variables to temporary files
# Otherwise, existing database schema will be migrated to a new format, and
# you will have to stay with the Atuin version from this test branch
$Env:ATUIN_DB_PATH = "/home/user1/tempatuin/temp_atuin_dev.db"
$Env:ATUIN_RECORD_STORE_PATH = "/home/user1/tempatuin/temp_atuin_records.db"

# Install
atuin init powershell | Out-String | Invoke-Expression

# Example activateion of Starship prompt with 3 lines
## Using https://github.com/justunsix/dotfiles/blob/main/.config/starship.toml
if (Get-Command "starship" -ErrorAction SilentlyContinue) {
    Invoke-Expression (&starship init powershell)
}
