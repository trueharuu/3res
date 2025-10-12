# running
```
git clone https://github.com/trueharuu/3res
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
- `vision`: amount of pieces in the queue that the bot can consider
- `foresight`: amount of pieces *after* vision to "guess" for; it's used to decide the "goodness" of tied continuations
- `can180`: whether to do 180s
- `finesse`: style of placements; either `human` or `instant`
