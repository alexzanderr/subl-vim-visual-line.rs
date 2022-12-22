


# pip wont install a wheel
https://stackoverflow.com/questions/33213430/whl-is-not-a-valid-wheel-filename-storing-debug-log-for-failure-in-c

# watchexec doesnt work
bug desription: the first time a folder changes it works, the other times ... watchexec doesnt detect the changes and doesnt run the specified command

cause: the target folder was deleted and created again with new files, even if the file names were the same or the folder name was the same

solution: use `--force-poll <interval>` to force poll changes from the target paths

references:
- https://github.com/watchexec/watchexec/issues/179

- https://github.com/watchexec/watchexec/issues/55