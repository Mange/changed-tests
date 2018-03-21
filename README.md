# changed-tests

> Quickly identifies tests that have changed in your repo.

**NOTE:** This file is all lies right now. I'm currently just trying to get a
good idea on how it should work.

## Usage

```
changed-tests 0.1.0
Magnus Bergmark <magnus.bergmark@gmail.com>


USAGE:
    changed-tests [FLAGS] [OPTIONS] [TYPE]

FLAGS:
    -e, --execute
            Run tests instead of just listing them.

    -h, --help
            Prints help information

    -l, --list
            Only list tests to STDOUT.

    -V, --version
            Prints version information

    -w, --whole-files
            Always run the whole test files instead of trying to extract subsets of them (where supported).


OPTIONS:
    -b, --branch <BRANCH>
            Diff against given branch name instead of HEAD commit. Can be specified without a value to diff against
            upstream master. [default: origin/master]

ARGS:
    <TYPE>
            The type of tests to search for. This option is case insensitive. [default: RSpec]  [possible values: RSpec]
```

## RSpec support

The RSpec support will try to extract individual tests to run. If a shared
example is changed, all files that use that shared example will be run. This is
not context sensitive so files containing different shared examples with the
same name will all be found this way.

## License

MIT
