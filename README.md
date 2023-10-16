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

When run no arguments will print a dry-run.

Look first to see if it will do what you want.

And pass "GO" as the first argument to execute renaming.

# todo 
* Capture args and include in glob to search for other file extensions
* Optional rename depth from args
* Other args as well, such as -h help screen as run default, -d for dry run, -G to exec with confirmation

