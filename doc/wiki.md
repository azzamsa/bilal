# Wiki

## Cofiguration

You can get latitude and longitude value from online services, such as
[mapcoordinates](https://www.mapcoordinates.net/en).

### Method

Available methods are:

| Value | Description |
| ----- | ----------- |
| `MuslimWorldLeague` | Muslim World League. Fajr angle: 18, Isha angle: 17 |
| `Egyptian` | Egyptian General Authority of Survey. Fajr angle: 19.5, Isha angle: 17.5 |
| `Karachi` | University of Islamic Sciences, Karachi. Fajr angle: 18, Isha angle: 18 |
| `UmmAlQura` | Umm al-Qura University, Makkah. Fajr angle: 18.5, Isha interval: 90.  |
| `Dubai` | Method used in UAE. Fajr angle: 18.2, Isha angle: 18.2. |
| `Qatar` | Modified version of Umm al-Qura used in Qatar. Fajr angle: 18, Isha interval: 90. |
| `Kuwait` | Method used by the country of Kuwait. Fajr angle: 18, Isha angle: 17.5 |
| `MoonsightingCommittee` | Moonsighting Committee. Fajr angle: 18, Isha angle: 18. |
| `Singapore` | Method used by Singapore. Fajr angle: 20, Isha angle: 18. |
| `NorthAmerica` | Referred to as the ISNA method. Fajr angle: 15, Isha angle: 15 |
| `Other` | Fajr angle: 0, Isha angle: 0. |

This docs is adapted from [insha/salah](https://github.com/insha/salah/).

### Madhab

This settings corresponds to Asr prayer time.
For Hanafi madhab, the Asr is bit later than that of the Shafi madhab.

| Value | Description |
| ----- | ----------- |
| `Shafi` | Earlier Asr time |
| `Hanafi` | Later Asr time |

## Contribution

### Release

To make a release. Change the `version` in `Cargo.toml` then run:

``` bash
$ source scripts/relase.sh
```
