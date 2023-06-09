For this program we are looking to collect some key information.

 - Hostname
 - Serial Number
 - Make
 - Model
 - Category

 The way that we will gather this information will be different for every system. Below are the various methods that we may use to get that information.


## Windows



## Linux

### Hardware Information

On Linux we want to run the following command 

`sudo dmidecode -t system`

The above command will produce the following:

```text
# dmidecode 3.4
Getting SMBIOS data from sysfs.
SMBIOS 2.8 present.

Handle 0x0001, DMI type 1, 27 bytes
System Information
        Manufacturer: Micro-Star International Co., Ltd.
        Product Name: US Desktop Aegis ZS
        Version: 1.0
        Serial Number: MSBZ01KAS0106620
        UUID: abea5a55-6364-a212-a595-2cf05d99283b
        Wake-up Type: Power Switch
        SKU Number: Desktop
        Family: Desktop

Handle 0x000C, DMI type 12, 5 bytes
System Configuration Options
        Option 1: To be filled by O.E.M.

Handle 0x000D, DMI type 32, 20 bytes
System Boot Information
        Status: No errors detected
```

To get individual information like the Serial Number we can run the following command.

`sudo dmidecode -t system | grep "Serial Number"`

The above command will produce the following output

```text
Serial Number: MSBZ01KAS0106620
```

From there we will just need to do a little bit of string manipulation.

For us, dmidecode only suffices the following categories.

 - Make (Manufacturer)
 - Model (Product Name)
 - Serial Number
 - Category (Family)

From there the last thing we will need to get from the computer is the hostname.

### Hostname

To get the hostname, you will want to run the following command `hostname`. Hostname will simply spitout the devices hostname just like that and your good to go.

## Macos