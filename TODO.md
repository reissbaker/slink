* [x] `sync up`
* [ ] `--watch` flag for `sync up`. To keep the connection alive as long as
  `--watch` is running, without needing to persist the connection forever even
  once it stops running, in a separate thread have an empty shell open on the
  remote. (Or check if SSH supports a do-nothing command that doesn't open a
  shell, but keeps the connection active.)
* [x] `sync down`
* [ ] .slink config directory, with an ignore file inside
* [ ] also allow a `target` file inside the directory-specific .slink config
  dir, to mirror a directory under a different path on the remote (or just to
  force a stable path in general)
* [x] `upload`
* [x] `download`
* [x] `forward ...`
* [ ] Figure out how to safely canonicalize paths for download, where they
  don't exist on the local system but do exist on the remote
* [ ] Allow up, down, upload, and download to take an optional second argument
  to allow uploading/downloading/syncing to specific directories that don't
  match pwd on the remote machine
* [ ] `reset` should pop back up to last configuration. Implement this by
  changing the host config file to be multiples lines, and always use the last
  line; to reset, just delete the last line
* [ ] `clear` should clear all host configuration and socket files
* [x] `current` should print the current host
* [ ] Integration test slink by running an `sshd` in a Docker container
* [ ] Actually exit with correct exit codes rather than panicking
* [ ] Support `kubectl` as a transport. Upload and download can use `kubectl
  cp` instead of `scp`, go and run can use `kubectl exec` instead of `ssh`, and
  [this ServerFault
  post](https://serverfault.com/questions/741670/rsync-files-to-a-kubernetes-pod)
  explains how to use `kubectl exec` as an `rsh`-alike to get rsync to work.
  You'll need to alter how `use` works, to make it more like git remotes (since
  you need more configuration when you support multiple transports)... `slink
  remote add <shortcut> ...` and `slink remote use <shortcut>`? Allow setting a
  global default, but also allow per-directory transports and defaults in the
  `.slink/` config directory.
* [ ] Allow nesting arbitrary transports via forwarding. Syntax TBD, but maybe
  something like `--transport=forward(<transport> over(<shortcut> <port>))`.
  Or better: `slink remote add ssh <shorcut> ...`,
  `slink remote add kubectl <shortcut> ...`,
  `slink remote add forward <shortcut> --via <other-shortcut> --port <port>`
  Which allows the type system to verify that flags for SSH aren't used for
  kubectl or forward, etc, via StructOpt. You can nest infinitely by making
  additional remotes that themselves are forwarded. This allows you to e.g. use
  a local `kubectl` to transport to a remote Kubernetes cluster that is
  inaccessible to the outside world by traversing through an SSH bastion.
* [ ] Allow per-command remote defaults in the slink config files, so that you
  can sync to one remote but e.g. run or go to another. This is useful if
  you're syncing files to a remote host over SSH, and the files are
  volume-mounted into containers run by a local Kubernetes cluster on the
  remote; you'd want the files to go to the underlying host, but `go` and `run`
  to jump into the containers.
