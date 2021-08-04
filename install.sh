#!/bin/bash

# Build script for transferring the compiled kernel.img, along with necessary
# with raspberry pi firmware, to the appropriate SD card.
# OSX

KERNEL_FILE="kernel.img"  # the image to write

BOOT_FILES="./boot"       # directory of other firmware to include
                          # (if none is needed, specify an empty directory)

SDCARD_NAME="MAGPIESD"    # name of the SD card


# Check for compiled kernel.img
if ! test -f "$KERNEL_FILE"; then
    echo "\nNo kernel.img found; did compilation succeed?"
    exit 1
fi

# Check for mounted SD card and get its name
SDCARD_INFO=`diskutil list external physical | grep $SDCARD_NAME`
if [[ $? == '1' ]]; then
    echo "No device named $SDCARD_NAME"
    exit 1
fi

SDCARD=/dev/`echo $SDCARD_INFO | sed -E "s/.* ([^ ]*)s[0-9]$/\1/g"`
echo "Writing kernel & boot files to" $SDCARD_NAME "at" $SDCARD

echo "Erasing and reformatting to FAT32"
sudo diskutil eraseDisk FAT32 $SDCARD_NAME MBRFormat $SDCARD

echo "Writing firmware and kernel to $SDCARD_NAME"
cp $KERNEL_FILE $BOOT_FILES/$KERNEL_FILE
cp $BOOT_FILES/* /Volumes/MAGPIESD/

diskutil unmountDisk $SDCARD
echo "Yay, you can remove $SDCARD_NAME and test it now!"

