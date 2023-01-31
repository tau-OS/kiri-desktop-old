# d5 - the Kiri Session Manager

`d5` is a session manager for the Kiri desktop, written in Rust.

It handles all the desktop and D-Bus session stuff that make a proper desktop session, like notification and managing XDG desktop daemons.

It is useless in of itself. But combined with other Kiri components, it is a major component that directly impacts the UX of Kiri Desktop.

## What's with the name?

**d5** is short for "dadadada daemon". It's no-good in of itself, [but thats the point!](https://youtu.be/ANp0qch3XVM)

But the question is, *is the no-no-no-no-no-good daemon actually no-no-no-no-no-good?* We'll see in the future.

## Why? why don't we just bootstrap using the GNOME session?

Yeah, the GNOME session launches the desktop, but the system still needs a daemon for the desktop userspace. The GNOME Settings Daemon is one example of this.

Another example is Budgie's session daemon, which manages system notifications and displays it onto the desktop.

Also, this is more of an experiment to see if I could do a proper desktop session using Rust.