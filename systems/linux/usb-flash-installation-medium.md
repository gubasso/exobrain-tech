# USB flash installation medium

> https://wiki.archlinux.org/title/USB_flash_installation_medium
> https://docs.voidlinux.org/installation/live-images/prep.html

## Practical rule of thumb

- **Updating an already-working Arch ISO USB** (made via `dd`/`cp`/`cat` to the whole device): ✅
  Just re-write the new ISO to the whole device. No `wipefs`, no `parted`, no `mkfs`.

- **Converting the stick back to normal storage**: ✅ `wipefs` + partition + format.

### Minimal “update” procedure (Write / `dd`)

1. Identify the device (verify carefully):

   ```sh
   lsblk -o NAME,SIZE,MODEL,SERIAL,TRAN
   ls -l /dev/disk/by-id/usb-*
   ```

2. Unmount anything auto-mounted:

   ```sh
   sudo umount /dev/disk/by-id/usb-My_flash_drive* 2>/dev/null || true
   ```

3. Write the new ISO to the whole device (no `-partN`):

   ```sh
   sudo dd bs=4M if=/path/to/archlinux-x86_64.iso of=/dev/disk/by-id/usb-My_flash_drive \
     conv=fsync oflag=direct status=progress
   sudo sync
   ```

4. Replug the USB (recommended) so the kernel re-reads the new layout.

### When you would use `wipefs`/partitioning again

Only if you are **restoring the stick for normal storage use** (e.g., single FAT32 partition), or if
the device has ended up with confusing remnants from other tooling and you want a clean “storage
drive” layout. For simply updating the Arch installer, skip it.

### Small verification checks (optional)

After writing:

```sh
lsblk -f
sudo fdisk -l /dev/disk/by-id/usb-My_flash_drive
```

You should see the ISO’s hybrid layout (often multiple partitions / ISO9660-related entries). That
is expected.

If you want, paste the output of `ls -l /dev/disk/by-id/usb-*` and `lsblk -f` (with the USB plugged
in) and I’ll point to the exact `by-id` path you should use to avoid hitting the wrong disk.

## Prerequisites

- A USB drive whose contents can be destroyed.
- Root access on the host.
- The drive's stable path from `ls -l /dev/disk/by-id/usb-*`, so no other disk can be hit.

## Steps

1. Identify the USB drive.

   ```sh
   lsblk
   # or
   sudo fdisk -l
   ```

2. Remove the ISO 9660 signature a previous image write left behind, so the drive can serve as
   ordinary storage again.

   ```sh
   sudo wipefs --all /dev/disk/by-id/usb-_My_flash_drive_
   ```

   - Expected: a success message naming each signature removed.
   - Run as root, before [repartitioning](https://wiki.archlinux.org/title/Repartition "Repartition")
     and [reformatting](https://wiki.archlinux.org/title/Reformat "Reformat") the USB drive.

3. Create a partition table and a partition covering the whole drive.

   ```sh
   sudo parted -s /dev/sdX mklabel msdos mkpart primary fat32 0% 100%
   ```

   - `-s` runs in script mode, which suppresses interactive prompts.
   - `mklabel msdos` creates an MBR (DOS) partition table; use `gpt` for a GPT one.
   - `mkpart primary fat32 0% 100%` creates a primary partition from 0% to 100% of the disk and
     labels it FAT32; `ext4` or `ntfs` where another filesystem is wanted.
   - The partition is created but not formatted — step 4 is what makes it usable.

4. Format the partition.

   ```sh
   sudo mkfs.fat -F 32 /dev/disk/by-id/usb-My_flash_drive-partn
   ```

   - Older form, by device node: `sudo mkfs.vfat /dev/sdX1`, with `/dev/sdX1` replaced by the
     partition created above.

5. Confirm the drive is ready.

   ```sh
   lsblk -f
   ls -l /dev/disk/by-id/usb-*
   ```

   - Expected: the partition reports its filesystem, and the drive is listed under `by-id`.

## Write

(Do **not** append a partition number, so do **not** use something like
`/dev/disk/by-id/usb-Kingston_DataTraveler_2.0_408D5C1654FDB471E98BED5C-0:0**-part1**` or
`/dev/sdb**1**`):

- using [cat(1)](https://man.archlinux.org/man/cat.1):

```bash
sudo cat path/to/archlinux-version-x86_64.iso > /dev/disk/by-id/usb-My_flash_drive
```

- using [cp(1)](https://man.archlinux.org/man/cp.1):

```bash
sudo cp path/to/archlinux-version-x86_64.iso /dev/disk/by-id/usb-My_flash_drive
```

- using [dd](https://wiki.archlinux.org/title/Dd "Dd"):

```bash
sudo dd bs=4M if=path/to/archlinux-version-x86_64.iso of=/dev/disk/by-id/usb-My_flash_drive conv=fsync oflag=direct status=progress
```

- using [tee](https://wiki.archlinux.org/title/Tee "Tee"):

```bash
sudo tee < path/to/archlinux-version-x86_64.iso > /dev/disk/by-id/usb-My_flash_drive
```

- using [pv](https://archlinux.org/packages/?name=pv):

```bash
sudo pv path/to/archlinux-version-x86_64.iso -Yo /dev/disk/by-id/usb-My_flash_drive
```

Executing:

```sh
sudo sync
```

...with root privileges after the respective command ensures buffers are fully written to the device
before you remove it.

---

## When you _do_ need wipefs/partitioning again

You only need the `wipefs` + `parted` + `mkfs` sequence when your goal is **not** “make it boot
Arch”, but **restore the USB to normal storage use** (single FAT32/exFAT/ext4 partition, etc.), or
if you intentionally want a custom partition scheme.

Also, do those steps if you previously did something like:

- **reformatted** it as a normal storage drive (FAT32/exFAT/ext4) and now want it bootable again, or
- built a **custom multiboot / persistence** layout that you want to recreate cleanly.

In those cases:

- `wipefs --all` is useful to remove confusing leftover signatures,
- then repartition + format as desired.
