# TCG Boot

This is EFI application to secure loading Linux with TPM.

## How it works?

1. The system firmware will measure TCG Loader.
2. The TCG Loader will measure it's all configurations.
3. The TCG Loader will check Linux's signature and all related files.
4. If signature passed TCG Loader will execute Linux.

Then Linux need to unseal the secret keys and do something with it after this to
establish trusted boot. If the unseal operation is failed that mean the boot
processes cannot be trusted anymore.

With this way the Linux and it related files are freely to update without
breaking TPM's measurement.

## Dependencies

- GNU-EFI

## Additional tools required for development

- GNU Parted
- Mtools
- OVMF
- QEMU

It is required for development only, not building.
