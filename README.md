# pocket-arcade-grabber
Grabs your files you need for Analgoue Pocket Arcade Cores

Like everyone else, I take the MAME files I legally own, convert them to the pocket format then put them on a fileserver on my local network.
This utility will, when given the pocket's root folder as the 1st argument:
- Read the `file_host` _either_ from the 2nd arg OR from the `arcade_grabber.json` file (which it'll create if it's not there when it looks for it)
- Have a look around any `.json` files in the Assets folder for cores, see if it spots any `data_slot`s with filenames
- Try to find those files on _your_ fileserver which _you've_ supplied as the `file_host`
- It'll then have a look in the core's `data.json` too & try to download any named files in the same way

Note this utility is purely a **convience** method for loading **your own** ROMs which you **legally** own on to the Pocket.

It should be run after installing a new / updated Arcade core on the Pocket.

## To run
- Download the binary from the releases page for your OS, in it in a terminal / command line and supply the path to the pocket root folder as the 1st argument.

- Optionally, you can supply 2nd argument to say where to look for the ROMs (that you own)

## Known issues
- It's not very smart, it'll try to download things which aren't needed - but these downloads will fail & it handles that -- ~~but it can take a while looping through _all_ of the Neo Geo json files, for example.~~ now ignores neogeo
- In the future, once the arcade / console etc categories are rolled out into more cores I'll probably have it read that & only focus on arcade cores.

## FAQs

#### GUI?
No. Others are welcome to integrate this script into _their_ GUIs though, with proper credit being given etc.

#### What should I set `file_host` to?
It should be set to your local file server where you have all your legally owned & converted MAME ROMs, e.g. `http://192.168.1.38/my-legitimate-pocket-roms` or `http://localhost:8080/real-roms-i-extracted-myself`
