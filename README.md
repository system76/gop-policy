# System76 Platform GOP Policy

A driver that installs the Intel Platform GOP Policy protocol. Requires a VBT
file included in the UEFI FFS with a specific GUID.

```
FILE FREEFORM = 56752da9-de6b-4895-8819-1945b6b76c22 {
  SECTION RAW = vbt.rom
  SECTION UI = "IntelGopVbt"
}
```

```
make
```

The `qemu` target will use the target directory as a pseudo-drive so the
driver files can be viewed.

```
make qemu
```

In QEMU, the driver can be loaded using the UEFI Shell `load` command.

```
Shell> fs0:
FS0:\> load release\system76_gop_policy.efi
```
