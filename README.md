# libeva (0.1.0)
*libeva* is an open-source cross-platform libusb-based implementation of AFP imaging's *sar3kldr.sys* (`USB\VID_0C2A&PID_4000`) and *sar3kusb.sys* (`USB\VID_0C2A&PID_4001`) drivers for windows

# linux
libeva depends on *libusb-1.0*

##### debian/ubuntu
`sudo apt-get -y install libusb-1.0-0`
##### fedora/centos/redhat
`sudo yum install libusb` or `sudo yum install libusbx`
##### arch linux
`sudo pacman -S libusb`

# windows
#### 1) uninstall existing drivers
if present, uninstall any existing drivers for the `USB\VID_0C2A&PID_4000` and `USB\VID_0C2A&PID_4001` devices
#### 2) download *zadig*
download *zadig* from https://zadig.akeo.ie
#### 3) install *WinUSB* or *libusb-win32* drivers
> note: unplug your device before installing WinUSB/libusb drivers

open *zadig* and select *Device* > *Create New Device* menu and install WinUSB or libusb-win32 drivers
###### install driver for `USB\VID_0C2A&PID_4000`
![EVA Loader](https://i.imgur.com/TR9VwPw.png)
###### install driver for `USB\VID_0C2A&PID_4001`
![EVA Driver](https://i.imgur.com/ifbT1IQ.png)
#### 4) try `acquire_tiff.exe` example