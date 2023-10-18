### Multy executable project 

To call the speciffic executable 

```rust
cargo run --bin securitycenter
cargo run --bin executable2

```


To build the speciffic executable
```rust
cargo build --bin executable1
```


**NOTE** You can then run the built executable directly from the target/debug directory
> ./target/debug/executable1



Windows Management Instrumentation (WMI) 

>uses namespaces to organize and categorize the information stored within³. Each namespace represents a distinct area of the managed systems, such as software, networking, or hardware³. 


Here are some of the namespaces created by Configuration Manager and a brief description of each¹:

- `root\\ccm`: The root namespace for Configuration Manager.
- `root\\ccm\\CCMPasswordSettings`: Contains classes related to password settings.
- `root\\ccm\\CIModels`: Contains classes related to Configuration Items (CIs).
- `root\\ccm\\CIStateStore`: Contains classes related to the state of Configuration Items.
- `root\\ccm\\CIStore`: Contains classes related to the storage of Configuration Items.
- `root\\ccm\\CITasks`: Contains classes related to tasks associated with Configuration Items.
- `root\\ccm\\ClientSDK`: Contains classes related to the Client Software Development Kit (SDK).
- `root\\ccm\\ContentTransferManager`: Contains classes related to the transfer of content.
- `root\\ccm\\DataTransferService`: Contains classes related to data transfer services.
- `root\\ccm\\dcm`: Contains classes related to Desired Configuration Management (DCM).
- `root\\ccm\\DCMAgent`: Contains classes related to the DCM Agent.
- `root\\ccm\\evaltest`: Contains classes related to evaluation tests.
- `root\\ccm\\Events`: Contains classes related to events.
- `root\\ccm\\InvAgt`: Contains classes related to inventory agents.
- `root\\ccm\\LocationServices`: Contains classes related to location services.
- `root\\ccm\\Messaging`: Contains classes related to messaging.
- `root\\ccm\\NetworkConfig`: Contains classes related to network configuration.
- `root\\ccm\\PeerDPAgent`: Contains classes related to peer distribution point agents.
- `root\\ccm\\Policy`: Contains classes related to policies.
- `root\\ccm\\PowerManagementAgent`: Contains classes related to power management agents.
- `root\\ccm\\RebootManagement`: Contains classes related to reboot management.
- `root\\ccm\\ScanAgent`: Contains classes related to scan agents.
- `root\\ccm\\Scheduler`: Contains classes related to scheduling.
- `root\\ccm\\SMSNapAgent`: Contains classes related to Network Access Protection (NAP) agents.
- `root\\ccm\\SoftMgmtAgent`: Contains classes related to software management agents.
- `root\\ccm\\SoftwareMeteringAgent`: Contains classes related to software metering agents.
- `root\\ccm\\SoftwareUpdates`: Contains classes related to software updates.
- `root\\ccm\\StateMsg`: Contains classes related to state messages.
- `root\\ccm\\VulnerabilityAssessment`: Contains classes related to vulnerability assessments.
- `root\\ccm\\XmlStore`: Contains XML store-related information.

> Others 
 - `ROOT\\securitycenter2` Antivirus used on the system.


Please note that each site might have additional namespaces depending on the specific site settings, the inventory that is tracked, and so forth¹.

Source: Conversation with Bing, 10/17/2023
(1) What Is A Windows Management Instrumentation (WMI) Database?. 
https://cellularnews.com/definitions/what-is-a-windows-management-instrumentation-wmi-database/
https://learn.microsoft.com/en-us/windows/win32/wmisdk/about-wmi

(2) WMI namespaces and classes for reports - Configuration Manager. 
https://learn.microsoft.com/en-us/mem/configmgr/develop/core/understand/sqlviews/wmi-namespaces-classes-configuration-manager-reports.

(3) WMI Classes - Win32 apps | Microsoft Learn. https://learn.microsoft.com/en-us/windows/win32/wmisdk/wmi-classes.

(4) about WMI - PowerShell | Microsoft Learn. 
https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_wmi?view=powershell-5.1.



#### Cool repo used as an inspiration 
https://github.com/trickster0/OffensiveRust