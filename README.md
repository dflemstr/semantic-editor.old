# Semantic Editor [![Build Status](https://travis-ci.org/dflemstr/semantic-editor.svg)](https://travis-ci.org/dflemstr/semantic-editor)

A toy project that I work on whenever I'm bored.  Eventually it's
supposed to become a semantic editor (i.e. a file editor for editing
semantic structure -- not bytes or characters -- making it impossible
to perform for example syntax errors) but that will require some work.

Currently I've focused on deployment.  The latest version of the
editor can at any time be downloaded with:

    wget dflemstr.name/se
    chmod +x se

After you have a version available, you can run:

    se update

...to self-update the editor at any point (but it must be allowed to
write to the directory where the executable resides -- maybe use
`sudo`...)

For more information, use the built-in command-line help:

    se help
