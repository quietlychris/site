## Technology I'm Excited About

It's possible that I'm going to be taking a break from "tech" for a while, as I take on a new challenge that doesn't necessarily involve writing software or building physical things. If things go accordingly to plan (although, of course, they never do), in maybe a year or so, I'll be looking at picking back up working as an engineer in the robotics area. I'm writing this as a bit of a note for myself about tech that I'm currently excited about, and a list of things that I should look into if/when I'm back into the field. 

### [Bevy](https://bevy.org/)

Bevy is maybe the project that I'm most looking forward to see progress in over the next year or so. I've played around with it for various simulations and visualizations over the past few years, and the ability to integrate native Rust code that I wrote for other projects directly into a game engine is a wild(ly good) experience. It's also among my gold standard reference projects in operations; the writing-based communication style, project leadership's self-reflection and EQ, willingness to critically evaluate and update their development workflows, insanely good release notes, and ability to balance being both opinionated and open to new ideas is inspiring. 

In my humble opinion, Bevy is currently sitting on an adoption precipice (it's a good thing if it goes over the edge) due to the number of features currently sitting on the roadmap:

- Bevy Scenes and the associated `.bsn` and `bsn!()` constructs: the prototypes are ostensibly targeted to land in the next couple of releases. 
- Bevy UI: Scenes open up UI. There's already a number of third-party crates here, but I expect that any maturity lead those hold will likely be minimized by the advantages the native option can leverage in terms of tight integration with scenes.
- Bevy Editor: Once Bevy UI begins to land and solidify, the Editor story will become more more front and center. The Figma [design doc](https://www.figma.com/design/fkYfFPSBgnGkhbQd3HOMsL/Bevy-Editor?node-id=90-2) on this that some members of the community are working on looks awesome, and would likely help to solve some of the big friction points I've had with Bevy (properly managing reference frames). 
- Firefox landed initial support for WebGPU in July 2025 on Windows, and is likely out-of-the-box WebGPU support for Mac and Linux within the next year. Since so many developers use these platforms, I'd expect the size of the communtiy 

### [Folding@Home](https://foldingathome.org/)

Folding@Home allows researchers to distribute small bits of work (typically protein folding simulations) or similar to folks who donate some electricity and otherwise-idle computing resources. I also don't have access to heat pumps, so in the winter (it hit below freezing in my neck of the woods area last night for the first time this year) my computer can basically act as a quiet little heater for my living room anyway. I've only recently started doing this, but it's been rewarding. For a computer that I already own and a few dollars worth of electricity, I have the privilege of supporting researchers working on some of the Big Problems like cancer, Alzheimer's, and addiction. My desktop is likely going into storage for a bit, but eventually I'd like to spin this up again. 

### [Burn](https://github.com/tracel-ai/burn) 

Over the years, I've occasionally tried getting into computer vision work. I flatter myself that I'm a 

For my purposes, the "network" is the least interesting part. I don't *care* about your ROC curve or what your activation function or how many residual blocks your big fancy architecture has or what version of CUDA happens to be installed on my machine. I just want to:

<ol type="a">Spin up a little Docker container with a [CVAT](https://github.com/cvat-ai/cvat) instance</ol>  

b) Spend a few hours drinking a cup of coffee while drawing bounding boxes 

c) Export the COCO-formatted archive

d) Pass that to a program that will use my desktop computer's big honkin gaming GPU to do some data augmentation and train a single-shot object detector that I can test locally

e) Cross-compile it into a single statically-linked executable that I can `scp` over to an NVIDIA Jetson Orin or whatever vision-enabled embedded board happens to be available, 

f) Then pass in `/dev/video0` as an argument to that executable, and start getting frames and bounding boxes and labels out at maybe 10 fps or better.

And at the moment, I just can't figure out how to do that. I either get pushed towards someone's (usually very frustrating) cloud service or towards a Medium article or a grad student's 7-year-old half-working Github repo. Burn seems like it has a lot of the pieces in place to make a difference here. First, it's written in Rust, and with the exception of maybe recently with [`uv`](https://github.com/astral-sh/uv), `cargo` is the best tool I've ever used for building software that Just Works(TM). The combination of:

a) Package reuse through `cargo`. I could just import a `burn-yolox` or `burn-ssd` package with a software-engineer-facing, *not* machine-learning-engineer-facing API. 

b) Support with WebGPU-based targets for cross-platform GPU support (or maybe NVIDIA gets their act together and has CUDA not absolutely suck to get working on a new platform)

