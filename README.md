# running
```
git clone https://github.com/trueharuu/6res
```
in an `.env` file:
```
TOKEN=
HOSTS=
PREFIX=
```
then just run `tsc` in any terminal to compile and then `node dist` to run

# playstyle options
- `pps`: pieces per second without any pacing
- `burst`: pieces per second during a [burst] phase
- `slack`: pieces per second during a [slack] phase
- `vision`: amount of pieces in the queue that the bot can consider
- `foresight`: amount of pieces *after* vision to "guess" for; it's used to decide the "goodness" of tied continuations
- `can180`: whether to do 180s
- `finesse`: style of placements; either `human` or `instant`
- `start_threshold`: amount of pieces at the start of a match to hold `burst` for
- `break_threshold`: amount of pieces after breaking combo to stay in `slack`
- `garbage_threshold`: amount of incoming garbage to be "comfortable" with before changing pace
- `gb_weight`: determines behaviour for when `garbage_threshold` is passed. if greater than 0, enter `burst`, if less than 0, enter `slack`
- `pace`: enables/disables all pacing.
