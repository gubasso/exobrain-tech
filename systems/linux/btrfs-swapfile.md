# Swap file on btrfs

> swapfile

- [Arch Wiki: Btrfs: Swap file](https://wiki.archlinux.org/title/Btrfs#Swap_file)

```text
# btrfs subvolume create /swap
# btrfs filesystem mkswapfile --size 4g --uuid clear /swap/swapfile
# swapon /swap/swapfile
```

Edit `/etc/fstab`, add line:

```text
/swap/swapfile none swap defaults 0 0
```
