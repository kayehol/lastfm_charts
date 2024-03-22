# lastfm-charts

Get your user charts from LastFM as text output from your command line.

## Installation
Linux:
- Download the latest release
- Make the binary executable: `chmod +x lastfm-charts`
- Copy to your shell path: `cp lastfm-charts /usr/local/bin`

## Usage
#### Pass username as an argument

`lastfm-charts username`

Output:
```
username's weekly chart:
1. Artist 1
2. Artist 2
3. Artist 3
4. Artist 4
5. Artist 5
```
#### Limit the number of results shown (optional, default is 5)

`lastfm-charts username -l 3`

Output:
```
1. Artist 1
2. Artist 2
3. Artist 3
```

More support soon.
