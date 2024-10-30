#!/bin/sh
echo Run $1 on target
HOSTNAME=192.168.42.1

sshpass -pmilkv ssh root@${HOSTNAME} 'echo stop > /sys/class/remoteproc/remoteproc0/state'
sshpass -pmilkv scp -O $1 root@${HOSTNAME}:/lib/firmware/firmware.elf
sshpass -pmilkv ssh root@${HOSTNAME} 'echo firmware.elf > /sys/class/remoteproc/remoteproc0/firmware'
sshpass -pmilkv ssh root@${HOSTNAME} 'echo start > /sys/class/remoteproc/remoteproc0/state'
