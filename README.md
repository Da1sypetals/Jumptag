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
jt-inner init <your_rc_file>
# please restart your shell right after
```
### Use
An example in an interactive shell (zsh) :
```
~/dev/rs > jt ls
[jump-tag] (3 bindings)
dev => /home/da1sypetals/dev
docs => /home/da1sypetals/dev/docs
rs => /home/da1sypetals/dev/rs

~/dev/rs > jt dev

~/dev > jt rs

~/dev/rs > jt docs

~/dev/docs > jt add desktop /mnt/a/Desktop

~/dev/docs > jt desktop

/mnt/a/Desktop > jt del desktop

/mnt/a/Desktop > jt dev

~/dev > jt desktop
[jump-tag] failed: jump-tag: tag not found: desktop
```
