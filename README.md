![Workflow Status](https://github.com/ofgrenudo/tattoo/actions/workflows/rust.yml/badge.svg)

# Tattoo

Tattoo is a program designed to automatically collect device information on run, and insert it into the registry of the device. This information is intended for archival purposes and will remain there for later inspection. Some of the bennifits of storing information in the registry is that it provides a static and proctected way to maintain information like,

- The day the computer was deployed.
- The task sequence used when you deployed the computer.
- The asset tag assigned.
- The device name when deployed.
- The serial number of the device.
- The device model.
- The device make.

## Requirements

- Windows >=10, or Windows Server 2016.
- PowerShell 5.1 or later
