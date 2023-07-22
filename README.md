# uplay-stub
A stub for the Ubisoft Connect r1 API and compatible games.

## Tested titles
| Name | Storefront | State | Version | File name |
|---|---|---|---|---|
| Tom Clancy's Splinter Cell Blacklist | Steam | Plays fine, saving doesn't work at all yet. | 0.1.0 | `uplay_r1_loader.dll` |
| Far Cry 3 | Steam | Crashes on launch | 0.1.0 | `upc_r1_loader.dll` |
| Far Cry 4 | Steam | Crashes on launch | 0.1.0 | `uplay_r1_loader64.dll` |
| Far Cry 5 | Steam | Forcefully launches Uplay, requests Steam to revalidate | 0.1.0 | `uplay_r1_loader64.dll` |

# Development Notes
Uplay makes heavy use of singletons and has a class per "function" namespace, e.g ACH for achievements, PARTY for parties, and so on.  
Most function calls are preceded by a call to retrieve the singleton, then the function, and then the function is called with parameters and the given singleton.

Uplay itself seems to be a C++ written interface, with an awkward C wrapper.
