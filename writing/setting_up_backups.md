## Setting up backups

*Currently incomplete, 2022-11-05*

<div class="code-block">
<pre style="width: 120%">

```sh
  # Use the command below to find which device your storage disk is
  $ df -h  
  # You'll need to enter the password of the storage disk here
  $ sudo cryptsetup luksOpen /dev/$storage_disk $device_name
  $ mkdir ~/$device_name
  $ sudo mount /dev/mapper/$device_name ~/$device_name
```
</pre>
</div>

And then use the following the instantiate and run the backup

<div class="code-block">
<pre style="width: 120%">

[Official docs here](https://restic.readthedocs.io/en/latest/030_preparing_a_new_repo.html)
```sh
# Must specify full paths--no tildes allowed!
# I'm using 192.168.1.2 as a placeholder for the ARM SBC I'm using as a home server/NAS
# but that could feasibly use whatever drive you want
$ restic -r sftp:username@192.168.1.2:/home/username/hdd/restic_backup init
$ restic -r sftp:username@192.168.1.2:/home/username/hdd/restic_backup --verbose backup /home/username/ --exclude="*/target/*" --exclude="*/build/*" --exclude="*.iso" --exclude="*.deb;"
```
</pre>
</div>