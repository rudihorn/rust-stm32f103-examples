# Description
This is an example that demonstrates how to make use of static variables, which may or may not be initialized. For more information on what is going on see http://rudi-horn.de/Home/6-rust-stm-handling-static-variables.

## Note
This example does not run if compiled with --release for some reason. Unfortunately I don't have a debugger (yet) and so it is a bit difficult to find out what the exact cause is, but it seems to be the memclr function and may be related to the actual function call to memclr. Changing the memclr amount to few enough bytes so it inlines it causes it to work fine. If anyone has any ideas let me know!

Edit: I've got a debugger and wrote a small post on debugging and what seems to be the issue with this source code, even though I have not quite found a solution yet http://rudi-horn.de/Home/7-debugging-the-stmf-with-an-st-linkv-and-openocd.
