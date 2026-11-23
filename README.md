# lindexer
a CLI file by-type sorter/indexer (soon with online capabilities), written in Rust

<p align="center">
  <img src="https://github.com/ghlilith1k/lindexer/blob/main/assets/preview.png" alt="lindexer preview"/>
</p>

Copyright (C) 2026 ghlilith1k - GNU/GPLv2 or later

# Description

lindexer is a CLI utility for by-type file sorting/indexing, basically it gets the list of all files in a specific directory (the main directory where you put all files of every kind you want to sort) and lindexer sorts them in the corresponding directory based on the file type. it works using a sequential algorithm that detects file wildcards, assigns a certain directory to it, then moves the file in the assigned directory. in the future, lindexer will also support online file grabbing/indexing (basically giving lindexer a list of youtube/spotify or other platforms links and lindexer converts and downloads the videos/audio/files and sort/indexes them automatically but this part is still getting planned). note that this project is pretty big and it will take alot of time to code, stay tuned for updates.
**bugs are expected, please report them in the "issues" tab**

# How to Install:

    make

# Usage:

    lindexer

# About

**note: lindexer has only been tested on GNU/linux, stability/full functionality and compilation on other OSes is not guaranteed.**

- current version: v0.0.1-alpha
- current stable: none
- total lines of code: 130
- for future releases: see "Dev Notes"

# Dev Notes / Known Issues

- finish parsing implementation
- work on sorting algorithm
- work on config arguments

# Changelog

## release v0.0.1-alpha - packaged

just a starting point for future release