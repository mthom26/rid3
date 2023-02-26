## Tutorial

### Starting rid3

When you run `rid3` with no arguments it will start in the current working directory. You may want to show the logs widget by pressing the `l` key. This will display some helpful feedback when you interact with the app and show any warnings or errors.

Press the `2` key to switch to the file browser window. From here you can move up and down the list with the `up` and `down` keys, enter directories with the `enter` key and add the highlighted file with the `s` key. Enter a directory that has some mp3 files and add some of them. You can add all files in the window with the `a` key (this does not recursively add files from directories).

### Viewing active files

After adding a few files press `1` to switch to the main screen. This is where you will spend most of your time as all the frame editing is done here. On the left side of the screen is the active files list, you should see the files you added in the previous step here. The `up` and `down` keys will select the next/previous file and the right side of the screen contains the filename and id3 frames for the highlighted file. If your files already have metadata you may already see some frames in this list.

The `s` key will select the currently highlighted file. If you highlight a different file you will see that the previous file is still selected. Pressing `d` will remove the highlighted file, and pressing `c` will remove all files. For now press `a` to select all files.

### Adding frames

Press the `3` key to switch to the frames screen. On the left you will see a list of frames and on the right information about the currently highlighted frame. Using `up` and `down` highlight the `title` frame and press `a` to add the frame to the selected files from the active files list. Highlight the `artist` frame and add it aswell.

### Editing frames

Switch back to the active files window with the `1` key and for now deselect all files by pressing `a` (if all the files are already selected `a` will deselect all files). Now highlight one of the files and press `tab` to switch the focus to the details list on the right. The first item will always be the filename. Pressing `d` will remove the highlighted frame from the selected files.

Highlight the `title` frame added earlier and press `enter` to spawn a popup to edit the frame. This popup only has one field to edit and the input box at the bottom is already selected so you can start typing the name of the track immediately. Press `enter` again to save your changes and close the popup. The highlighted frame should be updated with the new data. Repeat this for the rest of the files.

Press `tab` to switch back to the active files and select all the files. Now switch back to the details (`tab`) and highlight the `artist` frame. Press `enter`, type in the artist name and press `enter` again to save the changes. Now switch back to the active files and highlight each file. You will notice that the artist frame has updated on **all** selected files. A lot of metadata will be the same for tracks from the same album, you don't have to input duplicate data individually.

### Saving changes

When you are finished adding and editing frames press the `w` key to save changes to the files. The log widget at the bottom of the screen (`l` to toggle) will display a message telling you when the changes have saved. Now you can keep editing or close the app using the `q` key.

