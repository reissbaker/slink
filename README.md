## Slink: simple remote development environments over SSH

```bash
export SLINK_HOST=remote-devbox.mydomain.com

# sync the current directory to the remote machine:
slink sync up

# Run a command on the remote machine in the synced directory:
slink run "ls -la"

# SSH into the machine and change to the synced directory:
slink go

# Forward ports 8080 and 4443 on your local machine to the same ports on your
# remote machine:
slink forward 8080 4443

# Forward port 80 on your local machine to 80 on the remote machine -- slink
# will automatically prompt for sudo as necessary
slink forward 80
```

Slink is designed to make remote development environments simple and painless.
It allows you to treat a remote machine as being a mirror of your local
machine; it syncs directories, keeping your directory structure the same, opens
shells on the remote machine in the directories that mirror your PWD, etc. It
abstracts over SSH, rsync, and scp to provide a simple interface for
interacting with a remote dev environment, and multiplexes connections for all
of them over a single cached SSH connection for performance.

Slink assumes you want your remote machine to effectively mirror the directory
structure of your local machine: the expectation is you're treating your remote
like your local machine, but on [different hardware|a different OS|etc].

## Commands

* `slink use <hostname>`: set the hostname to use for commands.
* `slink go`: SSH to the machine, switching to the mirror of PWD (if it
  exists).
* `slink run <command>`: runs a command on the machine. Automatically allocates
  a PTY for you to allow interactive commands to work corrrectly.
* `slink forward <ports...>`: forward ports from your local machine to the same
  ports on the remote machine.
* `slink sync up`: sync the current directory to the remote machine via rsync,
  maintaining relative path from $HOME if in $HOME, or from root otherwise.
* `slink sync down`: inverse of `sync up`.
* `slink upload <file>`: uploads a file to the remote, in the same relative
  location from $HOME if in $HOME, or from root otherwise.
* `slink download <file>`: inverse of `upload`.

## Host configuration

To set a default host for Slink, edit your `.bashrc` (or `.zshrc`, or relevant
file for your shell) to export the `SLINK_HOST` environment variable. From then
on, all new shell sessions will use that host by default. For example:

_Bash or Zsh:_
```bash
export SLINK_HOST=remote-devbox.mydomain.com
```

In previous versions, Slink used a config file to store the current host. While
this made using a single machine as a remote very simple, it made managing
multiple machines painful, especially if you were trying to manage them
concurrently in multiple shell sessions, or were using Slink in automated
wrapper scripts. Environment variables are just about as easy to set up, but
make it simpler to manage multiple machines.
