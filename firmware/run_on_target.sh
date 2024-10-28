#!/bin/sh
file_path=$1
file_name="${file_path##*/}"
sshpass -pmilkv ssh root@192.168.42.1 'echo stop > /sys/class/remoteproc/remoteproc0/state'
sshpass -pmilkv scp -O $1 root@192.168.42.1:/lib/firmware
sshpass -pmilkv ssh root@192.168.42.1 'echo $file_name > /sys/class/remoteproc/remoteproc0/firmware'
sshpass -pmilkv ssh root@192.168.42.1 'echo start > /sys/class/remoteproc/remoteproc0/state'
