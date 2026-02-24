# TODO workbench

## TODO 2026-02-22

- create workbench in any given folder (alert if folder is not valid date `YYYY-MM-DD`)
  - create logs subdir
  - create `.gitignore` with initial content defined below
  - run `git init` within workbench dir (and git -add if files already exist in that dir.), and add logs dir to .gitignore
  - git add and git commit if any files already exist while excluding ./logs

- workbench commands/features:
  - `workbench sh` spawns a bash-compatible REPL interface which seamlessly drop-in-replaces the interactive **bash** session, but also always logs the stderr/stdout of every command before forwarding that command to bash


### `.gitignore` initial content

```gitignore
/logs
/logs/*
*.log
*.pid
*.stdout
*.stderr
```

