# One-command Windows installer. Requires no Rust when a GitHub Release binary exists.
$ErrorActionPreference = 'Stop'
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
$Repo = if ($env:PI_CLOCK_REPO) { $env:PI_CLOCK_REPO } else { 'RUdolf1517/pi-clock' }
$Branch = if ($env:PI_CLOCK_BRANCH) { $env:PI_CLOCK_BRANCH } else { 'master' }
$InstallDir = if ($env:PI_CLOCK_INSTALL_DIR) { $env:PI_CLOCK_INSTALL_DIR } else { Join-Path $env:LOCALAPPDATA 'Programs\pi-clock' }
$Version = $env:PI_CLOCK_VERSION
$Target = 'x86_64-pc-windows-msvc'
$Asset = "pi-clock-$Target.zip"
if ($Version) {
    $ReleaseUrl = "https://github.com/$Repo/releases/download/v$Version"
} else {
    $ReleaseUrl = "https://github.com/$Repo/releases/latest/download"
}

if ($env:PROCESSOR_ARCHITECTURE -notin @('AMD64', 'x86_64')) {
    throw "Unsupported Windows architecture: $env:PROCESSOR_ARCHITECTURE (currently x64 only)."
}

$Temp = Join-Path $env:TEMP ([Guid]::NewGuid().ToString())
New-Item -ItemType Directory -Force -Path $Temp | Out-Null
try {
    $Archive = Join-Path $Temp $Asset
    $Downloaded = $false
    try {
        Invoke-WebRequest -Uri "$ReleaseUrl/$Asset" -OutFile $Archive
        $Downloaded = $true
    } catch {
        Write-Warning "No prebuilt release found for $Target."
    }

    if ($Downloaded) {
        $ChecksumFile = "$Archive.sha256"
        Invoke-WebRequest -Uri "$ReleaseUrl/$Asset.sha256" -OutFile $ChecksumFile
        $ExpectedHash = ((Get-Content $ChecksumFile -Raw).Trim() -split '\s+')[0].ToLowerInvariant()
        $ActualHash = (Get-FileHash $Archive -Algorithm SHA256).Hash.ToLowerInvariant()
        if ($ExpectedHash -ne $ActualHash) { throw 'Release checksum verification failed.' }
        Expand-Archive -Path $Archive -DestinationPath $Temp -Force
        $Binary = Join-Path $Temp 'pi-clock.exe'
    } else {
        if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
            throw "No prebuilt release and Cargo is not installed. See https://github.com/$Repo/releases"
        }
        Write-Host 'Building from source with Cargo…'
        $CargoRoot = Join-Path $Temp 'cargo'
        & cargo install --git "https://github.com/$Repo" --branch $Branch --root $CargoRoot --force --features tray-app
        if ($LASTEXITCODE -ne 0) { throw "cargo install failed with exit code $LASTEXITCODE" }
        $Binary = Join-Path $CargoRoot 'bin\pi-clock.exe'
    }

    if (-not (Test-Path $Binary)) { throw "The package does not contain pi-clock.exe: $Binary" }
    & $Binary install --path $InstallDir
    if ($LASTEXITCODE -ne 0) { throw "pi-clock install failed with exit code $LASTEXITCODE" }
} finally {
    Remove-Item -Recurse -Force $Temp -ErrorAction SilentlyContinue
}

$UserPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$PathEntries = @($UserPath -split ';' | Where-Object { $_ })
if ($PathEntries -notcontains $InstallDir) {
    [Environment]::SetEnvironmentVariable('Path', (($PathEntries + $InstallDir) -join ';'), 'User')
}
$env:Path = "$InstallDir;$env:Path"
Write-Host "Installed: $InstallDir"
Write-Host 'Run: pi-clock (TUI) · pi-clock help (commands)'
Write-Host 'System tray command is included in Windows release builds: pi-clock tray'
