# ~/.shell.d/entrypoint.sh

```bash
# <TODO: write report of new shell session at end of entrypoint.sh>
# path: workbench/$(today)/.shell-sessions/session.pid.$$.tty.${tty_name}.$(nowdz).json
#
# containing:
# - started_at
# - finished_at
# - tty_name, tty_path
# - bash pid + `lsof -p` output of bash pid
# - bash ppid + `lsof -p` output of bash ppid
# - bash metadata:
#   - BASH_ARGV
#   - BASH_ARGC
#   - BASH_SUBSHELL
#   - et cetera
# - bash variables (declare -p)
# - bash function names (declare -p -F)
# - bash functions (declare -p -f)
# - bash history length
# <SUB-TODO>
#    report end-of-session via ~/.bash_logout
# </SUB-TODO>
# </TODO: write report of new shell session at end of entrypoint.sh>
#
# unset IFS
# exec 2>/dev/stderr
# Disable tracing and clean up
# unset BASH_XTRACEFD
# set +x
# exec 5>&- # Close the file descriptor
# declare -g my_tty_name=$(basename $(tty))
# echo "${$}" > "${HOME}/.shell.d/entrypoint.${my_tty_name}.${WEZTERM_PANE}.finished"
```
