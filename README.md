# PREREQ
## libsteam_api.so
`find /home -type f -name libsteam_api.so`

`libsteam_api.so` will need to be in the same directory as the `samuel` binary. Or change the .cargo/config.toml in this project, I don't really know how to link-- maybe you do!

## ENV VARS
### STEAM API KEY
[request a steam api key](https://steamcommunity.com/dev/apikey), they don't seem to care what website you use, I use my personal Github page as my website. I'm sure if you use a domain that isn't yours or agreed to be managed by you, you might get in trouble.

Set the env variable API_KEY={API_KEY}

### STEAM ID
Get your steam ID

Set the env variable STEAM_ID={STEAM_ID}

# BUILD
```
cargo build --release
```

# RUN
`samuel games list`
```
id        name                                              playtime (h)
240       Counter-Strike: Source                            64.0      
320       Half-Life 2: Deathmatch                           0.5       
3920      Sid Meier's Pirates!                              41.8      
4700      Total War: MEDIEVAL II - Definitive Edition       70.4      
```
`samuel games list --no-header`
```
240       Counter-Strike: Source                            64.0      
320       Half-Life 2: Deathmatch                           0.5       
3920      Sid Meier's Pirates!                              41.8      
4700      Total War: MEDIEVAL II - Definitive Edition       70.4      
```
`samuel games list --sort-by {0, 1, 2}` where {0, 1, 2} is the index of the header to sort by

`samuel games list --sort-by 2`
```
id        name                                              playtime (h)
320       Half-Life 2: Deathmatch                           0.5   
3920      Sid Meier's Pirates!                              41.8  
240       Counter-Strike: Source                            64.0 
4700      Total War: MEDIEVAL II - Definitive Edition       70.4    
```

`samuel achievements list GAME_ID`

`samuel achievements list --by-game-name "Baldur's Gate 3"`
```
got   id                  name                          description                                  
✅    BG3_Quest01         Descent From Avernus          Take control of the nautiloid and escape t...
✅    BG3_Quest02         The Plot Thickens             Leave Act I - for somewhere altogether dar...
✅    BG3_Quest03         The City Awaits               Leave Act II - Baldur's Gate is just over ...
✅    BG3_Quest04         All's Well That Ends Well     Finish the game (with a heartfelt 'thank y...
```

`samuel achievements list 489830`
```
got   id                  name                          description                                  
❌    NEW_ACHIEVEMENT_10_0Blood Oath                    Become a member of the Circle                
❌    NEW_ACHIEVEMENT_11_0Glory of the Dead             Complete "Glory of the Dead"                 
✅    NEW_ACHIEVEMENT_12_0Gatekeeper                    Join the College of Winterhold               
✅    NEW_ACHIEVEMENT_13_0Revealing the Unseen          Complete "Revealing the Unseen"       
```

`samuel achievements trigger {GAME_ID} {ACHIVEMENT_ID}`

`samuel achievements trigger 489830 NEW_ACHIEVEMENT_10_0`

`samuel achievements clear 489830 NEW_ACHIEVEMENT_10_0`