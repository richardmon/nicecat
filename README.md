# Nicecat

This tool lets you simulate any server and use custom responses to test your application.

## Why?
In the past I've been in situation where I had to work in an application that had no unit test, no mocking for external services, no way to test my code but with some logs, print statements, and maybe checking changes in a database, for this reason I decided to create a tool that lets me control all HTTP connections with the outside world, giving me more visibility into what is going on.

## Tasks
- [ ] Implement a way to respond to request with a custom body
    - This could either be achived by being able generate custom handlers or doing some kind of dirty url matching.
- [ ] Catch HTTP requests coming out of my PC
- [ ] Write a TUI that shows the requests going through your network
- [ ] Implement a YAML configuration file 
