#!/bin/sh
echo Run $1 on target

#ssh-keyscan -H 192.168.42.1 >> ~/.ssh/known_hosts

sshpass -pmilkv ssh root@192.168.42.1 'echo stop > /sys/class/remoteproc/remoteproc0/state'
sshpass -pmilkv scp -O $1 root@192.168.42.1:/lib/firmware/firmware.elf
sshpass -pmilkv ssh root@192.168.42.1 'echo firmware.elf > /sys/class/remoteproc/remoteproc0/firmware'
sshpass -pmilkv ssh root@192.168.42.1 'echo start > /sys/class/remoteproc/remoteproc0/state'
