# Sample Scheduelr
This is the start of a scheduler that I am building to handle a command, assign it a time then execute it at that time. The command is a tuple of (time,command).
***IMPORTANT:*** The scheduler expects time arguments in UTC to compare the current time with the time of the command.
For now, the command is the first command line argument and the time of execution is the second.
**TIME OF EXECUTION** is expected in this format: *YYYY-MM-DD HH:MM:SS*