# 打开相机
guvcview -d /dev/video0

# 登陆uboot
ext4load mmc 0:2 0x85000000 /uEnv.txt;env import -t 0x85000000 $filesize;setenv boot_rollback 1;run bootcmd_cmd; reset;

# 推送镜像
adb push arceos-fada.itb /userdata/thw

# 替换内核
mount /dev/mmcblk0p1 /mnt
cp /userdata/thw/arceos-fada.itb /mnt/Image.itb
reboot
