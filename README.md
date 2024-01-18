# mxw - Model (O/O-/D/D-) Wireless
Cross platform CLI for configuring Glorious wireless mice.

See the original project and it's README at [korkje/mow](https://github.com/korkje/mow).

This fork uses updated dependencies, adds support for more devices (see below) and fixes panics when changing the polling rate and lift-off distance.

## Supported Devices
- [x] Model O
    - [ ] Tested
- [x] Model D
    - [x] Tested
- [x] Model O-
    - [ ] Tested
- [ ] Model D-
    - [ ] Tested

Please, if you have a device that isn't supported, submit a pull request to add it! You can follow [this guide](https://kb.synology.com/en-ph/DSM/tutorial/How_do_I_check_the_PID_VID_of_my_USB_device) to figure out how to find the relevant info you need.

I have tried to search for other product IDs to add to this project, but online databases only hold so much.

Glorious also unfortunately could not provide me with these after I submitted a support request.

## Usage with nix 
You can either import this in your flake or just run 

```nix 
    nix run github:dxbednarczyk/mxw -- [INSERT_FLAGS_HERE]
`

