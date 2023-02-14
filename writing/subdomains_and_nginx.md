## Subdomains, Certs, and Nginx
### 13 Feb. 2023

I've been thinking about adding subdomains to my site for a while, but I've also been a little intimidated by the whole thing. There's a bunch of language around this kind of system-administration-type stuff that can be a little confusing, and even though I don't have too many visitors (shout-out to the two or three people a week who stop by to check out my little guide on switching over to a combined [Zellij and Alacritty set-up](/writing/switching_to_alacritty) up and running), I try not to break things too much. To that end, I recently noticed while working on a new computer that upon visiting my site, the `www.` subdomain led to nowhere. Now, maybe someday I'll want more subdomains, but this was particularly important because it's such a common way to type in a site name, so it took top spot priority-wise. 

First, it turns out that adding a subdomain is actually pretty easy using Digital Ocean. Once you've selected the project that you're working with, you should see two tabs in the main part of the page, labelled "Droplet" and "Domains". Click on "Domains", which should then take you take a page that says "Create a new record." In this case, per some of the documentation that I'd see, what we apparently want is a new "A" record. This should actually pretty much be the default--there are a lot of options, including tabs labeled "A", "AAA", "CNAME", "MX", etc. The "MX" domain is what you would use if you were going to set up an email address for your domain, for instance. However, "A" should be the default option. 

Now, there should a textbox available directly underneath that (and above the existing DNS records), that says something like "Enter @ or hostname", which when you type in, will shadow the intended hostname beneath it. In this case, I literally just typed in the "www." into it, and hit the "Create Record" button. That's it! 

Afterwards, I tried accessing that subdomain, but came across a warning from my browser that the TLS certificate that was issued for the site had been issued for `cmoran.xyz` but *not* `www.cmoran.xyz`. As a result, the next thing I needed to do was add this. After searching around the docs for a bit, I came across the right command. 

```sh
  $ certbot install --cert-name cmoran.xyz
  $ certbot -d cmoran.xyz,www.cmoran.xyz
```

However, upon first doing this, I was receiving an error that said 
```
  Could not automatically find a matching server block for www.cmoran.xyz. 
  Set the `server_name` directive to use the Nginx installer
```

It turns out, when I started the `www.cmoran.xyz` was simply not present, and since Certbot was checking the `nginx` config to make sure that I was trying to fetch a valid certificate, I needed to add that sub-domain first. After making the change, my Nginx configuration file (as of 5 Sept 2022) looks something like this (again, I hope that this isn't going to lead to me getting pwned, but hey). 

<div class="code-block">
<pre style="width: 130%;">

```
  server {
        server_name cmoran.xyz https://cmoran.xyz www.cmoran.xyz https://www.cmoran.xyz;

            location / {
                    proxy_pass         http://localhost:8080;
            }

        listen 443 ssl; # managed by Certbot
        ssl_certificate /etc/letsencrypt/live/cmoran.xyz/fullchain.pem; # managed by Certbot
        ssl_certificate_key /etc/letsencrypt/live/cmoran.xyz/privkey.pem; # managed by Certbot
        include /etc/letsencrypt/options-ssl-nginx.conf; # managed by Certbot
        ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem; # managed by Certbot

  }
```
</pre>
</div>

Upon changing this and re-running the `certbot` command, I received a valid certificate, which I was able to check using the command below, and then restarted the `nginx` service to make sure that everything stuck. 

```sh
  # This should show all the domains Certbot has secured valid certificates for
  $ certbot certificates
  $ sudo systemctl restart nginx.service
```

And now I('m pretty sure I) have a `www.` subdomain, and hopefully a better understanding of how to move forward if I'm interested in creating more in the future 🎉 


--- 

### Update 

To add a completely separate subdomain, for example a tileserver, add the following block underneath the initial `server` block

<div class="code-block">
<pre style="width: 130%;">

``` 
  server {
      server_name tiles.cmoran.xyz https://tiles.cmoran.xyz;

          location / {
                  proxy_pass http://localhost:3001;
          }

  }

```
</pre>
</div>

