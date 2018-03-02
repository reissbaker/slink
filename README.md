## Slink: simple remote development environments

```bash
slink use remote-devbox.mydomain.com

# sync the current directory to the remote machine:
slink sync up

# Run a command on the remote machine in the synced directory:
slink run "ls -la"

# SSH into the machine and change to the synced directory:
slink go
```

Slink is designed to make remote development environments simple and relatively
painless. It allows you to treat a remote machine as being a mirror of your
local machine; it syncs directories, keeping your directory structure the same,
opens shells on the remote machine in the directories that mirror your PWD,
etc. It abstracts over SSH, rsync, and scp to provide a simple interface for
interacting with a remote dev environment, and multiplexes connections for all
of them over a single cached SSH connection for performance.

Slink assumes you want your remote machine to effectively mirror the directory
structure of your local machine: the expectation is you're treating your remote
like your local machine, but on [different hardware|a different OS|etc].

### Commands

* `slink use <hostname>`: set the hostname to use for commands.
* `slink go`: SSH to the machine, switching to the mirror of PWD (if it
  exists).
* `slink run <command>`: runs a command on the machine. Automatically allocates
  a PTY for you to allow interactive commands to work corrrectly.
* `slink sync up`: sync the current directory to the remote machine via rsync,
  maintaining relative path from $HOME if in $HOME, or from root otherwise.
* `slink sync down`: inverse of `sync up`.
* `slink upload <file>`: uploads a file to the remote, in the same relative
  location from $HOME if in $HOME, or from root otherwise.
* `slink download <file>`: inverse of `upload`.
