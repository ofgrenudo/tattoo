If your app produces `error: process didn't exit successfully: `target\debug\tattoo.exe` (exit code: 0xc0000139, STATUS_ENTRYPOINT_NOT_FOUND)`

It is because GetWindowSubclass is a function present in Common Controls 6.0 and above. The OS inbox version is stuck at 5.x for backwards compatibility reasons. Your app will need to embed/deploy the appropriate manfiest to load in 6.0+.

As a quick fix, create a file on disk next to yourapp.exe, name it yourapp.exe.manifest, and put this inside:

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<assemblyIdentity
    version="1.0.0.0"
    processorArchitecture="*"
    name="app"
    type="win32"
/>
<dependency>
    <dependentAssembly>
        <assemblyIdentity
            type="win32"
            name="Microsoft.Windows.Common-Controls"
            version="6.0.0.0"
            processorArchitecture="*"
            publicKeyToken="6595b64144ccf1df"
            language="*"
        />
    </dependentAssembly>
</dependency>
</assembly>
```