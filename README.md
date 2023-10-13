# batch_renamer

**<ins>Recursively</ins>** renames .jpg files in subdirectories **at least** two steps away after their containing dirs,

Like ;

```
Parent/Dir/File.jpg >> Parent/Dir/ParentDir0.jpg

GrandparentParent/Dir/File.jpg >> Grandparent/Parent/Dir/ParentDir0.jpg 

Parent/Dir/File.jpg >> Parent/Dir/ParentDir0.jpg
Parent/Dir/AnotherFile.jpg >> Parent/Dir/ParentDir1.jpg
Parent/Dir/YetAnotherFile.jpg >> Parent/Dir/ParentDir2.jpg
```
etc.

While it should not delete or overwite files, this prog is still potentially destructive.

Look at the printout to see if it will do what you want.

And pass "GO" as the first argument to execute renaming.

* Currently only manipulates .jpg files. Plans to improve this
* 
