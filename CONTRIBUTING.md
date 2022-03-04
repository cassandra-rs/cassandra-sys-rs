# How to Contribute

## Issue Contribution
When opening issues please contain a description of the issue, plus example
code which exhibits the issue.

## Code Contribution
If you wish to contribute code to this project, you are welcome to do so
using a github pull request, however you will need to sign each commit,
see details below.

A few important notes:

* Please use `cargo fmt` to format the code. If it's not already properly formatted, please put the reformatting into a separate commit so it is easier to review; don't combine it with actual changes.
* There are no tests in this repo, but there are examples which at least use most features. If it makes sense, when you're adding a feature please add to the examples.
* Please do ensure you carry your changes through to https://github.com/Metaswitch/cassandra-rs and add tests there (if that makes sense).
* In general: I know this code isn't as nice as it could be, but please ensure new code does things better, and if you have an opportunity to clean up old code then please do so.

If updating the Cassandra driver version, there are examples at #18 and #39. The steps are:
  * Update `cassandra.h` from the latest driver.
  * Regenerate the binding code with bindgen utility, using `--size_t-is-usize` for compatibility.
  * Update or add examples - we should ideally have code that uses each new binding.
  * Update the changelog.
  * Update the CI files to download the correct driver library.

(We try not to update the driver version too often, since this forces clients to update their driver too. You should only do this if you need to use a newly-added binding.)

### Sign your work

The sign-off is a simple line at the end of the explanation for the patch. Your
signature certifies that you wrote the patch or otherwise have the right to pass
it on as an open-source patch. The rules are pretty simple: if you can certify
the below (from [developercertificate.org](http://developercertificate.org/)):

```
Developer Certificate of Origin
Version 1.1

Copyright (C) 2004, 2006 The Linux Foundation and its contributors.
1 Letterman Drive
Suite D4700
San Francisco, CA, 94129

Everyone is permitted to copy and distribute verbatim copies of this
license document, but changing it is not allowed.

Developer's Certificate of Origin 1.1

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I
    have the right to submit it under the open source license
    indicated in the file; or

(b) The contribution is based upon previous work that, to the best
    of my knowledge, is covered under an appropriate open source
    license and I have the right under that license to submit that
    work with modifications, whether created in whole or in part
    by me, under the same open source license (unless I am
    permitted to submit under a different license), as indicated
    in the file; or

(c) The contribution was provided directly to me by some other
    person who certified (a), (b) or (c) and I have not modified
    it.

(d) I understand and agree that this project and the contribution
    are public and that a record of the contribution (including all
    personal information I submit with it, including my sign-off) is
    maintained indefinitely and may be redistributed consistent with
    this project or the open source license(s) involved.
```

Then you just add a line to every git commit message:

    Signed-off-by: Joe Smith <joe.smith@email.com>

Use your real name (sorry, no pseudonyms or anonymous contributions.)

If you set your `user.name` and `user.email` git configs, you can sign your
commit automatically with `git commit -s`.

### Take care with the base revision

This repository is a fork of another repository. This means that whenever
you create a pull request, it defaults to using that other repository as
base.

**Please make sure you change the base to this repository's master branch
before submitting the pull request.**

If you make a mistake, unfortunately it is not possible to delete an
erroneous pull request from GitHub. However you can change the name
of the pull request to something like "(please ignore)" and close it.
