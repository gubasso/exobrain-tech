# Mount namespaces and mount propagation

A mount namespace is the mount table that a group of processes sees. Mount propagation decides whether a new mount in one namespace also appears in the others. This explains why a mount made inside a sandbox such as bubblewrap does not reach the host.

## Each process sees a mount table

The kernel keeps a table of mounts: "this filesystem is attached at this path". On a normal system, every process sees the same table.

A mount namespace gives a group of processes its own copy of that table. bubblewrap, podman, and systemd services use them. When a tool creates a namespace, the kernel copies the current table into it. From then on, there are two tables.

## The problem that propagation solves

After the copy, one side can mount something new. If the two tables are fully independent, a USB stick mounted on the host never appears in the container, and a mount made in the container never appears on the host.

Sometimes that is the goal. Sometimes the host's new mounts must show up inside, or the inside mounts must show up on the host. Mount propagation is the kernel feature that decides this, one mount at a time. Each mount has a propagation type.

## The propagation types

Think of a mount as a person who hears about new mounts, and of its copies in other namespaces as other people.

| Type       | Tag in `mountinfo` | Analogy                             | Behavior                                                           |
| ---------- | ------------------ | ----------------------------------- | ------------------------------------------------------------------ |
| shared     | `shared:N`         | a group chat                        | A mount on any member appears on every member, in both directions. |
| slave      | `master:N`         | a subscriber to a broadcast channel | It receives new mounts from group N and sends nothing back.        |
| private    | no tag             | a person with no phone              | It receives nothing and sends nothing.                             |
| unbindable | `unbindable`       | private, and a bind cannot copy it  | Rare in practice.                                                  |

The number N is the ID of a peer group. A peer group is a set of mounts that share mount events with each other. `shared:1` means "a member of group 1". `master:1` means "a slave that listens to group 1". The word "master" names the group that the slave listens to.

A mount can be a slave and shared at the same time. Then it shows both tags.

## Rules that decide the type

- systemd makes every mount shared at boot. So on a systemd host, `/` is `shared:N`.
- When the kernel copies a table for a new namespace, each shared mount in the copy joins the same peer group as the original. Without further action, the new namespace is a full member of the group chat.
- A bind of a mount copies its propagation. A bind of a slave mount is a slave too.
- A new mount joins a peer group only if the mount below it is shared. Under a slave or a private mount, a new mount starts out private.
- A mount that a tool creates fresh, such as an overlay root, belongs to no group. It is private.

## Read the type from mountinfo

`/proc/self/mountinfo` has one line per mount:

```text
ID PARENT MAJ:MIN ROOT MOUNTPOINT OPTIONS [TAGS] - FSTYPE SOURCE SUPEROPTS
```

The tags sit between the mount options and the `-`. Field 5 is the mount point. On a btrfs root, field 4 shows the subvolume path, not `/`, so select lines by field 5:

```sh
awk '$5 == "/" || $5 ~ "^/mnt"' /proc/self/mountinfo
findmnt -o TARGET,PROPAGATION /
```

## Example: a mount inside bubblewrap

The host root of a systemd host:

```text
47 1 0:41 /@/.snapshots/1/snapshot / rw,relatime shared:1 - btrfs /dev/mapper/system-root rw,...
```

A sandbox that binds `/mnt/bindtest` and mounts a tmpfs on it:

```sh
sudo bwrap --dev-bind / / --bind /mnt/bindtest /mnt/bindtest \
  sh -c 'mount -t tmpfs bindtest /mnt/bindtest && grep bindtest /proc/self/mountinfo'
findmnt /mnt/bindtest || echo "  not on host"
```

```text
1456 1415 0:41 /@/.snapshots/1/snapshot/mnt/bindtest /mnt/bindtest rw,nosuid,nodev,relatime master:1 - btrfs ...
1291 1456 0:97 / /mnt/bindtest rw,relatime - tmpfs bindtest rw,inode64
  not on host
```

Line by line:

1. The host `/` is `shared:1`, a member of peer group 1.
2. The bind in the sandbox is `master:1`. It listens to the host root but is not a member. A host mount under `/mnt` reaches the sandbox. A sandbox mount does not reach the host.
3. The tmpfs has no tag. The mount below it is a slave, so the tmpfs starts out private and tells nobody.

When the last process of the sandbox exits, the kernel deletes the namespace and every mount in it. The tmpfs never existed on the host.

```text
HOST namespace                    SANDBOX namespace (bwrap)

/  shared:1  ───── events ─────►  /mnt/bindtest  master:1
                  one way only         │
                                       └── tmpfs  (private)
           ◄──── nothing ──────
```

## Why bubblewrap chooses slave

A copied namespace starts as a full member of the host peer groups. For a sandbox, that is a security hole: a sandboxed program could mount a filesystem over `/etc` on the host. So bubblewrap turns the whole copy into slaves before it makes any bind (`bubblewrap.c`, bubblewrap 0.12.0):

```c
/* Mark everything as slave, so that we still
 * receive mounts from the real root, but don't
 * propagate mounts to the real root. */
if (mount (NULL, "/", NULL, MS_SILENT | MS_SLAVE | MS_REC, NULL) < 0)
```

`MS_SLAVE` sets the type. `MS_REC` applies it to every mount below `/` as well. bubblewrap has no option that sets a mount back to shared.

The consequence: a `--bind` carries files between the sandbox and the host, but not mounts. A bind changes which directory the sandbox sees, not which way mount events travel. A program that must add a mount to the host table, such as a mount helper, cannot do so from inside bubblewrap.

## Try it with unshare

`unshare -m` creates a mount namespace without bubblewrap. `--propagation` chooses the type, and the default is `private`.

```sh
sudo mkdir -p /mnt/bindtest

# Shared: the namespace stays in the host peer group. The mount reaches the host.
sudo unshare -m --propagation unchanged sh -c 'mount -t tmpfs demo /mnt/bindtest'
findmnt /mnt/bindtest
sudo umount /mnt/bindtest

# Slave: the same as bubblewrap. The mount stays inside.
sudo unshare -m --propagation slave sh -c 'mount -t tmpfs demo /mnt/bindtest'
findmnt /mnt/bindtest || echo "  not on host"
```

In the shared case, the mount stays on the host even after the namespace exits, because the host received its own copy of the mount.

## References

- `man 7 mount_namespaces`: the kernel rules for namespaces and propagation.
- `man 1 unshare`: the `--propagation` option.
- Linux kernel documentation, `Documentation/filesystems/sharedsubtree.rst`: the full rule set for shared subtrees.
