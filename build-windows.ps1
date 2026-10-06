param(
    [ValidateSet('nsis', 'msi', 'all')]
    [string]$Bundle = 'nsis'
)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if ($env:OS -ne 'Windows_NT') { throw 'Compile este projeto no Windows.' }
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw 'Instale Rust (MSVC) e as ferramentas C++ do Visual Studio. Consulte README.md.'
}
$taskProject = Split-Path -Parent $PSScriptRoot
Push-Location -LiteralPath $taskProject
try {
    & cargo tauri --version
    if ($LASTEXITCODE -ne 0) { throw 'Instale a CLI: cargo install tauri-cli --version "^2" --locked' }
    $taskBundles = if ($Bundle -eq 'all') { 'nsis,msi' } else { $Bundle }
    & cargo tauri build --target x86_64-pc-windows-msvc --bundles $taskBundles
    if ($LASTEXITCODE -ne 0) { throw 'A compilação falhou. Leia o erro acima e README.md.' }
    $taskDist = Join-Path $taskProject 'dist'
    New-Item -ItemType Directory -Path $taskDist -Force | Out-Null
    $taskBundleRoot = Join-Path $taskProject 'src-tauri\target\x86_64-pc-windows-msvc\release\bundle'
    $taskKinds = if ($Bundle -eq 'all') { @('nsis','msi') } else { @($Bundle) }
    foreach ($taskKind in $taskKinds) {
        $taskExtension = if ($taskKind -eq 'nsis') { '*.exe' } else { '*.msi' }
        $taskArtifacts = @(Get-ChildItem -LiteralPath (Join-Path $taskBundleRoot $taskKind) -Filter $taskExtension -File)
        if ($taskArtifacts.Count -eq 0) { throw "Instalador $taskKind não encontrado." }
        $taskArtifacts | Copy-Item -Destination $taskDist -Force
    }
    Copy-Item -LiteralPath 'src-tauri\Cargo.lock' -Destination $taskDist -Force
    Get-ChildItem -LiteralPath $taskDist -File | Where-Object { $_.Name -ne 'SHA256SUMS.txt' } |
        ForEach-Object { '{0}  {1}' -f (Get-FileHash -LiteralPath $_.FullName -Algorithm SHA256).Hash.ToLowerInvariant(), $_.Name } |
        Set-Content -LiteralPath (Join-Path $taskDist 'SHA256SUMS.txt') -Encoding ascii
    Write-Host "Instaladores e hashes: $taskDist"
} finally {
    Pop-Location
}

