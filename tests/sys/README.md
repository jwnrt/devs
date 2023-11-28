# Mock sysfs directory

This directory contains a mock Linux sysfs directory for testing with.

The structure here should be built up to resemble a real `/sys/` as closely as
possible, though the files here obviously won't control real devices.

There are some empty files in `devices/...` since Git will only track files and
not empty directories. These will be replaced with mocks of various sysfs
attribute files in the future.
