![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Workflow Status](https://github.com/ofgrenudo/tattoo/actions/workflows/rust.yml/badge.svg)
# Tattoo

The Tattoo Update Script is a PowerShell script designed to collect device information on run, and insert it into the Registry of the device. This information is for archival purposes and will remain there as a level of forensics, as well as to allow for managment / exporting to a spread sheet or database later down the road. Currently this script will only gather device information and tattoo it. You will need another script to accompany it to be able to upload that information into whatever asset inventory system you are using.
Prerequisites

- Windows >=10, or Windows Server 2016 or later, GNU/Linux, Macos
- PowerShell 5.1 or later
- MECM Task Sequence Variable OSDAssetTag is already set. (If your unsure where to put this variable, it should be in the same place as your OSDComputerName) variable.

## Information Collected

- Deployment Date
- Task Sequence Name
- Task Sequence ID
- Asset Tag
- Device Name
- Serial Number
- Device Model
- Device Make
