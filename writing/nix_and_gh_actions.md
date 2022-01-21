# Running `act` through Nix
https://nixos.wiki/wiki/Nix_Installation_Guide

```sh
$ sudo install -d -m755 -o $(id -u) -g $(id -g) /nix
$ curl -L https://nixos.org/nix/install | sh
```

Uses the `install` command, which per it's man page: `This  install  program  copies  files  (often just compiled) into destination locations you choose.`

I ran into this problem the next time I restarted by machine, where upon logging into my desktop environment, I received a message about an error while parsing the `.profile` file. Upon inspection, the Nix installation script added a pair of lines to `.profile` that looked like

```sh
-e 
if [ -e /home/chrism/.nix-profile/etc/profile.d/nix.sh ]; then . /home/chrism/.nix-profile/etc/profile.d/nix.sh; fi # added by Nix installer
```

However, the first of those, which simply had `-e` was not recognized, and threw an error. By entering into the system in headless mode, I was able to remove that line, which allowed the rest of the system to boot fine, and Nix to become available. At that point, following the basics of the [Nix Book](https://nixos.org/manual/nix/stable/quick-start.html) with commands like `$ nix-shell -p hello` seemed to work fine. 

At this point, we were trying to run `act`. Per the README page, first we needed to install [Docker Engine](https://docs.docker.com/engine/install/ubuntu/) (since I'm on Pop!_OS, I followed the Ubuntu instructions). At this point however, I ran into a Docker socket issue when trying to run the `act` default command. After a little searching, I found the solution to this problem in a StackOverflow [post](https://stackoverflow.com/questions/51342810/how-to-fix-dial-unix-var-run-docker-sock-connect-permission-denied-when-gro), which allows the socket connection. 
```sh
# run this line to allow the Docker connection
$ sudo setfacl --modify <your_username>:rw /var/run/docker.sock
```

On an Odroid-N2+ running Armbian, `$ sudo softy`, select Docker and hit "Install". `Softy` is a built-in package manager for installing common packages for Armbian. `setfacl` doesn't work on Armbian, though, so using
```sh
sudo usermod -aG docker $USER
sudo reboot
```