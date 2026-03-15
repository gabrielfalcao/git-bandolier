# TODO workbench


## **epiphany** 2026-03-14


every new shell session (e.g. terminal tab) initiates a new "context"
which SHALL be described textually, that way every time you pivot from
one main task to a new idea you "justify" with a short rationale why
you are shifting context. By default, when creating a new terminal tab
from an already contextualized shell session, creates a subsession (or
"child") which "inherits" the context from its parent, this way,
workbench can keep track of your tasks and keep record of what idea
originated another, what tasks generated dependency subtasks and more
generally what context prompted another context.

Workbench data engine stores each and every new terminal session as a
DAG node, and is general enough to not limit itself to "terminal
sessions" but instead model one overarching, "umbrella" concept of
"session" comprised - at minimum - of a *textual context*, one *parent
node* and one *main category* (e.g. a "terminal session" is a session
whose main category is "terminal").

This way, every session node directly connects to other sessions of
other categories.

The first goal is to model only terminal sessions, which the challenge
of creating a harness for BASH which takes full advantage of features
such as `PROMPT_COMMAND`, generating per-session shell functions which
collect data throughout every shell session, which use shell functions
as a trojan horse of sorts which are capable of collecting all kinds
of shell variables such as via `declare -p` and `declare -p -f`, then
hooking those functions to BASH debugging features and facilities such
as `trap RETURN` `trap DEBUG` and `trap ERR`. All that data is
*collected* and reported in real-time to workbench either by forking a
subprocess of the "Workbench CLI" binary that stores (.i.e. "writes")
to the workbench graph directly or by pushing data to a local
"workbench-daemon" which is a subcommand of the single-binary
"workbench" and thus shares all the core capabilities.

The "workbench daemon" is a single local server which runs in the
background and provides several options in terms of receiving streams
of collected data, accepting requests and authenticating clients:

- Unix Pipes which rely on well known Unix-style filesystem permissions
- Unix Sockets supporting simple, SSH-style descentralized PKI such as curve ed25519
- HTTP/2 supporting simple server-side TLS as well as mutual TLS
- ZeroMQ with application-level encryption and authentication

Yet in terms of the daemon networking, the PoC must not support
encryption at the first stages, so as to give room to fully developing
the PoC of the *Workbench Data Engine* so as to comprise the general
concept of session by supporting 2 types of session categories: "Terminal Session" and "Browser Session".

The primary session type: "Terminal Session", is the flagship feature
of the workbench project.

The secondary session type: "Browser Session" shares common concepts,
semantically similar concepts and often instinctively analogous
concepts to terminal sessions such as Windows and Tabs. Both Terminal
and Browser have Windows and Tabs. Whether you are performing the
current task at hand in a browser or in a terminal, pivoting to a
subtask or to a new idea often means opening a new window or a new
tab, therefore it is imperative that once the "Workbench Data Engine"
can keep track of your context switching between tabs and windows
within your favorite Terminal app, then the next step of supporting
keep track of context-switching in your favorite browser is simply a
matter of writing a browser extension to collect and report the data
to the workbench server. Finally, at that point the last milestone to
conclude the PoC of the Workbench Project is to keep track of
context-switching between apps, which as it appears, should be
relatively easy to accomplish given the common concepts shared between
those two types of apps.

### Conclusion of the PoC: Product Requirements

1. Provide an overview of how much time you spent on every context,
   from every indispensable, dependent task, down toto every small
   subtask prompted by unrelated ideas

2. Provide an overview of how much time you spend down rabbit holes
   which started as a "small" task to which you pivoted while working
   on a "main" task or concept that you actually deemed more important
   than the small one which led you down the rabbit hole.

3. Provide, seamlessly, a powerful tool to help you fine-tune your
   priorities, organize your mind and go through the hell of unwanted
   distractions in your day.



## TODO 2026-03-12

### subcommand `git`


> runs git clone --depth=1 inside
> `${workbench_path}/git-tryouts/<kind>/<vendor>/<owner>/<repo_name>`
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
repo_id: parcel
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
repo_name: ANSI.md
repo_id: 458719343aabd01cfb17a3a4f7296797
```

and runs a command equivalent to, more or less:

```bash
mkdir -p ~/workbench/$(today)/git-tryouts/gist/github/fnky
git clone --depth=1 https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797.git ~/workbench/$(today)/git-tryouts/gist/github/fnky/ANSI.md/
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
