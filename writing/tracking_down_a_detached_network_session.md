## Tracking down a detached network session

Today I spent some time working on debugging a small middleware application that I've been working on. As part of the current implementation, I typically open up a central host node on port 25000. However, when I went to start this process today, I began to receive the following error:

<div class="code-block">
<pre style="width: 110%">

```sh
  thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: I
  o(Custom { kind: Other, error: "could not acquire lock on \"store/db\": O
  s { code: 11, kind: WouldBlock, message: \"Resource temporarily unavailab
  le\" }" })', src/bin/start_host.rs:53:10
```
</pre>
</div>

This error was coming from `sled`, the key-value store program that my application is using to store data. In this case, there was a conflict between two programs trying to access the same database, which `sled` doesn't allow. But where was the other session? I checked all of my open terminals and tabs; no extra session. This means that it must have accidentally got detached from my terminal multiplexer along the way. Now, I've done this intentionally in the past using `screen`, but haven't had much experience with the process using `zellij`. 

First things first; how can we figure out which system process is actually running the session? Well, there are a few ways of going about this; the first one is really just pulling up `htop`, and separating processes that are being run by my user; since there are relatively few, this ended showing the problem pretty easily. However, it got me wondering; what if I was on a much larger system, or had started the process running via `root` for some reason. That might have made it harder to check just by manually browsing a system monitor. Is there a way to specifically find which processes are holding ports open? According to [this](https://superuser.com/questions/307624/how-to-close-or-unbind-a-port-left-open-by-a-rogue-application) Stack Overflow thread, the answer is yes. I ended up going with the `$ lsof -n -i` command which, according to its man page, "lists  on  its standard output file information about files opened by processes."  In this case, we get an output that looks something like the following (some of the IP addresses have been obfuscated):

<div class="code-block">
<pre style="width: 120%">

```sh
 COMMAND      PID   USER   FD   TYPE  NODE NAME
 GeckoMain   3709 chrism   35u  IPv4  TCP x.x.x.x:53930->x.x.x.x:https (ESTABLIS
 GeckoMain   3709 chrism   81u  IPv4  TCP x.x.x.x:55684->x.x.x.x:https (ESTABLIS
 GeckoMain   3709 chrism 3380u  IPv4  TCP x.x.x.x:47824->x.x.x.x:https (ESTABLIS
 GeckoMain   3709 chrism 3408u  IPv4  UDP *:mdns
 multiple_ 107620 chrism   10u  IPv4  TCP 127.0.0.1:25000 (LISTEN)
 multiple_ 107620 chrism   39u  IPv4  TCP 127.0.0.1:59464->127.0.0.1:25000 (ESTA
 multiple_ 107620 chrism   46u  IPv4  TCP 127.0.0.1:25000->127.0.0.1:59464 (ESTA
```
</pre>
</div>

In this case, it's pretty clear. I'm assuming the GeckoMain has something to do with Firefox, but the `multiple_nodes` application is holding port 25000, which is exactly what we're looking for, running with a PID of `107620`. At this point, I could run a command like `$ kill -9 107620` to get rid of that process, but curiosity got the better of me; how would I re-attach the process if I wanted to examine it further?

It turns out, Zellij has a set of commands to deal with this as well. The combination of `$ zellij ls` and `$ zellij attach session-name` will do the trick. I'm not certain, but it looks like the sessions are listed by Zellij in chronological order; since I had opened up a new session at the start of the day, I guessed that the bottom-most session would probably be holding the detached process. Sure enough, it was, and I was able to end it and get back to work. 

### key takeways
<ul>

• `sled`'s exclusivity means you won't be able to run multiple default Host nodes from the same directory.

• `$ lsof -ni` will list all processes that are using a networking port, including their PID 

• `$ zellij -ls` will list all open sessions in chronological order

• `$ zellij attach session-name` will re-attach a session that has been detached using `zellij` => `Ctrl-O` => `d`.
</ul>

