Do not modify contents inside this folder!

This folder contains the `spritec.node` file that is generated by `neon`.
Folder contents are autogenerated by `npm run copy`.

To use, simply `require` the path to this folder.

We copy the file here instead of listing it as a dependency because
`electron-builder` includes unneccessary rust build files when building the app.
`files` config did not seem to work, so this is used as the stopgap.
