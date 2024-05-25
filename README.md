# coffee-machine-controller
Code for controlling a coffee machine on/off-switch with a Raspberry Pi

## Setup
We compile to target `aarch64-unknown-linux-gnu` to run on our RPi.

## Structure
We have a *back-end* and a *front-end*. The back-end is in charge of controlling the actual coffee machine while the front-end is for the user to actually control everything.

For the database will just use a simple **[insert db here]**.

For logging we're using `log4rs` which is easy to setup crate that will allow us to have a rolling log file with limited size that keeps track of what our program is doing.

## .env
In the `.env` file we just need to fill in the login information for the database server like so

```
user = user_name
pass = user_pass
```
