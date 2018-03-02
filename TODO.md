* [x] `sync up`
* [x] `sync down`
* [ ] .slink config directory, with an ignore file inside
* [x] `upload`
* [ ] `download`
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
