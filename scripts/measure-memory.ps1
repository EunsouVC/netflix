$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
$taskSnapshot = @(Get-CimInstance Win32_Process)
$taskRoots = @($taskSnapshot | Where-Object { $_.Name -eq 'lume-netflix.exe' })
if ($taskRoots.Count -eq 0) { throw 'Abra o Lume Netflix antes de medir.' }
$taskIds = [System.Collections.Generic.HashSet[uint32]]::new()
foreach ($taskRoot in $taskRoots) { [void]$taskIds.Add([uint32]$taskRoot.ProcessId) }
do {
    $taskChanged = $false
    foreach ($taskProcess in $taskSnapshot) {
        if ($taskIds.Contains([uint32]$taskProcess.ParentProcessId)) { if ($taskIds.Add([uint32]$taskProcess.ProcessId)) { $taskChanged = $true } }
    }
} while ($taskChanged)
$taskRows = @(foreach ($taskId in $taskIds) { try { $taskLive = Get-Process -Id $taskId -ErrorAction Stop; [PSCustomObject]@{PID=$taskId;Processo=$taskLive.ProcessName;WorkingSetMiB=[math]::Round($taskLive.WorkingSet64 / 1MB,1);PrivateCommitMiB=[math]::Round($taskLive.PrivateMemorySize64 / 1MB,1)} } catch { Write-Verbose "Processo $taskId encerrou durante a medição." } })
$taskRows | Sort-Object PID | Format-Table -AutoSize
Write-Host ('Processos observados: {0}' -f $taskRows.Count)
if ($taskRows.Count -gt 0) { Write-Host ('Soma working sets (MiB): {0:N1}' -f (($taskRows | Measure-Object WorkingSetMiB -Sum).Sum)); Write-Host ('Memória privada comprometida (MiB): {0:N1}' -f (($taskRows | Measure-Object PrivateCommitMiB -Sum).Sum)) }
Write-Host 'Fotografia aproximada. Working sets somados repetem páginas compartilhadas; memória comprometida não equivale a RAM residente.'
