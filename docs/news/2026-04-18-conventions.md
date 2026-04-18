---
title: "Changing Conventions"
date: 2026-04-18
---

# Version Prefix Change!

It seems that unnecessarily changing conventions and stuff is becoming the norm across tech with everything now 
becoming CalVer after 10 years of being SemVer, or basically the same thing but with phones, systems, etc. (not pointing out anybody here, but you know who..)

...and Farben's jumping right on top of that hype train! I'm planning on changing how the members' versions are being referenced! It's not as bad as changing version numbering convention anyway.

Right now they use the classic old `v` prefix. From now on, the version prefix will be changed across crates:

* farben: `frb*`, or just `v*` if there's no `frb*`
* farben-core: `core*`
* farben-macros: `macros*`
* farben-build: `build*`
* farben-md: `md*`

Why am I changing them? I'm not sure. It's just better to look at the versions in a glance and know which crates they represent instead of having to put their whole name infront of it (farben-macros 0.x.y vs macros0.x.y).

This wouldn't change anything about Farben itself, just how I'll represent the versions in future CHANGELOG entries or these news headers. Make great things with Farben!

Cheers, RazkarStudio
