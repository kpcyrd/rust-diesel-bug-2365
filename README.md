This is an attempt to trace a locking issue I've experienced with diesel:

https://github.com/diesel-rs/diesel/issues/2365

Diesel is awesome and I hope this test case can be used to resolve this issue.
If you're using diesel commercially please consider donating to the developers
through github sponsors:

https://github.com/sponsors/sgrif
https://github.com/sponsors/weiznich

Output, at the time of writing (diesel 1.4.5):
```
Testing rusqlite
Setting up database
Spawning 25 threads and increase a field by one 100 times
All threads finished
Verified database content successfully

Testing diesel
Setting up database
Spawning 25 threads and increase a field by one 100 times
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
Error: Failed to execute PRAGMAs: database is locked
All threads finished
Expected field to be: 2500, but is actually: 100
```
