# RCheckAV

The provided code snippet is designed to identify antivirus software running in the Windows system processes. It quickly detects the presence of antivirus software, preparing for subsequent communication channel setup


## INFO

First, use the tasklist command to view the processes running on the Windows server, then copy the results into a file named "tasklist.txt". After that, iterate through the "tasklist.txt" file to extract the process names, and then compare these process names with the ones listed in a file named "antivirus_identification.txt". If there is a match, it indicates that the corresponding antivirus software is present on the server.


## method

```bash
RCheckAV.exe
```


