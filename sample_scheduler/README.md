# Sample Schedueler
This is the start of a scheduler that I am building to handle a command, assign it a time then execute it at that time. The command is a tuple of (time,command).

This program will also create two new files when it runs:

1. A *scheduler_log* folder will be made of any logging that is made as part of the code.
2. A *saved_commands* folder which will have the command saved in it's own timestamped file. As of now, if multiple commands are given, a new file will be created for each one. There is a rolling file system in place for this that right now holds 2 KB of files.

***IMPORTANT:*** The scheduler expects time arguments in UTC to compare the current time with the time of the command.

**TIME OF EXECUTION** is expected in this format: *YYYY-MM-DD HH:MM:SS*

### Example Command:
```bash
cargo run

1

2024-05-39 22:34:50
```

where 1 is the command and the line below it is the execution time.