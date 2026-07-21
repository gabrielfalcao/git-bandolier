
# TODO

- [ ] command `git fixtimes` essentially runs `path fix times` on
      every file in order to set the mtime to ctime

- [ ] `git-autocommit` and `git-commit-dated` by default does not commit
      files whose diff only includes deletion of 2 or more empty lines

- [ ] command `git st` produce the same output as git-status except
      that files as listed by created date and modified date in
      descending order

- [ ] command `git tryout` performs a shallow clone of repo at `${HOME}/projects/<git_svc_owner>/<git_svc_repo_name>`

- [ ] command `git ls` which is exactly like `ls` but only lists files tracked by git

- [ ] command `git commit-dated --added --modified --no-deleted`

- [ ] command `git tryout` to clone third-party repos in to a
      well-known path of third-party repos, meant for inspecting
      source code without planning to run a fork: `git tryout --deep
      https://github.com/denoland/deno.git
      https://github.com/oven-sh/bun.git`
