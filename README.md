# lastfm-charts

Get your user charts from LastFM as text output from your command line.

## Installation
Linux:
- Download the latest release
- Make the binary executable: `chmod +x lastfm-charts`
- Copy to your shell path: `cp lastfm-charts /usr/local/bin`

## Usage
```
lastfm-charts -u <username> [options]

-l, --limit <limit>           limit results from 1 to 10 (default: 5)
-p, --period <period>         get results from a range of periods : 7day, 1month, 6month, 12month, overall (default: 7day)
```
## Example
Get user foo's top 3 artists from the last 6 months
```
lastfm-charts -u foo -l 3 -p 6month

foo's charts
1. Artist 1
2. Artist 2
3. Artist 3
```
```
