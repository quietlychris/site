## transitioning this site from `screen` to `systemd`

This website has been one of my first explorations into doing any sort of web programming, and as result, has taught me a lot about different computing concepts from networking to encryption to system processes. At the start, I was running this service pretty simply over `https`. Since the content is largely static and doesn't really include any sensitive information, adding support for TLS to accommodate `https` wasn't a high priority. 

The site uses the [Rocket](https://rocket.rs) web server, which basically requires typing in `$ cargo run --release` into the `site` directory, which will use the specified compilation profile to bind to the given port. For http-only, this is usually Port 80; for https, Port 443. However, Rocket doesn't automatically run in the background, so if you want to exit your SSH session. 


I tested this first using a [Odroid-N2+](https://www.hardkernel.com/shop/odroid-n2-with-4gbyte-ram-2/)[^1] that I use as test machine in my homelab. However, as mentioned on my site's [About](/writing/about) page, I host this site using a Digital Ocean droplet, which has a different directory structure from the [Armbian](https://www.armbian.com/) user profile that I'm using on the N2+, where the standard user files are located a `/home/user`. Instead, the Droplet has files (by default) at `/root`. Keep this in mind when switching configuration paths between the test and "production" (everything's relative, right?) environments.

The file described the website's executable as a service, with a syntax as described [here](https://www.freedesktop.org/software/systemd/man/systemd.service.html), with a good introductory article to using `systemd` [here](https://www.cloudsavvyit.com/3092/how-to-add-your-own-services-to-systemd-for-easier-management/). 

<div class="code-block">
<pre style="width: 120%">

```toml
  # A file with this information goes at /etc/systemd/system/site.service
  [Unit]
  Description=Website (cmoran.xyz)
  After=network.target
  StartLimitIntervalSec=1

  [Service]
  Type=simple
  Restart=always
  RestartSec=1
  User=root
  WorkingDirectory=/root/site
  # ExecStart requires an absolute path
  ExecStart=/root/site/target/release/site

  [Install]
  WantedBy=multi-user.target
```
</pre>
</div>

From there, we can 
```bash
 # Reload the systemd service configuration files
 $ sudo systemctl daemon-reload
 $ sudo systemctl enable site.service
 $ sudo systemctl restart site.service
```
During this process, I like to keep an eye on what's actually happening in the system. The easiest way to do this is through `journalctl` using the command 
```bash
 # Move to end (-e) of the file and follow (-f)
 $ journalctl -e -f
``` 
but most of the important information will also appear if you use the command 
```bash
 # Follow (-f) the tail end of the syslog
 $ sudo tail -f /var/log/syslog 
```
---

[^1]: The N2+ a great little machine, but if you're in North America and considering a purchase, I'd suggest using [Ameridroid](https://ameridroid.com/products/odroid-n2-plus?variant=32211327320098) instead of Hardkernel. I at one point purchased an Odroid-C4 directly from Hardkernel which quickly developed some hardware issues. Getting an RMA notice from Hardkernel didn't take too long, but the shipping cost to/from the United States to their factory would have been over twice the cost of buying a new board. On the other hand, I haven't any issues with Ameridroid-purchased boards, but their customer support for things like making small order modifications has been really great, and I've been a pretty happy customer for a number of SBC-related part orders. 