% Learn you a git!
% Michael Noronha
% May 15, 2018

## Outline

- The problems
    - [Memorize the api](https://try.github.io/levels/1/challenges/1)
- Motivation
    - [The git parable](http://tom.preston-werner.com/2009/05/19/the-git-parable.html)
- Internals
    - [Git from scratch](https://codewords.recurse.com/issues/two/git-from-the-inside-out)
- Not exhaustive
- Please feel free to leave at any point!

## Motivation

- I make lots of mistakes
- Going back in time is helpful
- "Streaming" version control
    - Too much
- Snapshots!
    - Only save things we really care about

## Snapshots

- `working`
- `snapshot-0`
    - MESSAGE
- `snapshot-1`
    - MESSAGE
- ...
- `snapshot-90` *
    - MESSAGE
- ...
- `snapshot-100`
    - MESSAGE
- "Hey, there are some bugs"

## Snapshots

- Copy `working` to `snapshot-101`
- Overwrite `working` as `snapshot-90`
- Fix the bug
- Save it as `snapshot-90.5`
- Nonlinear!

## Branches

- Allow diverging paths
- `bugfix` and `development`
- New file: `branches`
    - `bugfix`: `snapshot-90.5`
    - `development`: `snapshot-101`
- Eventually the changes from `bugfix` will make it into development

## Tags

- Quality-of-life improvement
- Some branches are static, other's aren't
- `release-1`, `release-2`, ...

## Groups

- `snapshot-96`
- Plan out what each of you will do
- ...
- `snapshot-97`
- `snapshot-98`
- `snapshot-99`
- ...
- `snapshot-97`
- `snapshot-98`
- `snapshot-99`
- `snapshot-100`
- Central numbering authority?
- Solution
    - date, message, parent, author
    - named with hashes

## Merges

- History diverges
    - Working on two different features
    - Want to release both
- Resulting snapshot has two (direct) parents
- Replay each set of changes to get histories to match

## Staging

- Working on two features
- Realize they should be in different snapshots
- Every time a feature is done, copied to staging

## What we have so far

- Snapshots + metadata
- Branches
- Tags
- Merges
- Staging
- ###All files
    - `git log`
    - `git add`
    - `git commit`
- "gitlab"
    - `git push`

## Not quite done

- Not very efficient
    - Lots of near-identical copies
- `objects` database
- Only copy the file if it is new or changed
- File compression
- Implementation details
    - bubble sort = merge sort

## What we have now

- Everything from before...
- ...but more efficient!
- But how did the last step work?
    - Time to talk about internals
    - Really, this is just more about how git does it
- Let's look at the files!

## Internals

```
$ ls -a
magic.txt

$ git init
Initialized empty Git repository!

$ tree -a
.
├── .git
│   ├── HEAD
│   ├── branches
│   ├── objects
└── magic.txt
```

Wow, no magic!

- `magic.txt` is _outside_ the `.git` directory
- `HEAD`
- `objects`
    - objects database
- `branches`
    - Not actually used by newer git version

## add

```
$ git add magic.txt
```

This is like adding to `staging`.

- We can delete it from our working area
- We can edit it, and recover the version
- How does git do this?

## add

### Objects directory

- Hash file contents
    - `56170f5429b35dea081bb659b884b475ca9329a9`
- Create a directory: `.git/objects/56`
- Write compressed contents to `.git/objects/56/170f5429b35dea081bb659b884b475ca9329a9`
- If we change the file, it gets written somewhere else
- If we end up undoing the changes and running `add` again, still there

### Index file

- File: `.git/index`
- Hash written to an index
    - `magic.txt 56170f5429b35dea081bb659b884b475ca9329a9`

## Add

More files,

`index`:
```
magic.txt 56170f5429b35dea081bb659b884b475ca9329a9
secret.txt 9a9239ac574b488b956bb180aed53b9245f07165
```

`tree` again:
```
.
├── .git
│   ├── HEAD
│   ├── branches
│   ├── objects
│   │   ├── 56
│   │   │   └── 170f5429b35dea081bb659b884b475ca9329a9
│   │   ├── 9a
│   │   │   └── 9239ac574b488b956bb180aed53b9245f07165
└── magic.txt
```

## commit

Ready for a snapshot

```
$ git commit -m "first"
1 file changed, 1 insertion(+)
create mode 100644 magic.txt
```

- Jump back to at any time
- Captures lineage
    - One or more (merges) parents

- What is git doing?

## commit

- Tree graph
    - Blobs
    - Trees
- Blobs
    - File contents
    - Created by `git add`
- Tree
    - Captures `state`

## commit -- Tree

Everything needed to jump to a state.
```
100644 blob 56170f5429b35dea081bb659b884b475ca9329a9 magic.txt
```
`index`, plus a bit more!
- File permission
- `blob`, rather than `tree`
- Hash of contents
- File name

## commit object

```
tree b0e66a8a93b83161375f18dcdc9e9329af61e04f
author Michael Noronha <michaeltnoronha@gmail.com> 1526403571 -0500
committer Michael Noronha <michaeltnoronha@gmail.com> 1526403571 -0500
first
```

- Hash pointing to the tree graph
- Author
- Commiter
- Unix timestamps
- Message
- etc...

## HEAD

Small file that points that the current commit object

`.git/HEAD`:
```
ref: refs/heads/master
```

`master` is a _ref_ -- a name used to refer to a commit

`refs/heads/master` is stored in the `.git` directory at that path, set to the hash of the contents of the commit object.

## Another commit

- We make some changes
    - Blobs change
    - Index changes
- Create new tree object

## Resulting commit

```
tree e04fb0e66a8a93b83161375f1f18dcdc9e9329af61
parent f18dcdc9e9329af61e04fb0e66a8a93b83161375f1
author Michael Noronha <michaeltnoronha@gmail.com> 1526403572 -0500
committer Michael Noronha <michaeltnoronha@gmail.com> 1526403572 -0500
second
```

And `HEAD` and related `ref`s are updated based on the new commit.

Note that

- Any unchanged blobs aren't recreated -- they are just pointed to
- Commits (aftr the first) have parents

## checkout

- The point of our snapshots was to be able to jump back in time
- `checkout` lets us do this!
- Given a commit hash, jump to state at that commit
- How does git do it?

## checkout

- Find the tree graph of the commit
    - Look in `objects`
- Writes file entries to the working copy (by decompressing blobs)
- Write to `index` according to tree graph
- Update `HEAD`
    - "detached HEAD"
- We make some changes and commit
    - "third" points to "second"
    - But ref `master` doesn't point to it (or some other ref)
    - Work can be lost easily

## Branches

Branches are just _refs_!

- In other words, branches just point to commits
- Very cheap
- Checking out branches is just checking out the commit they point to

## conflicts

After making some changes to `magic.txt` on a branch

```
$ git checkout magic-branch
Your changes to these files would be overwritten
by checkout:
  magic.txt
Commit your changes or stash them before you
switch branches.
```

- Easy to detect conflicts!
- Avoid data loss at all costs
- Throw it back to the user
    - `commit`, `stash`

## merge

- Easy case: linear
    - do nothing
    - fast-forward
- Harder case:
    - ...

## merge

Creating a merge commit

#. Write to hash of "giver" commit to `.git/MERGE_HEAD`
    - Indicator that we're merging
#. Trace lineage through parents to find last common commit
    - "base" commit
#. Generate the index at each commit
    - We can step through these to find changes
#. Generate a diff that combines the changes
    - add, remove, modify, conflict
#. Apply the changes in the diff to the working copy
    - This is what `git status` would tell us before the merge commit
#. Append the changes to the index
#. Create a new commit from the updated index
    - This will have two or more parents
# Point the working branch at the new merge commit
- Of course, it can get worse!

## merge

```
<<<<<<< HEAD
0
=======
1
>>>>>>> magic_branch
```

- Suppose the commits modify the same files
- We'll go through the indices
- Some entries will be marked as conflicts
    - `0` in the index
- The merge pauses
- User edits the file
    - `add` indicates the conflict is resolved in that file
- The user commits
    - `.git/MERGE_HEAD` is deleted, if all conflicts are resolved
- `HEAD` is updated to point to the new commit

## Sharing

- We want to be able to work with other people
- With our snapshots, we were sharing files, somehow
- How does git do it?
    - Surely we aren't passing commit object files around :)

## remotes

- remotes are set in `.git/config`
- Additional refs added to `.git/refs/remotes`
- Links our repository to another

```
[remote "origin"]
    url = ...
```

## misc

- Deleting things removes them from the index, and thus the tree
- Objects remain
- Copying a directory clones the repository
    - Just files :)
- Cloning is just copying

## Bare repositories

- Alice is working with Bob
- They decide the "canonical" repository will live on Alice's computer
- Bob will `push` to it
- Special `bare` repository
    - Indicator in `config`
    - No need for working space
        - No `.git`
    - Given to us by gitlab, github, etc.

## Learn more

- [The git parable](http://tom.preston-werner.com/2009/05/19/the-git-parable.html)
    - Motivation, story form
- [Git from the inside out](https://codewords.recurse.com/issues/two/git-from-the-inside-out)
    - Look through the files
- [Git internals](https://git-scm.com/book/en/v2/Git-Internals-Git-Objects)
    - Part of the git book
