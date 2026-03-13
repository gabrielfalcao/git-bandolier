# TODO workbench

## TODO 2026-03-12

### subcommand `git`


> runs git clone --depth=1 inside `${workbench_path}/git-tryouts/<kind>/<vendor>/<owner>/<repo_name>`
> where:
>
>  - `kind` is one of `repo`, `gist`, `wiki` etc
>  - `vendor` is one of `github`, `gitea`, `gitlab`, `source_hut` etc
>  - `owner`  is the vendor's `username`
>  - `repo_id`  is the git repo "name" as found in the git clone uri, e.g.: when `kind` is `gist` `repo_id` is the gist id, whereas for when `kind` is `repo` `repo_id` is the git repo name
>  - `repo_name`  is the vendor's name or title for specific `kind`, such as the name/title of a gist or repo name when kind is `kind`
>  - `vendor` is one of `github`, `gitea`, `gitlab`, `source_hut` etc

#### Examples

##### kind: repo

```bash
workbench git tryout https://github.com/scrapy/parsel
```

renders the metadata:

```yaml
kind:   repo
vendor: github
owner: scrapy
repo_name: parcel
```

and runs a command equivalent to, more or less:

```bash
mkdir -p ~/workbench/$(today)/git-tryouts/repo/github/scrapy
git clone --depth=1 https://github.com/scrapy/parsel.git ~/workbench/$(today)/git-tryouts/repo/github/scrapy/parcel.git
```

#### kind: gist


```bash
workbench git tryout https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
```

renders the metadata:

```yaml
kind: gist
vendor: github
owner: fnky
repo_name: parcel
```

and runs a command equivalent to, more or less:

```bash
mkdir -p ~/workbench/$(today)/git-tryouts/repo/github/scrapy
git clone --depth=1 https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797.git ~/workbench/$(today)/git-tryouts/repo/github/scrapy/parcel.git
```


```bash
workbench git tryout https://gist.github.com/458719343aabd01cfb17a3a4f7296797.git

# simply runs:

git clone --depth=1 https://gist.github.com/458719343aabd01cfb17a3a4f7296797.git ~/workbench/$(today)/git-tryouts/github/gist/fnky__ANSI.md
```


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

### add item to finder sidebar

#### via osascript


To add a folder to the macOS Finder sidebar via command-line, use  to instruct Finder to add a specific path. The most reliable method is executing this in terminal: [1, 2, 3, 4]
Key Command-Line Methods:

• AppleScript (Recommended): .
• Keyboard Shortcut (Terminal simulation): You can use  to trigger the  shortcut. [1, 3, 4]

Important Notes:

• Ensure the path is an absolute path (e.g.).
• These methods add the item to the "Favorites" section.
• You may need to restart Finder for changes to take effect immediately: . [1, 4, 5, 6, 7]

AI responses may include mistakes.

[1] https://apple.stackexchange.com/questions/392088/add-finder-sidebar-section-via-terminal
[2] https://support.apple.com/en-az/guide/mac-help/mchl83c9e8b8/mac
[3] https://osxdaily.com/2011/05/05/add-your-own-folders-to-the-finder-sidebar-in-mac-os-x/
[4] https://stackoverflow.com/questions/4852516/add-item-to-finder-sidebar
[5] https://support.apple.com/guide/mac-help/customize-the-finder-sidebar-on-mac-mchl83c9e8b8/mac
[6] https://support.apple.com/en-mt/guide/mac-help/mchl83c9e8b8/mac
[7] https://www.youtube.com/watch?v=HIFAU5X5uh0



#### via sfltool

sfltool add-item com.apple.LSSharedFileList.FavoriteItems file:///path/to/folder

You can add items to the macOS Finder sidebar via command-line using the  (Shared File List tool) to manage . [1]
Use the following command in Terminal, replacing  with your target:
Note: You may need to relaunch Finder for changes to appear (). [1, 2, 3]
Key Details:

• Target:  controls the "Favorites" section.
• Utility:  works on modern macOS versions.
• Requirement: The path must be a valid file URL (e.g.).
• Alternative: You cannot directly use  for sidebar modifications. [1, 4, 5]

AI responses may include mistakes.

[1] https://community.jamf.com/t5/jamf-pro/script-to-edit-finder-favorites-to-include-ad-network-folder/m-p/68916
[2] https://www.lifewire.com/add-files-folders-and-apps-with-finder-sidebar-2260877
[3] https://www.avast.com/c-mac-show-hidden-files
[4] https://apple.stackexchange.com/questions/392088/add-finder-sidebar-section-via-terminal
[5] https://www.reddit.com/r/macsysadmin/comments/5tun12/setting_finder_preferences_sidebar_and_dock_items/
