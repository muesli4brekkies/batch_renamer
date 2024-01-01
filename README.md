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

*  **-h**               - Print this screen and exit.

*  **-v**               - Verbose terminal printing.
*  **-q**               - Disable terminal printing entirely. Overrides -v.

*  **-p**               - Practice run. Combine with -v to print what the script will do!
*  **-x**               - Execute renaming. Use with caution.

*  **-s**               - Optional Sort by EXIF timestamp ascending. Default is not to sort, so the files are ordered however the OS picks them up.
*  **-g** "glob_string" - Optional string to glob files with.        Defaults to "*.jpg".
*  **-d** <path>        - Optional path to run search from.          Defaults to directory the binary is run from.
       
## notes

Works recursively from the directory it is run from.

Will ignore files in current dir and 1 subdirectory deep.

Files are first renamed to temporary files to avoid clobbering. 

If anything goes wrong, check for .brtmp files. Nothing should be lost.

This prog is nondestructive and should not delete or overwrite files.

But will cheerfully, and very quickly, rename every file on your PC if allowed, breaking ***everything***. Do be careful.

For instance, `./batch_renamer -x -g "*"` run from the root dir would be a killer.

**Look first to see if it will do what you want.**