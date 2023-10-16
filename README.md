# batch_renamer

**<ins>Recursively</ins>** renames .jpg files in subdirectories after their containing dirs,

Like ;

```
Foo/Bar/File.jpg >> Foo/Bar/FooBar0.jpg
Foo/Bar/AnotherFile.jpg >> Foo/Bar/FooBar1.jpg
Foo/Bar/YetAnotherFile.jpg >> Foo/Bar/FooBar2.jpg
...

TopDir/Foo/Bar/File.jpg >> TopDir/Foo/Bar/FooBar0.jpg 
```
etc.

Will ignore files only 1 subdirectory deep and in current dir.

If anything goes wrong, check for .tmp files. 

Files are first written to temporary files to avoid clobbering. 

This prog is nondestructive and should not delete or overwite files, but will cheerfully rename every .jpg on your PC if allowed. Do be careful.

## usage


batch_renamer -x -q -g "glob pattern" 


When run with no arguments will print a dry-run and the help screen.

-x                - Execute renaming, use with caution.
-q                - Quiet mode - Suppress prints to terminal.
-g "glob pattern" - Optional. The next argument will be taken as a glob pattern to use. Globs for "*.jpg" by default.
-h                - Print help and exit.

Look first to see if it will do what you want.

## todo 
* Capture args and include in glob to search for other file extensions - DONE
* Optional rename depth from args
* Other args as well, such as -h help screen as run default, -d for dry run, -G to exec with confirmation - DONE