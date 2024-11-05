# Jumptag

## Install
- Please read before run:
  ```sh
  bash build.sh -r
  ```

## Use
### Initialize
```sh
# to initialize
jt-inner -init <your_rc_file>
# please restart your shell right after
```
### Use
An example in an interactive shell (zsh) :
- Currently we have bindings:
  ```sh
  ~/dev/rs > jt -ls
  [jump-tag] (3 bindings)
  desk => /mnt/a/Desktop
  dev => /home/da1sypetals/dev
  rs => /home/da1sypetals/dev/rs
  ```
- Create a new one and jump to it!
  ```sh
  ~/dev/rs > jt desk

  /mnt/a/Desktop > jt -add docs ~/dev/docs

  /mnt/a/Desktop > jt docs
  ```
- Now delete an existing one!
  ```sh
  ~/dev/docs > jt dev

  ~/dev > jt -del dev

  ~/dev > jt rs

  ```
- And jump to it...
  ```sh
  ~/dev/rs > jt dev
  [jump-tag] failed: tag not found: dev
  ```

- Finally, see what we have now!
  ```sh
  ~/dev/rs > jt -ls
  [jump-tag] (3 bindings)
  desk => /mnt/a/Desktop
  docs => /home/da1sypetals/dev/docs
  rs => /home/da1sypetals/dev/rs
  ```
