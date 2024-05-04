#!/bin/bash

IMAGE="./target/x86_64-blog_os/debug/bootimage-blog_os.bin"
QEMU=qemu-system-x86_64

$QEMU -drive format=raw,file=$IMAGE
