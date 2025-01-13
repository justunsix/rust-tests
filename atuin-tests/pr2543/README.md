# Testing Atuin Pull Request #2543 Add PowerShell module

Link to Pull Request (PR): [#2543 Add PowerShell module](https://github.com/atuinsh/atuin/pull/2543)

## Test Steps

### Pre-requisites

- [rustup](https://www.rust-lang.org/tools/install)
- [protobuf (protoc)](https://github.com/protocolbuffers/protobuf)
- Optional - Clear your pwsh and/or PowerShell profiles during testing to be
  sure other modules or environment settings do not conflict with atuin.

### Test on Windows 11

Run on 2025-01-13

```sh
# Remove any installed atuin binary
cargo uninstall atuin

# Get PR
git clone https://github.com/ltrzesniewski/atuin.git
cd atuin
# Switch to branch powershell-pr
git branch --track powershell-pr origin/powershell-pr
git checkout powershell-pr

# Build atuin
cargo build --release

cd target/release

# Run installation per steps explained in first post of https://github.com/atuinsh/atuin/pull/2543

# Temporarily add atuin to path
$Env:Path += ";path\to\target\release"
# Temporarily change where atuin stores it's database
# Set environment variables to temporary files
# Otherwise, existing database schema will be migrated to a new format, and
# you will have to stay with the Atuin version from this test branch
mkdir $Env:USERPROFILE\tempatuin
$env:ATUIN_DB_PATH = "$Env:USERPROFILE\tempatuin\temp_atuin_dev.db"
$env:ATUIN_RECORD_STORE_PATH = "$Env:USERPROFILE\tempatuin\temp_atuin_records.db"

# Verify path change and location of binary
$Env:Path
# Check atuin points to the PR version
Get-Command "atuin"

# Install
atuin init powershell | Out-String | Invoke-Expression

# Type some commands like ls, cd ...
# Verify commands are showing up in atuin history
atuin search -i
# Press up arrow to see same history

```

### Optional - Continue Testing on Nushell

- Continue from the commands above and enter nushell

```sh

# Check atuin points to the PR version
which atuin

# Verify commands are showing up in atuin history
atuin search -i

```

### Clean up Test Environment

- Remove the directory `$Env:USERPROFILE\tempatuin` and test branch release files if desired.

## Test Environment and Versions

From tests run on 2025-01-12

Summary:

- Windows 11 10.0.26100 N/A Build 26100
- PowerShell 7.4.6 with PSReadLine 2.3.5
- PowerShell 5.1.26100.2161 with PSReadLine 2.3.6

```sh
# Windows Version
systeminfo | findstr /B /C:"OS Name" /B /C:"OS Version"
OS Name:                       Microsoft Windows 11 Home
OS Version:                    10.0.26100 N/A Build 26100

# PowerShell
$PSVersionTable

Name                           Value
----                           -----
PSVersion                      7.4.6
PSEdition                      Core
GitCommitId                    7.4.6
OS                             Microsoft Windows 10.0.26100
Platform                       Win32NT
PSCompatibleVersions           {1.0, 2.0, 3.0, 4.0…}
PSRemotingProtocolVersion      2.3
SerializationVersion           1.1.0.1
WSManStackVersion              3.0

Get-Module PSReadLine

ModuleType Version    PreRelease Name                                ExportedCommands
---------- -------    ---------- ----                                ----------------
Script     2.3.5                 PSReadLine                          {Get-PSReadLineKeyHandler, Get-PSReadLineOption, Remove-PSReadLineKeyHandler, Set-PSReadLineKeyHandler…}

# Switch to Windows Powershell 5.1.x
$PSVersionTable

Name                           Value
----                           -----
PSVersion                      5.1.26100.2161
PSEdition                      Desktop
PSCompatibleVersions           {1.0, 2.0, 3.0, 4.0...}
BuildVersion                   10.0.26100.2161
CLRVersion                     4.0.30319.42000
WSManStackVersion              3.0
PSRemotingProtocolVersion      2.3
SerializationVersion           1.1.0.1

Get-Module PSReadLine

ModuleType Version    Name                                ExportedCommands
---------- -------    ----                                ----------------
Script     2.3.6      PSReadLine                          {Get-PSReadLineKeyHandler, Get-PSReadLineOption, Remove-PSReadLineKeyHandler, Set-PSReadLineKeyHandler...}

# rustup and protoc version
rustup --version
rustup 1.27.1 (54dd3d00f 2024-04-24)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.84.0 (9fc6b4312 2025-01-07)`

protoc --version
libprotoc 29.3

# Nushell
nu --version
0.101.0

```

## Error: migration 20230531212437 and Fix

The error below was encountered during testing and a fix is listed below. 

It is not related to changes in the pull request (PR) and is due to changes in the main branch of Atuin. It occurs during testing if the steps to temporarily change where atuin stores it's database are not done.

```sh

Error: migration 20230531212437 was previously applied but has been modified
>>
>> Location:
>>     C:\Users\user1\usr\reference\atuin\crates\atuin-client\src\record\sqlite_store.rs:61:9
Error:: The term 'Error:' is not recognized as a name of a cmdlet, function, script file, or executable program.
Check the spelling of the name, or if a path was included, verify that the path is correct and try again.
Location::
Line |
   3 |  Location:
     |  ~~~~~~~~~
     | The term 'Location:' is not recognized as a name of a cmdlet, function, script file, or executable program.
Check the spelling of the name, or if a path was included, verify that the path is correct and try again.
C:\Users\user1\usr\reference\atuin\crates\atuin-client\src\record\sqlite_store.rs:61:9:
Line |
   4 |      C:\Users\user1\usr\reference\atuin\crates\atuin-client\src\record …
     |      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
     | The term 'C:\Users\user1\usr\reference\atuin\crates\atuin-client\src\record\sqlite_store.rs:61:9' is not
recognized as a name of a cmdlet, function, script file, or executable program.
Check the spelling of the name, or if a path was included, verify that the path is correct and try again.

```

### Fix

- During testing, run the commands in the "Test on Windows 11" section under comment `# Temporarily change where atuin stores it's database`, then retry tests.

#### Explanation

If the step was not done, Atuin will try to migrate the database schema to a new format and files in `~/.local/share/atuin` will be an issue.

- The local `~/.local/share/atuin/records.db` will attempt to be migrated. A not recommended fix is to delete `records.db` and have a new one generated.
- The system could run powershell and nushell fine with atuin and they successfully shared `~/.local/share/atuin/`
