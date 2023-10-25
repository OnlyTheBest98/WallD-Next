# WallD Next

This projects aims to provide a wallpaper service similar to Wallclaimer.

We want you to be able to send any images privately without the server
owner to know the content. Only the receiving end can decrypt the images.

For that goal we publish the source code for the frontend and backend
so you don't need to trust us and can verify it yourself.


## Implementation

We plan on using Rust with Tokio for the server backend and Java for the Android App.
The app for the PC is no priority but could be written in Rust as well.
The server will be a Linux server and may use a nginx HTTPS reverse proxy
for SSL encrypted network traffic (could change).


## Technical Details

Every user generates a key pair consisting of a public and private key.
The public key is uploaded to the server and used to identify the user.
If you send a wallpaper to your friends the image gets encrypted using a
symmetric encryption algorithm and a random key on the client and then uploaded to the server.
Then the key is encrypted using the public key of each receiving user such that only the
owner of the corresponding private key can decrypt it again. Those encrypted keys are then send
to the server as well.

## Installation

TODO: there will be an Android App and maybe a Desktop App
