# Workbench Project PoC



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
