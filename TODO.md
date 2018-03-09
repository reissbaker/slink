* [x] `sync up`
* [x] `sync down`
* [ ] .slink config directory, with an ignore file inside
* [ ] also allow a `target` file inside the directory-specific .slink config
  dir, to mirror a directory under a different path on the remote (or just to
  force a stable path in general)
* [x] `upload`
* [x] `download`
* [x] `forward ...`
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
