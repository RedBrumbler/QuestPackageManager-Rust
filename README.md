# Deprecation notice
This version of qpm is deprecated, if you are looking for qpm for quest modding purposes you should get it from the new location at https://github.com/QuestPackageManager/QPM.CLI

# QuestPackageManager-Rust

QPM but rusty, this is a program that handles package downloading for quest modding, allowing modders to create packages to provide functionalities for mods.

# Building the program

First, make sure you have [Installed Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Open a command line / Powershell window

clone the repo

```
git clone https://github.com/RedBrumbler/QuestPackageManager-Rust.git
```

go into the folder

```
cd QuestPackageManager-Rust
```

run the build command

```
cargo build --release
```

the executable should now be found in `./target/release/qpm-rust`

if you want to use it like this, add it to path or move it to a place of your choosing that's already added to path.

# Downloading the program

Download qpm-rust from the [latest github actions build](https://github.com/RedBrumbler/QuestPackageManager-Rust/actions/workflows/cargo-build.yml), or if you're on windows [Download the installer](https://github.com/RedBrumbler/QuestPackageManager-Rust/actions/workflows/windows-installer.yml) from the latest action since that's easier. then you can also disregard the next instructions unless you absolutely want to get the executable yourself.

if nothing shows up, make sure you're logged in, if nothing still shows up we might have to manually make it generate a new version
Make sure you select the appropriate platform for your OS!

Now that you have this downloaded, you can unzip it and store it where you want it. I keep my qpm-rust executable in `S:/QPM-RUST` (irrelevant but just an example)

Now you want to add the program to path so that you can run it from anywhere, your best bet is to just google how to do this for your platform. just make sure that after you add it to path you restart any terminals you had left open.

Now to check if you installed it right you can run

```
qpm-rust --help
```

and you'll get a handy help message to get you on your way to using this program

# Requirements for using the program properly

To use the program properly with quest modding, you might also want to install the following programs, and make sure they are on path:
 - [Git](https://git-scm.com/downloads), used for downloading the repos that packages are stored on
 - [CMake](https://cmake.org/install/), generally used for compiling mods that use qpm-rust
 - [Ninja](https://ninja-build.org/), Used for building mods with cmake
