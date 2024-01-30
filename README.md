![Workflow Status](https://github.com/ofgrenudo/tattoo/actions/workflows/ci.yml/badge.svg)

# Tattoo

Tattoo is a versatile tool designed to streamline the process of gathering essential hardware details from your computer and transmitting them to a designated web server. Whether you're a system administrator looking to efficiently manage a fleet of devices or an individual seeking a comprehensive overview of your machine's specifications, this system provides an intuitive and robust solution.

![image](https://github.com/ofgrenudo/tattoo/assets/117940901/a7024807-850d-47e2-89ca-6edc37242c07)

## Table of Contents

- Introduction
- Features
- Getting Started
- Installation
- Usage
- Future Enhancements
- Contributing
- license

## Introduction

Modern IT operations demand efficient and accurate methods of maintaining device inventory. The Tattoo addresses this need by offering an application that seamlessly collects pertinent system information such as serial numbers, asset tags, make, and model. This information is then securely transmitted to a designated web server, enabling centralized tracking and management.

## Features

**Comprehensive Data Collection**: The client application compiles a wealth of hardware-related data, including serial numbers, asset tags, make, and model, providing a comprehensive snapshot of each device's specifications.

**Ease of Deployment**: Installing the application is a straightforward process, allowing users to quickly integrate the system into their existing infrastructure.

**Registry Storage**: Device data is stored securely in the Windows registry, ensuring data integrity and persistence across device reboots.

## Example Usage

The below is some sample usage of when you run the program using the `--help` key.

```text
Automatically collect device information on run, and insert it into the registry of the device.

Usage: tattoo.exe [OPTIONS]

Options:
  -a, --all                            Returns all device information.
  -t, --asset-tag                      Returns the device asset tag.
  -T, --set-asset-tag <SET_ASSET_TAG>  Inserts the device asset tag provided. [default: None]
  -M, --manufacturer                   Returns the device manufacturer.
  -m, --model                          Returns the device model.
  -n, --serial-number                  Returns the device serial number.
  -s, --status                         Returns the device status.
  -S, --set-status <SET_STATUS>        Inserts the device status with the string provided. [default: None]
  -u, --update                         Inserts all device information into the registry.
  -h, --help                           Print help
  -V, --version                        Print version
```

## Getting Started

To begin using the Tattoo, follow these simple steps:

## Installation

**Download the Client Application**: Obtain the latest version from [https://github.com/ofgrenudo/tattoo/releases](https://github.com/ofgrenudo/tattoo/releases).

From there you can run the application locally, by clicking on it and **running it as administrator**, or place the executable somewhere in your path. The recommended installation, if you intend to use this consistently, is to create a folder in `C:\Program Files` called `tattoo` and placing the executable there. Then modifying your path to include that folder.

## Future Enhancements

We have ambitious plans to enhance the Tattoo in the future:

- [x] **Graphical User Interface (GUI)**: We're working on developing a user-friendly GUI to complement the existing command-line interface, making the application even more accessible.

- [ ] **Customizable Data Gathering**: We're exploring options for users to customize the data collected based on their specific requirements.

## Contributing

We welcome contributions from the community! Whether you're a developer, designer, or just have great ideas, your input is valuable. To begin making contributions you will need to do the following

**Fork the Repository**: Forking the repository is important because this is how you will make your contributions independent of the master code base.

**Develop your changes**: Develop your changes, and commit them to a feature branch on your local codebase.

**Open a Pull Request**: Open a pull request with a detailed comment on the things that you changed, why and what features you have added. Additionally, you can include screenshots with any visual changes you may have made.

**Wait and respond to any comments**

Thank you for your contribution!
