```cs
   ____     _                  _____    _____  
U | __")u  |"|        ___     |_ " _|  |"_  /u 
 \|  _ \/U | | u     |_"_|      | |    U / //  
  | |_) | \| |/__     | |      /| |\   \/ /_   
  |____/   |_____|  U/| |\u   u |_|U   /____|  
 _|| \\_   //  \\.-,_|___|_,-._// \\_  _//<<,- 
(__) (__) (_")("_)\_)-' '-(_/(__) (__)(__) (_/ 
```
## What is Blitz?
Blitz is a blazingly-fast  open-source command line tool written in Rust that detects whether moronic players are in your **RISK: Global Domination** lobbies.
> [!IMPORTANT]  
> Blitz is cross-platform but the initial 1.0.0 release only supports **Windows**. Future releases will supports all Operating Systems.

> [!IMPORTANT]  
> Blitz is early in development and only supports monitors with dimensions of **1920x1080** pixels. Support for monitors of all sizes is actively being developed in [`src/detector.rs`](https://github.com/Hakxsorus/blitz/blob/master/src/detector.rs#L176).

## How does Blitz work?
Blitz is like the Sherlock Holmes of RISK. It snaps a quick screenshot of your pre-game lobby, then employs Optical Character Recognition to decipher the usernames of all the players present. If it spots any shady characters lurking about—ones you've already marked on your blacklist—it'll give you the heads-up that trouble might be brewing.

### An Example Blacklist
```json
{
  "morons": [
    {
      "username": "PokemonMaster1248",
      "reason": "Copy and paste the { } block to add more entries"
    },
    {
      "username": "Theo The Blade",
      "reason": "Don't forget the comma at the end of the block."
    },
    {
      "username": "Laura Burn",
      "reason": "Risk players seriously need a shower."
    }
  ]
}
```
### An Example Pre-Game Lobby
![image](https://i.imgur.com/oSX8Yz5.png)

### The Output of Blitz's Moron Detection
![image](https://i.imgur.com/meoHCT8.png)


## How do I use Blitz?
> [!IMPORTANT]  
> These instructions are for the initial 1.0.0 release which only supports **Windows** and monitors with dimensions of **1920x1080** pixels.

Download the latest `blitz.exe` version from [releases](https://github.com/Hakxsorus/blitz/releases) and move it anywhere memorable in your filestyle. You can optionally set it as a PATH Environment Variable, which I recommend.

From your terminal, you can now run any of the following commands.
### Edit Your Blacklist
```
blitz.exe edit
```
This command opens your blacklist in your default text editor for JSON files. Once open, you can add and remove morons as you wish. Remember to save the file once you're done editing. RISK does not have to be open.

![GIF](https://i.imgur.com/dBia5Kf.gif)
### Scan Your Risk Lobby For Morons
```
blitz.exe scan
```
This command scans your pre-game lobby for any morons who are present. RISK has to be open in the pre-game lobby.

![GIF](https://cdn.discordapp.com/attachments/872229558257078282/1231760357631070218/Animation3.gif?ex=6638213a&is=6625ac3a&hm=c944b885ab6d033e48835dd81cb39682fb809af785d02ac7951419169ee09c2b&)
## Blitz Isn't Working Properly
Please create an issue and I will support you.
