# Собирает и устанавливает CLI pi-clock в Windows. -WithTray добавляет команду tray.
param([switch]$WithTray)
$ErrorActionPreference = 'Stop'
$Root = Split-Path -Parent $MyInvocation.MyCommand.Path

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host 'Rust не найден. Скачиваю rustup и устанавливаю минимальный toolchain…'
    $Rustup = Join-Path $env:TEMP 'rustup-init.exe'
    Invoke-WebRequest -Uri 'https://win.rustup.rs/x86_64' -OutFile $Rustup
    Start-Process -FilePath $Rustup -ArgumentList '-y --profile minimal' -Wait -NoNewWindow
    $CargoBin = Join-Path $HOME '.cargo\bin'
    $env:Path = "$CargoBin;$env:Path"
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        throw 'Rust установлен, но cargo не найден. Перезапустите PowerShell и повторите установку.'
    }
}

$BuildArgs = @('--bin', 'pi-clock')
if ($WithTray) { $BuildArgs += @('--features', 'tray-app') }
Push-Location $Root
try {
    & cargo build --release @BuildArgs
    if ($LASTEXITCODE -ne 0) { throw "cargo завершился с кодом $LASTEXITCODE" }
    $Binary = Join-Path $Root 'target\release\pi-clock.exe'
    $InstallDir = Join-Path $env:LOCALAPPDATA 'Programs\pi-clock'
    & $Binary install --path $InstallDir
    if ($LASTEXITCODE -ne 0) { throw "pi-clock install завершился с кодом $LASTEXITCODE" }
} finally { Pop-Location }

$UserPath = [Environment]::GetEnvironmentVariable('Path', 'User')
$PathEntries = @($UserPath -split ';' | Where-Object { $_ })
if ($PathEntries -notcontains $InstallDir) {
    [Environment]::SetEnvironmentVariable('Path', (($PathEntries + $InstallDir) -join ';'), 'User')
}
$env:Path = "$InstallDir;$env:Path"

Write-Host "Установка завершена: $InstallDir"
Write-Host 'Команды: pi-clock, pi-clock help'
if ($WithTray) { Write-Host 'Трей включён: pi-clock tray' }