c) First-class cross-compile support through the combination of LLVM-backed `rustc` and the [`cross`](https://github.com/cross-rs/cross) toolchain, with static linking so my big honkin desktop can do this instead of waiting 20 minutes for the teeny-tiny embedded cores on Orin to tell me that I've got a package mismatch. 

Tracel AI (the org building Burn) appears to be mainly focused on building out their infrastructure and core libraries at the moment, but is increasingly dipping their toe into potential user-facing applications for Large Language Models via projets like [`burn-lm`](https://github.com/tracel-ai/burn-lm). And yet, there's some work going on in their [`models`](https://github.com/tracel-ai/models) repository and support for things like COCO-formatted datasets, so maybe one day this won't just be a pipe dream. 

### 3D Printing

After years of being a bit bearish on 3D printing (I'd spent too many hours tring to calibrate initial layer offsets with a piece of paper), [Bambu](https://bambulab.com/en) turned me into a believer after using a couple of their X1C printers. But the rest of the field (Prusa, Creality, etc.) hasn't stepped back from the fight. The state-of-the-art in desktop printing is wildly different from what it was 5 years ago, and while 
I'm not sure what's next (the H2-based series was more evolutionary that revolutionary), the table stakes have changed for what a vanilla 3D printing experience should look like, and the genie's not going back in the bottle.

My big fear is that a similar story might happen with Bambu that happened with DJI drones. Once it became clear how much better the Chinese versions were (not just in price, but in quality), Western manufacturing basically gave up on building competitive consumer-focused drones, and pivoted to government-focused customers with prohibitive acquisition requirements where they were bigger fish in a (much) smaller pond. This, alongside general threats to DJI's Western market access via trade restrictions, suddenly put DJI in a position where both they weren't technically threatened (leading to small, incremental innovations), and also allowed them to be more aggressive with market segmentation as the de-facto leader in consumer drones, reducing previously table-stakes features available on their consumer models (like a developer-focused Mobile SDK) and pushing that thousands of dollars up-market into the professionally-focused vehicle segments. The newest flagship DJI Mavic 4 is barely available in United States at all, and the DJI Air2S (released 2021) is the last full-size (over 250 grams) drone that will work well with third-party mission planners. 

If governments decide to take the same cut-em-off-at-the-knees approach to Bambu that they have with DJI? Hopefully there's enough remaining competition from other players to keep the race on, but it would be a significant step down in pressure. 


### 3D Modeling

The current state of 3D modeling has been stuck in the early 20's for the past 20 years. Look at the Solidworks interface from [2004](http://solidworks.cad.de/images/sw2004/ohne-realview.jpg) and what it looks like in [2025](https://www.3ds.com/assets/invest/styles/banner/public/2024-09/wn25-sw1-2-real-time-notifications.png.webp?itok=tJFh_3nE). The GUI's gotten a little more streamlined, but at the core? We're still talking the Parasolid CAD kernel, which was first released in *1989*. OpenCascade was 1999. Solvespace is the most recent at only 2008, but still really only supports boolean operations (try convincing a mechanical engineer to use a tool that doesn't natively support fillets).[^1] 

I don't mean to say that all old software is bad software; I know that optimized, battle-tested code can be truly valuable. But I have coworkers have been given essentially blank cheques to build a CAD workstation, and Solidworks *still* freezes and crashes on some large models with no warning. I know that they still have to export models into other simulation tools like ANSYS because somehow after many many many years of work, it's still more an art than a science to get a FEA mesh to build without issues in Solidworks Simulation. I know that doing anything resembling modern version control (deleting a part from the overall assembly from a sub-branch? Nope! Gotta hide it instead) is a fool's errand and can't be managed using any reasonable workflow via Solidworks PDM. I know that even experimenting with other workflows via not-even-that-modern tools like Git won't work because turns out `.sldpart` and `.sldasm` binaries because they're really only made for plain-text and if you can't diff two parts or assemblies, what's the point? 

No amount of R&D and re-work on that project is going to turn it into a modern software stack. Fortunately, there are companies like [Zoo](https://zoo.dev/) and [Istari](https://www.istaridigital.com/product) that are putting in work on slightly more updated version of tooling that might be called "digital engineering" if that term wasn't wildly overloaded. [FreeCAD](https://www.freecad.org/features.php?lang=en) finally (finally!) has an out-of-the-box assembly workbench, which is a huge, and there's various small open source projects ([Fornjot](https://github.com/hannobraun/fornjot), [Truck](https://github.com/ricosjp/truck), among others) that are helping to better understand the work required in the modern software environment to build a parametric 3D modeling kernel. 




[^1]: I know that Solvespace is FOSS and it's probably not fair to include it in this list. 