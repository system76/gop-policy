# System76 Platform GOP Policy

A VBT file is required for building.

```
FIRMWARE_OPEN_VBT=../lemp9.vbt make
```

The `qemu` target will use the target directory as a pseudo-drive so the
driver files can be viewed.

```
FIRMWARE_OPEN_VBT=../lemp9.vbt make qemu
```

In QEMU, the driver can be loaded using the UEFI Shell `load` command.

```
Shell> fs0:
FS0:\> load release\system76_gop_policy.efi
```
