## Switching to Alacritty

*This is half explanation, half how-to guide so I don't have to figure this whole thing out again the next time I want to set up a new machine.*

I've been using [GNOME Terminal](https://help.gnome.org/users/gnome-terminal/stable/) as my terminal emulator for a fair bit now. This wasn't by any particular choice on my part, but rather that it's the default system binary on most of the Linux distributions that I've used, including my current, Pop!_OS. This has worked out fine for the most part, but I recently ran into an issue between GNOME Terminal and my terminal multiplexer, [Zellij](https://zellij.dev/).

Like a lot of people, I started off using [tmux](https://github.com/tmux/tmux) as my multiplexer (and still do, especially on smaller, simple systems like the host computers on some of the robots I've worked with). After a while, however, I decided to give Zellij a try and found that I really enjoyed the explicitness of the directions on the system. Now that I'm more familiar with it, I have many of the key shortcuts memorized, but I still often appreciate the convenience of having the information laid out right in front of me, especially on days when my brain is moving a little more slowly than usual.

However, this led me to a problem; there's a bug in the compatibility between Zellij and GNOME Terminal, where copy-paste (which should be automatic through highlighting) doesn't seem to work, as of `zellij 0.19.0` and `GNOME Terminal 3.38.0`, maybe because of [this issue?](https://gitlab.gnome.org/GNOME/vte/-/issues/125). Since I've never had much of an attachment to GNOME Terminal, the right move seemed to be switching my terminal emulator. But to what?

Well, I'd heard good things about [Alacritty](https://alacritty.org/), and I liked that it was also written in Rust (Zellij is, too), so I figured I'd give that a shot. At this point, I'e had this set-up installed for a few days, and I'm not missing GNOME Terminal at all (plus, copying now works!). 

Anyways, here's some tips on how to install/configure this set-up: 

To start, of course, both Zellij and Alacritty need to be installed:
```sh
    # requires that cargo is installed
    $ cargo install zellij
    $ cargo install alacritty
```
### Configuring Alacritty, or *"ooooo, pretty colors"*

I like the ["hyper"](https://github.com/rajasegar/alacritty-themes) theme, personally, which can be added to the `~/.config/alacritty/alacritty.yml` configuration file under `colors:`. 
In addition, we'll want to apply the following to the `.bashrc` file in order to make our shell prompt more colorful. This done through the customization for the PS1 shell variable (Prompt String 1); a more in-depth overview of this feature can be found [here](https://www.linuxnix.com/linuxunix-shell-ps1-prompt-explained-in-detail/), with an explanation of groking this config string [here](https://tldp.org/HOWTO/Bash-Prompt-HOWTO/x329.html).

In addition, it might be helpful setting the `TERM` variable; I was attempting to SSH into a Raspberry Pi Zero, and found that I kept receiving an error during commands about Alacritty not being recognized; changing this environment variable on the host computer to a simple `=linux` solved this problem (per [this](https://techtitbits.com/2010/10/resolving-unknown-unknown-terminal-type-error/) post);

<div class="code-block">
<pre style="width: 120%">

```sh
    # Apply to the .bashrc file, can be done through `echo $PS1_CONFIG >> ~/.bashrc`
    # From https://wiki.gentoo.org/wiki/Alacritty#Colorless
    # Sets the hostname colors in Alacritty
    export PS1="\[\033[01;32m\]\u@\h\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ "
    export TERM=linux
```
</pre>
</div>


### Manual Set-up

<div class="code-block">
<pre style="width: 125%">

```sh
    > GNOME Keyboard Settings
        > "Customize Shortcuts"
            > "Custom Shortcuts"
                > "Add shortcuts"
                    - `alacritty -e zellij`
                    - Set keybinding to terminal launch (i.e. `Ctrl+Alt+T`)    
```
</pre>
</div>

### Automatic Set-up

In order to set Alacritty as the default terminal, we'll do the following: 

<div class="code-block">
<pre style="width: 110%;">

```sh
    # https://gist.github.com/aanari/08ca93d84e57faad275c7f74a23975e6
    # Remember to add the PS1 config to .bashrc for the pretty colors
    $ cargo install zellij alacritty
    $ sudo update-alternatives --install \
      /usr/bin/x-terminal-emulator x-terminal-emulator \
      $(which alacritty) 50
    $ sudo update-alternatives --config x-terminal-emulator
```
</pre>
</div>

and add to the `~/.config/alacritty/alacritty.yml` file, based on this Github [issue](https://github.com/zellij-org/zellij/issues/823):

```yaml
    shell:
        program: /usr/bin/bash
        args:
        - -l
        - -c
        - zellij attach --index 0 || zellij
```