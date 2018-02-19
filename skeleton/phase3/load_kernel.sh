#!/bin/bash

DEVICE_NAME=/dev/disk2
MOUNT_POINT=/Volumes/NO\ NAME
DESTINATION="$MOUNT_POINT/kernel8.img"

diskutil mountDisk "$DEVICE_NAME"
rm "$DESTINATION"
mv blinky.bin "$DESTINATION"
diskutil unmountDisk "$DEVICE_NAME"

echo "Kernel loaded!"