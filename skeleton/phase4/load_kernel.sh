#!/bin/bash

DEVICE_NAME=/dev/disk2
MOUNT_POINT=/Volumes/NO\ NAME
DESTINATION="$MOUNT_POINT/kernel8.img"
IMG_SRC=build/blinky.bin

diskutil mountDisk "$DEVICE_NAME"
rm "$DESTINATION"
mv "$IMG_SRC" "$DESTINATION"
diskutil unmountDisk "$DEVICE_NAME"

echo "Kernel loaded!"