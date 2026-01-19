# Lab 1: Git Workflow for Detections

## Objective
Practice the git workflow for detection development.

## Prerequisites
- Git installed
- Text editor
- Basic command line skills

## Tasks

### Part 1: Repository Setup (10 min)

1. Initialize a new git repository:
```bash
mkdir my-detections
cd my-detections
git init
```

2. Create directory structure:
```bash
mkdir -p rules/windows rules/linux tests
```

3. Create initial README:
```bash
cat > README.md << 'ENDMARKER'
# My Detection Rules

A collection of detection rules for security monitoring.
ENDMARKER
```

4. Make initial commit:
```bash
git add .
git commit -m "Initial commit: repository structure"
```

### Part 2: Feature Branch Workflow (20 min)

1. Create a new branch for SSH brute force detection:
```bash
git checkout -b detection/ssh-brute-force
```

2. Create the detection rule:
```bash
cat > rules/linux/ssh_brute_force.yml << 'ENDMARKER'
title: SSH Brute Force Attack
id: 12345678-1234-1234-1234-123456789abc
status: experimental
description: Detects multiple failed SSH login attempts
author: Your Name
date: 2024/01/01
tags:
    - attack.credential_access
    - attack.t1110.001
logsource:
    product: linux
    service: auth
detection:
    selection:
        event_type: 'authentication_failure'
        service: 'sshd'
    condition: selection | count() > 5 by source_ip | timeframe 5m
falsepositives:
    - Misconfigured automation
level: high
ENDMARKER
```

3. Commit with proper message:
```bash
git add rules/linux/ssh_brute_force.yml
git commit -m "feat: Add SSH brute force detection

Detects >5 failed SSH authentication attempts within 5 minutes
from the same source IP.

MITRE: T1110.001 - Brute Force: Password Guessing  
Data Source: Linux auth logs
FP Rate: Low (estimated <1%)
Testing: Validated against simulated attack traffic"
```

### Part 3: Code Review Simulation (15 min)

1. Create a second detection on a new branch:
```bash
git checkout main
git checkout -b detection/sudo-escalation
```

2. Create sudo privilege escalation detection:
```bash
cat > rules/linux/sudo_privilege_escalation.yml << 'ENDMARKER'
title: Suspicious Sudo Privilege Escalation
id: abcdef12-3456-7890-abcd-ef1234567890
status: stable
description: Detects unusual sudo usage patterns
author: Your Name
date: 2024/01/01
tags:
    - attack.privilege_escalation
    - attack.t1548.003
logsource:
    product: linux
    service: auth
detection:
    selection:
        event_type: 'sudo_command'
        command|contains:
            - '/bin/bash'
            - '/bin/sh'
    condition: selection
falsepositives:
    - Legitimate admin activity
level: medium
ENDMARKER
```

3. Commit the change:
```bash
git add rules/linux/sudo_privilege_escalation.yml
git commit -m "feat: Add sudo privilege escalation detection"
```

### Part 4: Merging and History (10 min)

1. Merge first detection:
```bash
git checkout main
git merge detection/ssh-brute-force
```

2. Merge second detection:
```bash
git merge detection/sudo-escalation
```

3. View commit history:
```bash
git log --oneline --graph --all
```

## Deliverables

- [ ] Git repository with proper structure
- [ ] Two detection rules committed
- [ ] Proper commit message format
- [ ] Clean git history

## Validation

Check your work:
```bash
# Should show 2 detection rules
ls -la rules/linux/

# Should show clean history
git log --oneline

# Should show no uncommitted changes
git status
```

## Bonus Challenges

1. Add a `.gitignore` file
2. Create a `CONTRIBUTING.md` guide
3. Practice interactive rebase
4. Create and resolve a merge conflict

## Submission

Run these commands and save the output:
```bash
git log --oneline > git-history.txt
tree . > directory-structure.txt
```
