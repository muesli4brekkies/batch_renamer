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


## usage

`./batch_renamer -[hpxvs] -g "glob pattern"`
for example
`./batch_renamer -xv -g "*.png"`

* **-h**                - Print help and exit.
* **-p**                - Practice run, no file manipulation. Combine with -v.
* **-x**                - Execute renaming, use with caution.
* **-v**                - Verbose mode. Print actions to terminal
* **-s**                - Optional. Sort by exif timestamp, ascending. Useful otherwise the program shuffles things around.
* **-g "glob pattern"** - Optional. The next argument will be taken as a glob pattern to use. Globs for "*.jpg" by default.

## notes

Works recursively from the directory it is run from.

Will ignore files in current dir and 1 subdirectory deep.

Files are first renamed to temporary files to avoid clobbering. 

If anything goes wrong, check for .brtmp files. Nothing should be lost.

This prog is nondestructive and should not delete or overwrite files.

But will cheerfully, and very quickly, rename every file on your PC if allowed, breaking ***everything***. Do be careful.

For instance, `./batch_renamer -x -g "*"` run from the root dir would be a killer.

**Look first to see if it will do what you want.**