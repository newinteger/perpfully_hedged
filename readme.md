Update 2023: Since FTX is no longer operational neither is this code, it can still however be forked and implemented for other exchanges as the heuristic still works.

# Perpfully Hedged

## What is this?
This is a automation of a delta neutral trading strategy. It hedges spot holdings with future perps and farms the funding rate. 

Currently it is only implemented for the markets on [FTX](https://ftx.com).

## How does it work?
The program will automatically find, enter and exit markets depending on past and future funding rates. A user inputted minimum 24h volume floor is used to filter markets that cannot accomodate for user size.

## How can I use this?
1. Clone the repo
2. Update `settings.txt` with your account information and minimum traded daily volume for a market
3. Build the program with `cargo build --release`
4. Automate the execution of the program

Note: the minimum daily volume should be input as a float i.e. `10000.0` and not `10000`

## Automation of execution
FTX currently pays funding payments every hour, therefore the program should be run at most once ever hour. However it can be run with bigger intervals if so desired. 

Automation can be done with `cron` or similar tools.

## Final notes
This program is provided as is and should only be used with your own precaution. I do not take any responsibility for any effects it can cause.