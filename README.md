# skribbliobot
drawing bot for skribblio written in rust

![Screenshot](screenshot.png)
  
# Running

- Download the latest `skribbliobot` from the github releases
- Install `xclip`
- Start the bot
- Take a screenshot of your game screen and paste it into gimp
- Hover your cursor over the top left of the drawing canvas and note down XY (bottom of gimp)
- Select the drawing canvas with box select and note down the size of it (bottom of gimp)
- Repeat the last two steps for only the white color of the color palette
- Click the Save Settings button.
- Search for image on the internet and copy it into your clipboard
- You can now run the drawer

# For now Windows is not supported because of two reasons:

- I don't want to port the image from clipboard function because clipboard transparency on windows is black
- I think it will hurt the game because this will be an easily obtainable and fast drawer for the majority of desktop users
