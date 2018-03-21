# changed-tests

> Quickly identifies tests that have changed in your repo.

**NOTE:** This file is all lies right now. I'm currently just trying to get a
good idea on how it should work.

## Usage

```
changed-tests --help

USAGE: changed-tests [OPTIONS] [TYPE]

TYPES:
  - rspec (default)
    Look for RSpec changes and only show files that can be given to the `rspec`
    command.

OPTIONS:
  -e, --execute
      Run tests instead of just listing them.

  -l, --list
      Only list tests to STDOUT.

  -b [BRANCH], --branch=[BRANCH]
      Diff against given branch name instead of HEAD commit. Defaults to
      `origin/master` when not specified.

  -w, --whole-files
      Always run the whole test files instead of trying to extract subsets of
      them (where supported).

  -v, --verbose
      Print test command to STDOUT before executing.
```

## RSpec support

The RSpec support will try to extract individual tests to run. If a shared
example is changed, all files that use that shared example will be run. This is
not context sensitive so files containing different shared examples with the
same name will all be found this way.

## License

MIT
