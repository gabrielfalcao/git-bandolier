# TODO

- [ ] git-autocommit and git-commit-dated by default does not commit
      files whose diff only includes deletion of 2 or more empty lines

- [ ] command `git fixtimes` essentially runs `path fix times` on
      every file in order to set the mtime to ctime

- [ ] command `git st` produce the same output as git-status except
      that files as listed by created date and modified date in
      descending order

- [ ] command `git tryout` performs a shallow clone of repo at `${HOME}/projects/<git_svc_owner>/<git_svc_repo_name>`

- [ ] command `git ls` which is exactly like `ls` but only lists files tracked by git
