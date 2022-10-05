# pocket-arcade-grabber
Grabs your files you need for Analgoue Pocket Arcade Cores

Like everyone else, I take the MAME files I legally own, convert them to the pocket format then put them on a fileserver on my local network.
This utility will, when given the pocket's root folder as the 1st argument:
- Will read the `file_host` _either_ from the 2nd arg OR from the `arcade_grabber.json` file (which it'll create if it's not there when it looks for it)
- Have a look around any `.json` files in the Assets folder for cores, see if it spots any data_slots with filenames
- Try to find those files on _your_ fileserver which you've supplied as the `file_host`
- It'll then have a look in the core's `data.json` too & try to download any named files in the same way

Note this utility is purely a **convience** method for loading **your own** ROMs which you **legally** own on to the Pocket.

## Known issues
- It's not very smart, it'll try to download things which aren't needed - but these downloads will fail & it handles that

## FAQs

#### GUI?
No. Others are welcome to integrate this script into _their_ GUIs though, with proper credit being given etc.

#### What should I set `file_host` to?
It should be set to your local file server where you have all your legally owned & converted MAME ROMs, e.g. `http://192.168.1.38/my-legitimate-pocket-roms` or `http://localhost:8080/real-roms-i-extracted-myself`
