# batch_renamer

**<ins>Recursively</ins>** renames files in subdirectories after their containing dirs,

Like ;

```
Foo/Bar/File.jpg >> Foo/Bar/FooBar0.jpg
Foo/Bar/AnotherFile.jpg >> Foo/Bar/FooBar1.jpg
Foo/Bar/ADifferentFile.png >> Foo/Bar/ADifferentFile.png
Foo/Bar/YetAnotherFile.jpg >> Foo/Bar/FooBar2.jpg
...

TopDir/Foo/Bar/File.jpg >> TopDir/Foo/Bar/FooBar0.jpg 
```
etc.

Will ignore files only 1 subdirectory deep and in current dir.

Files are first written to temporary files to avoid clobbering. 

If anything goes wrong, check for .batcher_renamertmp files. Nothing should be lost.

This prog is nondestructive and should not delete or overwite files, but will cheerfully rename every file on your PC if allowed. Do be careful.

## usage


`./batch_renamer -[xvdh] -g "glob pattern"`
for example
`./batch_renamer -xv -g "*.png"`

* -x                - Execute renaming, use with caution.
* -v                - Verbose mode. Print actions to terminal
* -d                - Dry run, no file manipulation. Combine with -v.
* -g "glob pattern" - Optional. The next argument will be taken as a glob pattern to use. Globs for "*.jpg" by default.
* -h                - Print help and exit.

Look first to see if it will do what you want.

## todo 
* Capture args and include in glob to search for other file extensions - DONE
* Optional rename depth from args
* Other args as well, such as -h help screen as run default, -d for dry run, -G to exec with confirmation - DONE