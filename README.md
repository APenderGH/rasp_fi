# rasp_fi
Performs a password spray attack on any raspberry pi's in an ip range.
 - Scans an ip range for raspberry pi's and tests an SSH connection using the default credentials. 
 - Uses
	 - DEFAULT_USERNAME: pi
	 - DEFAULT_PASSWORD: raspberry
### A QUT demonstration
This program was written for IFB102 and is to be used as a demonstration of a basic password spray attack.

```
USAGE:
    rasp_fi.exe [OPTIONS] --network-range <NETWORK_RANGE>

OPTIONS:
    -d, --demo <DEMO>
            Specify an IP to target. Other IP's will be scanned but not tested against

    -h, --help
            Print help information

    -n, --network-range <NETWORK_RANGE>
            Specify an ip range to scan. E.g. 192.0.0-255.0-255 or 192.0.0.0-255. THIS WILL PASSWORD
            SPRAY THIS NETWORK RANGE, USE -d FOR DEMONSTRATION
```

Without the `-d` parameter, you will spray the network you specified. It is recommended to always run this with `-d` unless you own all the raspberry pi's on the network.

### Dependencies
| Windows | Linux |
--------------|-------------
|PuTTY https://www.putty.org/       |sshpass  https://gist.github.com/arunoda/7790979   |
|Nmap  https://nmap.org/download       |Nmap  https://nmap.org/download      |
