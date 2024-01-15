//! Enums representing the tactics and techniques from [Mitre ATT&CK](https://attack.mitre.org/)
use crate::InvalidArgumentError;
use crate::shared::AttackTechnique;
/// An entry in the [Mitre ATT&CK table](https://attack.mitre.org/)
pub enum Tactic {
    /// [Credential Access](https://attack.mitre.org/tactics/TA0006)
    ///
    /// The adversary is trying to steal account names and passwords.
    CredentialAccess(CredentialAccess),

    /// [Execution](https://attack.mitre.org/tactics/TA0002)
    ///
    /// The adversary is trying to run malicious code.
    Execution(Execution),

    /// [Impact](https://attack.mitre.org/tactics/TA0040)
    ///
    /// The adversary is trying to manipulate, interrupt, or destroy your systems and data.
    Impact(Impact),

    /// [Persistence](https://attack.mitre.org/tactics/TA0003)
    ///
    /// The adversary is trying to maintain their foothold.
    Persistence(Persistence),

    /// [Privilege Escalation](https://attack.mitre.org/tactics/TA0004)
    ///
    /// The adversary is trying to gain higher-level permissions.
    PrivilegeEscalation(PrivilegeEscalation),

    /// [Lateral Movement](https://attack.mitre.org/tactics/TA0008)
    ///
    /// The adversary is trying to move through your environment.
    LateralMovement(LateralMovement),

    /// [Defense Evasion](https://attack.mitre.org/tactics/TA0005)
    ///
    /// The adversary is trying to avoid being detected.
    DefenseEvasion(DefenseEvasion),

    /// [Exfiltration](https://attack.mitre.org/tactics/TA0010)
    ///
    /// The adversary is trying to steal data.
    Exfiltration(Exfiltration),

    /// [Discovery](https://attack.mitre.org/tactics/TA0007)
    ///
    /// The adversary is trying to figure out your environment.
    Discovery(Discovery),

    /// [Collection](https://attack.mitre.org/tactics/TA0009)
    ///
    /// The adversary is trying to gather data of interest to their goal.
    Collection(Collection),

    /// [Resource Development](https://attack.mitre.org/tactics/TA0042)
    ///
    /// The adversary is trying to establish resources they can use to support operations.
    ResourceDevelopment(ResourceDevelopment),

    /// [Reconnaissance](https://attack.mitre.org/tactics/TA0043)
    ///
    /// The adversary is trying to gather information they can use to plan future operations.
    Reconnaissance(Reconnaissance),

    /// [Command and Control](https://attack.mitre.org/tactics/TA0011)
    ///
    /// The adversary is trying to communicate with compromised systems to control them.
    CommandAndControl(CommandAndControl),

    /// [Initial Access](https://attack.mitre.org/tactics/TA0001)
    ///
    /// The adversary is trying to get into your network.
    InitialAccess(InitialAccess),

}
/// [Credential Access](https://attack.mitre.org/tactics/TA0006)'s techniques
pub enum CredentialAccess {
    /// [Adversary-in-the-Middle](https://attack.mitre.org/techniques/T1557)
    ///
    /// Adversaries may attempt to position themselves between two or more networked devices using an adversary-in-the-middle (AiTM) technique to support follow-on behaviors such as [Network Sniffing](https://attack.mitre.org/techniques/T1040), [Transmitted Data Manipulation](https://attack.mitre.org/techniques/T1565/002), or replay attacks ([Exploitation for Credential Access](https://attack.mitre.org/techniques/T1212)). By abusing features of common networking protocols that can determine the flow of network traffic (e.g. ARP, DNS, LLMNR, etc.), adversaries may force a device to communicate through an adversary controlled system so they can collect information or perform additional actions.(Citation: Rapid7 MiTM Basics)
    AdversaryInTheMiddle,

    /// [OS Credential Dumping](https://attack.mitre.org/techniques/T1003)
    ///
    /// Adversaries may attempt to dump credentials to obtain account login and credential material, normally in the form of a hash or a clear text password, from the operating system and software. Credentials can then be used to perform [Lateral Movement](https://attack.mitre.org/tactics/TA0008) and access restricted information.
    OsCredentialDumping,

    /// [Steal Web Session Cookie](https://attack.mitre.org/techniques/T1539)
    ///
    /// An adversary may steal web application or service session cookies and use them to gain access to web applications or Internet services as an authenticated user without needing credentials. Web applications and services often use session cookies as an authentication token after a user has authenticated to a website.
    StealWebSessionCookie,

    /// [Network Sniffing](https://attack.mitre.org/techniques/T1040)
    ///
    /// Adversaries may sniff network traffic to capture information about an environment, including authentication material passed over the network. Network sniffing refers to using the network interface on a system to monitor or capture information sent over a wired or wireless connection. An adversary may place a network interface into promiscuous mode to passively access data in transit over the network, or use span ports to capture a larger amount of data.
    NetworkSniffing,

    /// [Steal or Forge Kerberos Tickets](https://attack.mitre.org/techniques/T1558)
    ///
    /// Adversaries may attempt to subvert Kerberos authentication by stealing or forging Kerberos tickets to enable [Pass the Ticket](https://attack.mitre.org/techniques/T1550/003). Kerberos is an authentication protocol widely used in modern Windows domain environments. In Kerberos environments, referred to as “realms”, there are three basic participants: client, service, and Key Distribution Center (KDC).(Citation: ADSecurity Kerberos Ring Decoder) Clients request access to a service and through the exchange of Kerberos tickets, originating from KDC, they are granted access after having successfully authenticated. The KDC is responsible for both authentication and ticket granting.  Adversaries may attempt to abuse Kerberos by stealing tickets or forging tickets to enable unauthorized access.
    StealOrForgeKerberosTickets,

    /// [Credentials from Password Stores](https://attack.mitre.org/techniques/T1555)
    ///
    /// Adversaries may search for common password storage locations to obtain user credentials. Passwords are stored in several places on a system, depending on the operating system or application holding the credentials. There are also specific applications and services that store passwords to make them easier for users to manage and maintain, such as password managers and cloud secrets vaults. Once credentials are obtained, they can be used to perform lateral movement and access restricted information.
    CredentialsFromPasswordStores,

    /// [Unsecured Credentials](https://attack.mitre.org/techniques/T1552)
    ///
    /// Adversaries may search compromised systems to find and obtain insecurely stored credentials. These credentials can be stored and/or misplaced in many locations on a system, including plaintext files (e.g. [Bash History](https://attack.mitre.org/techniques/T1552/003)), operating system or application-specific repositories (e.g. [Credentials in Registry](https://attack.mitre.org/techniques/T1552/002)), or other specialized files/artifacts (e.g. [Private Keys](https://attack.mitre.org/techniques/T1552/004)).
    UnsecuredCredentials,

    /// [Steal or Forge Authentication Certificates](https://attack.mitre.org/techniques/T1649)
    ///
    /// Adversaries may steal or forge certificates used for authentication to access remote systems or resources. Digital certificates are often used to sign and encrypt messages and/or files. Certificates are also used as authentication material. For example, Azure AD device certificates and Active Directory Certificate Services (AD CS) certificates bind to an identity and can be used as credentials for domain accounts.(Citation: O365 Blog Azure AD Device IDs)(Citation: Microsoft AD CS Overview)
    StealOrForgeAuthenticationCertificates,

    /// [Steal Application Access Token](https://attack.mitre.org/techniques/T1528)
    ///
    /// Adversaries can steal application access tokens as a means of acquiring credentials to access remote systems and resources.
    StealApplicationAccessToken,

    /// [Forge Web Credentials](https://attack.mitre.org/techniques/T1606)
    ///
    /// Adversaries may forge credential materials that can be used to gain access to web applications or Internet services. Web applications and services (hosted in cloud SaaS environments or on-premise servers) often use session cookies, tokens, or other materials to authenticate and authorize user access.
    ForgeWebCredentials,

    /// [Multi-Factor Authentication Request Generation](https://attack.mitre.org/techniques/T1621)
    ///
    /// Adversaries may attempt to bypass multi-factor authentication (MFA) mechanisms and gain access to accounts by generating MFA requests sent to users.
    MultiFactorAuthenticationRequestGeneration,

    /// [Exploitation for Credential Access](https://attack.mitre.org/techniques/T1212)
    ///
    /// Adversaries may exploit software vulnerabilities in an attempt to collect credentials. Exploitation of a software vulnerability occurs when an adversary takes advantage of a programming error in a program, service, or within the operating system software or kernel itself to execute adversary-controlled code. 
    ExploitationForCredentialAccess,

    /// [Brute Force](https://attack.mitre.org/techniques/T1110)
    ///
    /// Adversaries may use brute force techniques to gain access to accounts when passwords are unknown or when password hashes are obtained. Without knowledge of the password for an account or set of accounts, an adversary may systematically guess the password using a repetitive or iterative mechanism. Brute forcing passwords can take place via interaction with a service that will check the validity of those credentials or offline against previously acquired credential data, such as password hashes.
    BruteForce,

    /// [Forced Authentication](https://attack.mitre.org/techniques/T1187)
    ///
    /// Adversaries may gather credential material by invoking or forcing a user to automatically provide authentication information through a mechanism in which they can intercept.
    ForcedAuthentication,

    /// [Input Capture](https://attack.mitre.org/techniques/T1056)
    ///
    /// Adversaries may use methods of capturing user input to obtain credentials or collect information. During normal system usage, users often provide credentials to various different locations, such as login pages/portals or system dialog boxes. Input capture mechanisms may be transparent to the user (e.g. [Credential API Hooking](https://attack.mitre.org/techniques/T1056/004)) or rely on deceiving the user into providing input into what they believe to be a genuine service (e.g. [Web Portal Capture](https://attack.mitre.org/techniques/T1056/003)).
    InputCapture,

    /// [Multi-Factor Authentication Interception](https://attack.mitre.org/techniques/T1111)
    ///
    /// Adversaries may target multi-factor authentication (MFA) mechanisms, (i.e., smart cards, token generators, etc.) to gain access to credentials that can be used to access systems, services, and network resources. Use of MFA is recommended and provides a higher level of security than usernames and passwords alone, but organizations should be aware of techniques that could be used to intercept and bypass these security mechanisms. 
    MultiFactorAuthenticationInterception,

    /// [Modify Authentication Process](https://attack.mitre.org/techniques/T1556)
    ///
    /// Adversaries may modify authentication mechanisms and processes to access user credentials or enable otherwise unwarranted access to accounts. The authentication process is handled by mechanisms, such as the Local Security Authentication Server (LSASS) process and the Security Accounts Manager (SAM) on Windows, pluggable authentication modules (PAM) on Unix-based systems, and authorization plugins on MacOS systems, responsible for gathering, storing, and validating credentials. By modifying an authentication process, an adversary may be able to authenticate to a service or system without using [Valid Accounts](https://attack.mitre.org/techniques/T1078).
    ModifyAuthenticationProcess,

}
/// [Execution](https://attack.mitre.org/tactics/TA0002)'s techniques
pub enum Execution {
    /// [Windows Management Instrumentation](https://attack.mitre.org/techniques/T1047)
    ///
    /// Adversaries may abuse Windows Management Instrumentation (WMI) to execute malicious commands and payloads. WMI is an administration feature that provides a uniform environment to access Windows system components. The WMI service enables both local and remote access, though the latter is facilitated by [Remote Services](https://attack.mitre.org/techniques/T1021) such as [Distributed Component Object Model](https://attack.mitre.org/techniques/T1021/003) (DCOM) and [Windows Remote Management](https://attack.mitre.org/techniques/T1021/006) (WinRM).(Citation: MSDN WMI) Remote WMI over DCOM operates using port 135, whereas WMI over WinRM operates over port 5985 when using HTTP and 5986 for HTTPS.(Citation: MSDN WMI)(Citation: FireEye WMI 2015)
    WindowsManagementInstrumentation,

    /// [Shared Modules](https://attack.mitre.org/techniques/T1129)
    ///
    /// Adversaries may execute malicious payloads via loading shared modules. Shared modules are executable files that are loaded into processes to provide access to reusable code, such as specific custom functions or invoking OS API functions (i.e., [Native API](https://attack.mitre.org/techniques/T1106)).
    SharedModules,

    /// [Scheduled Task/Job](https://attack.mitre.org/techniques/T1053)
    ///
    /// Adversaries may abuse task scheduling functionality to facilitate initial or recurring execution of malicious code. Utilities exist within all major operating systems to schedule programs or scripts to be executed at a specified date and time. A task can also be scheduled on a remote system, provided the proper authentication is met (ex: RPC and file and printer sharing in Windows environments). Scheduling a task on a remote system typically may require being a member of an admin or otherwise privileged group on the remote system.(Citation: TechNet Task Scheduler Security)
    ScheduledTaskJob,

    /// [Native API](https://attack.mitre.org/techniques/T1106)
    ///
    /// Adversaries may interact with the native OS application programming interface (API) to execute behaviors. Native APIs provide a controlled means of calling low-level OS services within the kernel, such as those involving hardware/devices, memory, and processes.(Citation: NT API Windows)(Citation: Linux Kernel API) These native APIs are leveraged by the OS during system boot (when other system components are not yet initialized) as well as carrying out tasks and requests during routine operations.
    NativeApi,

    /// [Deploy Container](https://attack.mitre.org/techniques/T1610)
    ///
    /// Adversaries may deploy a container into an environment to facilitate execution or evade defenses. In some cases, adversaries may deploy a new container to execute processes associated with a particular image or deployment, such as processes that execute or download malware. In others, an adversary may deploy a new container configured without network rules, user limitations, etc. to bypass existing defenses within the environment.
    DeployContainer,

    /// [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059)
    ///
    /// Adversaries may abuse command and script interpreters to execute commands, scripts, or binaries. These interfaces and languages provide ways of interacting with computer systems and are a common feature across many different platforms. Most systems come with some built-in command-line interface and scripting capabilities, for example, macOS and Linux distributions include some flavor of [Unix Shell](https://attack.mitre.org/techniques/T1059/004) while Windows installations include the [Windows Command Shell](https://attack.mitre.org/techniques/T1059/003) and [PowerShell](https://attack.mitre.org/techniques/T1059/001).
    CommandAndScriptingInterpreter,

    /// [Container Administration Command](https://attack.mitre.org/techniques/T1609)
    ///
    /// Adversaries may abuse a container administration service to execute commands within a container. A container administration service such as the Docker daemon, the Kubernetes API server, or the kubelet may allow remote management of containers within an environment.(Citation: Docker Daemon CLI)(Citation: Kubernetes API)(Citation: Kubernetes Kubelet)
    ContainerAdministrationCommand,

    /// [User Execution](https://attack.mitre.org/techniques/T1204)
    ///
    /// An adversary may rely upon specific actions by a user in order to gain execution. Users may be subjected to social engineering to get them to execute malicious code by, for example, opening a malicious document file or link. These user actions will typically be observed as follow-on behavior from forms of [Phishing](https://attack.mitre.org/techniques/T1566).
    UserExecution,

    /// [Software Deployment Tools](https://attack.mitre.org/techniques/T1072)
    ///
    /// Adversaries may gain access to and use third-party software suites installed within an enterprise network, such as administration, monitoring, and deployment systems, to move laterally through the network. Third-party applications and software deployment systems may be in use in the network environment for administration purposes (e.g., SCCM, HBSS, Altiris, etc.).  
    SoftwareDeploymentTools,

    /// [Inter-Process Communication](https://attack.mitre.org/techniques/T1559)
    ///
    /// Adversaries may abuse inter-process communication (IPC) mechanisms for local code or command execution. IPC is typically used by processes to share data, communicate with each other, or synchronize execution. IPC is also commonly used to avoid situations such as deadlocks, which occurs when processes are stuck in a cyclic waiting pattern. 
    InterProcessCommunication,

    /// [Exploitation for Client Execution](https://attack.mitre.org/techniques/T1203)
    ///
    /// Adversaries may exploit software vulnerabilities in client applications to execute code. Vulnerabilities can exist in software due to unsecure coding practices that can lead to unanticipated behavior. Adversaries can take advantage of certain vulnerabilities through targeted exploitation for the purpose of arbitrary code execution. Oftentimes the most valuable exploits to an offensive toolkit are those that can be used to obtain code execution on a remote system because they can be used to gain access to that system. Users will expect to see files related to the applications they commonly used to do work, so they are a useful target for exploit research and development because of their high utility.
    ExploitationForClientExecution,

    /// [System Services](https://attack.mitre.org/techniques/T1569)
    ///
    /// Adversaries may abuse system services or daemons to execute commands or programs. Adversaries can execute malicious content by interacting with or creating services either locally or remotely. Many services are set to run at boot, which can aid in achieving persistence ([Create or Modify System Process](https://attack.mitre.org/techniques/T1543)), but adversaries can also abuse services for one-time or temporary execution.
    SystemServices,

    /// [Cloud Administration Command](https://attack.mitre.org/techniques/T1651)
    ///
    /// Adversaries may abuse cloud management services to execute commands within virtual machines or hybrid-joined devices. Resources such as AWS Systems Manager, Azure RunCommand, and Runbooks allow users to remotely run scripts in virtual machines by leveraging installed virtual machine agents. Similarly, in Azure AD environments, Microsoft Endpoint Manager allows Global or Intune Administrators to run scripts as SYSTEM on on-premises devices joined to the Azure AD.(Citation: AWS Systems Manager Run Command)(Citation: Microsoft Run Command)(Citation: SpecterOps Lateral Movement from Azure to On-Prem AD 2020)
    CloudAdministrationCommand,

    /// [Serverless Execution](https://attack.mitre.org/techniques/T1648)
    ///
    /// Adversaries may abuse serverless computing, integration, and automation services to execute arbitrary code in cloud environments. Many cloud providers offer a variety of serverless resources, including compute engines, application integration services, and web servers. 
    ServerlessExecution,

}
/// [Impact](https://attack.mitre.org/tactics/TA0040)'s techniques
pub enum Impact {
    /// [Disk Wipe](https://attack.mitre.org/techniques/T1561)
    ///
    /// Adversaries may wipe or corrupt raw disk data on specific systems or in large numbers in a network to interrupt availability to system and network resources. With direct write access to a disk, adversaries may attempt to overwrite portions of disk data. Adversaries may opt to wipe arbitrary portions of disk data and/or wipe disk structures like the master boot record (MBR). A complete wipe of all disk sectors may be attempted.
    DiskWipe,

    /// [Service Stop](https://attack.mitre.org/techniques/T1489)
    ///
    /// Adversaries may stop or disable services on a system to render those services unavailable to legitimate users. Stopping critical services or processes can inhibit or stop response to an incident or aid in the adversary's overall objectives to cause damage to the environment.(Citation: Talos Olympic Destroyer 2018)(Citation: Novetta Blockbuster) 
    ServiceStop,

    /// [Defacement](https://attack.mitre.org/techniques/T1491)
    ///
    /// Adversaries may modify visual content available internally or externally to an enterprise network, thus affecting the integrity of the original content. Reasons for [Defacement](https://attack.mitre.org/techniques/T1491) include delivering messaging, intimidation, or claiming (possibly false) credit for an intrusion. Disturbing or offensive images may be used as a part of [Defacement](https://attack.mitre.org/techniques/T1491) in order to cause user discomfort, or to pressure compliance with accompanying messages. 
    Defacement,

    /// [Financial Theft](https://attack.mitre.org/techniques/T1657)
    ///
    /// Adversaries may steal monetary resources from targets through extortion, social engineering, technical theft, or other methods aimed at their own financial gain at the expense of the availability of these resources for victims. Financial theft is the ultimate objective of several popular campaign types including extortion by ransomware,(Citation: FBI-ransomware) business email compromise (BEC) and fraud,(Citation: FBI-BEC) "pig butchering,"(Citation: wired-pig butchering) bank hacking,(Citation: DOJ-DPRK Heist) and exploiting cryptocurrency networks.(Citation: BBC-Ronin) 
    FinancialTheft,

    /// [Data Manipulation](https://attack.mitre.org/techniques/T1565)
    ///
    /// Adversaries may insert, delete, or manipulate data in order to influence external outcomes or hide activity, thus threatening the integrity of the data. By manipulating data, adversaries may attempt to affect a business process, organizational understanding, or decision making.
    DataManipulation,

    /// [Account Access Removal](https://attack.mitre.org/techniques/T1531)
    ///
    /// Adversaries may interrupt availability of system and network resources by inhibiting access to accounts utilized by legitimate users. Accounts may be deleted, locked, or manipulated (ex: changed credentials) to remove access to accounts. Adversaries may also subsequently log off and/or perform a [System Shutdown/Reboot](https://attack.mitre.org/techniques/T1529) to set malicious changes into place.(Citation: CarbonBlack LockerGoga 2019)(Citation: Unit42 LockerGoga 2019)
    AccountAccessRemoval,

    /// [Data Encrypted for Impact](https://attack.mitre.org/techniques/T1486)
    ///
    /// Adversaries may encrypt data on target systems or on large numbers of systems in a network to interrupt availability to system and network resources. They can attempt to render stored data inaccessible by encrypting files or data on local and remote drives and withholding access to a decryption key. This may be done in order to extract monetary compensation from a victim in exchange for decryption or a decryption key (ransomware) or to render data permanently inaccessible in cases where the key is not saved or transmitted.(Citation: US-CERT Ransomware 2016)(Citation: FireEye WannaCry 2017)(Citation: US-CERT NotPetya 2017)(Citation: US-CERT SamSam 2018)
    DataEncryptedForImpact,

    /// [Endpoint Denial of Service](https://attack.mitre.org/techniques/T1499)
    ///
    /// Adversaries may perform Endpoint Denial of Service (DoS) attacks to degrade or block the availability of services to users. Endpoint DoS can be performed by exhausting the system resources those services are hosted on or exploiting the system to cause a persistent crash condition. Example services include websites, email services, DNS, and web-based applications. Adversaries have been observed conducting DoS attacks for political purposes(Citation: FireEye OpPoisonedHandover February 2016) and to support other malicious activities, including distraction(Citation: FSISAC FraudNetDoS September 2012), hacktivism, and extortion.(Citation: Symantec DDoS October 2014)
    EndpointDenialOfService,

    /// [Resource Hijacking](https://attack.mitre.org/techniques/T1496)
    ///
    /// Adversaries may leverage the resources of co-opted systems to complete resource-intensive tasks, which may impact system and/or hosted service availability. 
    ResourceHijacking,

    /// [Data Destruction](https://attack.mitre.org/techniques/T1485)
    ///
    /// Adversaries may destroy data and files on specific systems or in large numbers on a network to interrupt availability to systems, services, and network resources. Data destruction is likely to render stored data irrecoverable by forensic techniques through overwriting files or data on local and remote drives.(Citation: Symantec Shamoon 2012)(Citation: FireEye Shamoon Nov 2016)(Citation: Palo Alto Shamoon Nov 2016)(Citation: Kaspersky StoneDrill 2017)(Citation: Unit 42 Shamoon3 2018)(Citation: Talos Olympic Destroyer 2018) Common operating system file deletion commands such as <code>del</code> and <code>rm</code> often only remove pointers to files without wiping the contents of the files themselves, making the files recoverable by proper forensic methodology. This behavior is distinct from [Disk Content Wipe](https://attack.mitre.org/techniques/T1561/001) and [Disk Structure Wipe](https://attack.mitre.org/techniques/T1561/002) because individual files are destroyed rather than sections of a storage disk or the disk's logical structure.
    DataDestruction,

    /// [Network Denial of Service](https://attack.mitre.org/techniques/T1498)
    ///
    /// Adversaries may perform Network Denial of Service (DoS) attacks to degrade or block the availability of targeted resources to users. Network DoS can be performed by exhausting the network bandwidth services rely on. Example resources include specific websites, email services, DNS, and web-based applications. Adversaries have been observed conducting network DoS attacks for political purposes(Citation: FireEye OpPoisonedHandover February 2016) and to support other malicious activities, including distraction(Citation: FSISAC FraudNetDoS September 2012), hacktivism, and extortion.(Citation: Symantec DDoS October 2014)
    NetworkDenialOfService,

    /// [Firmware Corruption](https://attack.mitre.org/techniques/T1495)
    ///
    /// Adversaries may overwrite or corrupt the flash memory contents of system BIOS or other firmware in devices attached to a system in order to render them inoperable or unable to boot, thus denying the availability to use the devices and/or the system.(Citation: Symantec Chernobyl W95.CIH) Firmware is software that is loaded and executed from non-volatile memory on hardware devices in order to initialize and manage device functionality. These devices may include the motherboard, hard drive, or video cards.
    FirmwareCorruption,

    /// [Inhibit System Recovery](https://attack.mitre.org/techniques/T1490)
    ///
    /// Adversaries may delete or remove built-in data and turn off services designed to aid in the recovery of a corrupted system to prevent recovery.(Citation: Talos Olympic Destroyer 2018)(Citation: FireEye WannaCry 2017) This may deny access to available backups and recovery options.
    InhibitSystemRecovery,

    /// [System Shutdown/Reboot](https://attack.mitre.org/techniques/T1529)
    ///
    /// Adversaries may shutdown/reboot systems to interrupt access to, or aid in the destruction of, those systems. Operating systems may contain commands to initiate a shutdown/reboot of a machine or network device. In some cases, these commands may also be used to initiate a shutdown/reboot of a remote computer or network device via [Network Device CLI](https://attack.mitre.org/techniques/T1059/008) (e.g. <code>reload</code>).(Citation: Microsoft Shutdown Oct 2017)(Citation: alert_TA18_106A)
    SystemShutdownReboot,

}
/// [Persistence](https://attack.mitre.org/tactics/TA0003)'s techniques
pub enum Persistence {
    /// [Boot or Logon Initialization Scripts](https://attack.mitre.org/techniques/T1037)
    ///
    /// Adversaries may use scripts automatically executed at boot or logon initialization to establish persistence. Initialization scripts can be used to perform administrative functions, which may often execute other programs or send information to an internal logging server. These scripts can vary based on operating system and whether applied locally or remotely.  
    BootOrLogonInitializationScripts,

    /// [Create or Modify System Process](https://attack.mitre.org/techniques/T1543)
    ///
    /// Adversaries may create or modify system-level processes to repeatedly execute malicious payloads as part of persistence. When operating systems boot up, they can start processes that perform background system functions. On Windows and Linux, these system processes are referred to as services.(Citation: TechNet Services) On macOS, launchd processes known as [Launch Daemon](https://attack.mitre.org/techniques/T1543/004) and [Launch Agent](https://attack.mitre.org/techniques/T1543/001) are run to finish system initialization and load user specific parameters.(Citation: AppleDocs Launch Agent Daemons) 
    CreateOrModifySystemProcess,

    /// [External Remote Services](https://attack.mitre.org/techniques/T1133)
    ///
    /// Adversaries may leverage external-facing remote services to initially access and/or persist within a network. Remote services such as VPNs, Citrix, and other access mechanisms allow users to connect to internal enterprise network resources from external locations. There are often remote service gateways that manage connections and credential authentication for these services. Services such as [Windows Remote Management](https://attack.mitre.org/techniques/T1021/006) and [VNC](https://attack.mitre.org/techniques/T1021/005) can also be used externally.(Citation: MacOS VNC software for Remote Desktop)
    ExternalRemoteServices,

    /// [Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547)
    ///
    /// Adversaries may configure system settings to automatically execute a program during system boot or logon to maintain persistence or gain higher-level privileges on compromised systems. Operating systems may have mechanisms for automatically running a program on system boot or account logon.(Citation: Microsoft Run Key)(Citation: MSDN Authentication Packages)(Citation: Microsoft TimeProvider)(Citation: Cylance Reg Persistence Sept 2013)(Citation: Linux Kernel Programming) These mechanisms may include automatically executing programs that are placed in specially designated directories or are referenced by repositories that store configuration information, such as the Windows Registry. An adversary may achieve the same goal by modifying or extending features of the kernel.
    BootOrLogonAutostartExecution,

    /// [Office Application Startup](https://attack.mitre.org/techniques/T1137)
    ///
    /// Adversaries may leverage Microsoft Office-based applications for persistence between startups. Microsoft Office is a fairly common application suite on Windows-based operating systems within an enterprise network. There are multiple mechanisms that can be used with Office for persistence when an Office-based application is started; this can include the use of Office Template Macros and add-ins.
    OfficeApplicationStartup,

    /// [Scheduled Task/Job](https://attack.mitre.org/techniques/T1053)
    ///
    /// Adversaries may abuse task scheduling functionality to facilitate initial or recurring execution of malicious code. Utilities exist within all major operating systems to schedule programs or scripts to be executed at a specified date and time. A task can also be scheduled on a remote system, provided the proper authentication is met (ex: RPC and file and printer sharing in Windows environments). Scheduling a task on a remote system typically may require being a member of an admin or otherwise privileged group on the remote system.(Citation: TechNet Task Scheduler Security)
    ScheduledTaskJob,

    /// [Browser Extensions](https://attack.mitre.org/techniques/T1176)
    ///
    /// Adversaries may abuse Internet browser extensions to establish persistent access to victim systems. Browser extensions or plugins are small programs that can add functionality and customize aspects of Internet browsers. They can be installed directly or through a browser's app store and generally have access and permissions to everything that the browser can access.(Citation: Wikipedia Browser Extension)(Citation: Chrome Extensions Definition)
    BrowserExtensions,

    /// [Traffic Signaling](https://attack.mitre.org/techniques/T1205)
    ///
    /// Adversaries may use traffic signaling to hide open ports or other malicious functionality used for persistence or command and control. Traffic signaling involves the use of a magic value or sequence that must be sent to a system to trigger a special response, such as opening a closed port or executing a malicious task. This may take the form of sending a series of packets with certain characteristics before a port will be opened that the adversary can use for command and control. Usually this series of packets consists of attempted connections to a predefined sequence of closed ports (i.e. [Port Knocking](https://attack.mitre.org/techniques/T1205/001)), but can involve unusual flags, specific strings, or other unique characteristics. After the sequence is completed, opening a port may be accomplished by the host-based firewall, but could also be implemented by custom software.
    TrafficSignaling,

    /// [Implant Internal Image](https://attack.mitre.org/techniques/T1525)
    ///
    /// Adversaries may implant cloud or container images with malicious code to establish persistence after gaining access to an environment. Amazon Web Services (AWS) Amazon Machine Images (AMIs), Google Cloud Platform (GCP) Images, and Azure Images as well as popular container runtimes such as Docker can be implanted or backdoored. Unlike [Upload Malware](https://attack.mitre.org/techniques/T1608/001), this technique focuses on adversaries implanting an image in a registry within a victim’s environment. Depending on how the infrastructure is provisioned, this could provide persistent access if the infrastructure provisioning tool is instructed to always use the latest image.(Citation: Rhino Labs Cloud Image Backdoor Technique Sept 2019)
    ImplantInternalImage,

    /// [Pre-OS Boot](https://attack.mitre.org/techniques/T1542)
    ///
    /// Adversaries may abuse Pre-OS Boot mechanisms as a way to establish persistence on a system. During the booting process of a computer, firmware and various startup services are loaded before the operating system. These programs control flow of execution before the operating system takes control.(Citation: Wikipedia Booting)
    PreOsBoot,

    /// [Compromise Client Software Binary](https://attack.mitre.org/techniques/T1554)
    ///
    /// Adversaries may modify client software binaries to establish persistent access to systems. Client software enables users to access services provided by a server. Common client software types are SSH clients, FTP clients, email clients, and web browsers.
    CompromiseClientSoftwareBinary,

    /// [Account Manipulation](https://attack.mitre.org/techniques/T1098)
    ///
    /// Adversaries may manipulate accounts to maintain and/or elevate access to victim systems. Account manipulation may consist of any action that preserves or modifies adversary access to a compromised account, such as modifying credentials or permission groups. These actions could also include account activity designed to subvert security policies, such as performing iterative password updates to bypass password duration policies and preserve the life of compromised credentials. 
    AccountManipulation,

    /// [Hijack Execution Flow](https://attack.mitre.org/techniques/T1574)
    ///
    /// Adversaries may execute their own malicious payloads by hijacking the way operating systems run programs. Hijacking execution flow can be for the purposes of persistence, since this hijacked execution may reoccur over time. Adversaries may also use these mechanisms to elevate privileges or evade defenses, such as application control or other restrictions on execution.
    HijackExecutionFlow,

    /// [Valid Accounts](https://attack.mitre.org/techniques/T1078)
    ///
    /// Adversaries may obtain and abuse credentials of existing accounts as a means of gaining Initial Access, Persistence, Privilege Escalation, or Defense Evasion. Compromised credentials may be used to bypass access controls placed on various resources on systems within the network and may even be used for persistent access to remote systems and externally available services, such as VPNs, Outlook Web Access, network devices, and remote desktop.(Citation: volexity_0day_sophos_FW) Compromised credentials may also grant an adversary increased privilege to specific systems or access to restricted areas of the network. Adversaries may choose not to use malware or tools in conjunction with the legitimate access those credentials provide to make it harder to detect their presence.
    ValidAccounts,

    /// [Event Triggered Execution](https://attack.mitre.org/techniques/T1546)
    ///
    /// Adversaries may establish persistence and/or elevate privileges using system mechanisms that trigger execution based on specific events. Various operating systems have means to monitor and subscribe to events such as logons or other user activity such as running specific applications/binaries. Cloud environments may also support various functions and services that monitor and can be invoked in response to specific cloud events.(Citation: Backdooring an AWS account)(Citation: Varonis Power Automate Data Exfiltration)(Citation: Microsoft DART Case Report 001)
    EventTriggeredExecution,

    /// [BITS Jobs](https://attack.mitre.org/techniques/T1197)
    ///
    /// Adversaries may abuse BITS jobs to persistently execute code and perform various background tasks. Windows Background Intelligent Transfer Service (BITS) is a low-bandwidth, asynchronous file transfer mechanism exposed through [Component Object Model](https://attack.mitre.org/techniques/T1559/001) (COM).(Citation: Microsoft COM)(Citation: Microsoft BITS) BITS is commonly used by updaters, messengers, and other applications preferred to operate in the background (using available idle bandwidth) without interrupting other networked applications. File transfer tasks are implemented as BITS jobs, which contain a queue of one or more file operations.
    BitsJobs,

    /// [Server Software Component](https://attack.mitre.org/techniques/T1505)
    ///
    /// Adversaries may abuse legitimate extensible development features of servers to establish persistent access to systems. Enterprise server applications may include features that allow developers to write and install software or scripts to extend the functionality of the main application. Adversaries may install malicious components to extend and abuse server applications.(Citation: volexity_0day_sophos_FW)
    ServerSoftwareComponent,

    /// [Create Account](https://attack.mitre.org/techniques/T1136)
    ///
    /// Adversaries may create an account to maintain access to victim systems. With a sufficient level of access, creating such accounts may be used to establish secondary credentialed access that do not require persistent remote access tools to be deployed on the system.
    CreateAccount,

    /// [Power Settings](https://attack.mitre.org/techniques/T1653)
    ///
    /// Adversaries may impair a system's ability to hibernate, reboot, or shut down in order to extend access to infected machines. When a computer enters a dormant state, some or all software and hardware may cease to operate which can disrupt malicious activity.(Citation: Sleep, shut down, hibernate)
    PowerSettings,

    /// [Modify Authentication Process](https://attack.mitre.org/techniques/T1556)
    ///
    /// Adversaries may modify authentication mechanisms and processes to access user credentials or enable otherwise unwarranted access to accounts. The authentication process is handled by mechanisms, such as the Local Security Authentication Server (LSASS) process and the Security Accounts Manager (SAM) on Windows, pluggable authentication modules (PAM) on Unix-based systems, and authorization plugins on MacOS systems, responsible for gathering, storing, and validating credentials. By modifying an authentication process, an adversary may be able to authenticate to a service or system without using [Valid Accounts](https://attack.mitre.org/techniques/T1078).
    ModifyAuthenticationProcess,

}
/// [Privilege Escalation](https://attack.mitre.org/tactics/TA0004)'s techniques
pub enum PrivilegeEscalation {
    /// [Boot or Logon Initialization Scripts](https://attack.mitre.org/techniques/T1037)
    ///
    /// Adversaries may use scripts automatically executed at boot or logon initialization to establish persistence. Initialization scripts can be used to perform administrative functions, which may often execute other programs or send information to an internal logging server. These scripts can vary based on operating system and whether applied locally or remotely.  
    BootOrLogonInitializationScripts,

    /// [Create or Modify System Process](https://attack.mitre.org/techniques/T1543)
    ///
    /// Adversaries may create or modify system-level processes to repeatedly execute malicious payloads as part of persistence. When operating systems boot up, they can start processes that perform background system functions. On Windows and Linux, these system processes are referred to as services.(Citation: TechNet Services) On macOS, launchd processes known as [Launch Daemon](https://attack.mitre.org/techniques/T1543/004) and [Launch Agent](https://attack.mitre.org/techniques/T1543/001) are run to finish system initialization and load user specific parameters.(Citation: AppleDocs Launch Agent Daemons) 
    CreateOrModifySystemProcess,

    /// [Boot or Logon Autostart Execution](https://attack.mitre.org/techniques/T1547)
    ///
    /// Adversaries may configure system settings to automatically execute a program during system boot or logon to maintain persistence or gain higher-level privileges on compromised systems. Operating systems may have mechanisms for automatically running a program on system boot or account logon.(Citation: Microsoft Run Key)(Citation: MSDN Authentication Packages)(Citation: Microsoft TimeProvider)(Citation: Cylance Reg Persistence Sept 2013)(Citation: Linux Kernel Programming) These mechanisms may include automatically executing programs that are placed in specially designated directories or are referenced by repositories that store configuration information, such as the Windows Registry. An adversary may achieve the same goal by modifying or extending features of the kernel.
    BootOrLogonAutostartExecution,

    /// [Scheduled Task/Job](https://attack.mitre.org/techniques/T1053)
    ///
    /// Adversaries may abuse task scheduling functionality to facilitate initial or recurring execution of malicious code. Utilities exist within all major operating systems to schedule programs or scripts to be executed at a specified date and time. A task can also be scheduled on a remote system, provided the proper authentication is met (ex: RPC and file and printer sharing in Windows environments). Scheduling a task on a remote system typically may require being a member of an admin or otherwise privileged group on the remote system.(Citation: TechNet Task Scheduler Security)
    ScheduledTaskJob,

    /// [Process Injection](https://attack.mitre.org/techniques/T1055)
    ///
    /// Adversaries may inject code into processes in order to evade process-based defenses as well as possibly elevate privileges. Process injection is a method of executing arbitrary code in the address space of a separate live process. Running code in the context of another process may allow access to the process's memory, system/network resources, and possibly elevated privileges. Execution via process injection may also evade detection from security products since the execution is masked under a legitimate process. 
    ProcessInjection,

    /// [Escape to Host](https://attack.mitre.org/techniques/T1611)
    ///
    /// Adversaries may break out of a container to gain access to the underlying host. This can allow an adversary access to other containerized resources from the host level or to the host itself. In principle, containerized resources should provide a clear separation of application functionality and be isolated from the host environment.(Citation: Docker Overview)
    EscapeToHost,

    /// [Abuse Elevation Control Mechanism](https://attack.mitre.org/techniques/T1548)
    ///
    /// Adversaries may circumvent mechanisms designed to control elevate privileges to gain higher-level permissions. Most modern systems contain native elevation control mechanisms that are intended to limit privileges that a user can perform on a machine. Authorization has to be granted to specific users in order to perform tasks that can be considered of higher risk. An adversary can perform several methods to take advantage of built-in control mechanisms in order to escalate privileges on a system.
    AbuseElevationControlMechanism,

    /// [Account Manipulation](https://attack.mitre.org/techniques/T1098)
    ///
    /// Adversaries may manipulate accounts to maintain and/or elevate access to victim systems. Account manipulation may consist of any action that preserves or modifies adversary access to a compromised account, such as modifying credentials or permission groups. These actions could also include account activity designed to subvert security policies, such as performing iterative password updates to bypass password duration policies and preserve the life of compromised credentials. 
    AccountManipulation,

    /// [Hijack Execution Flow](https://attack.mitre.org/techniques/T1574)
    ///
    /// Adversaries may execute their own malicious payloads by hijacking the way operating systems run programs. Hijacking execution flow can be for the purposes of persistence, since this hijacked execution may reoccur over time. Adversaries may also use these mechanisms to elevate privileges or evade defenses, such as application control or other restrictions on execution.
    HijackExecutionFlow,

    /// [Valid Accounts](https://attack.mitre.org/techniques/T1078)
    ///
    /// Adversaries may obtain and abuse credentials of existing accounts as a means of gaining Initial Access, Persistence, Privilege Escalation, or Defense Evasion. Compromised credentials may be used to bypass access controls placed on various resources on systems within the network and may even be used for persistent access to remote systems and externally available services, such as VPNs, Outlook Web Access, network devices, and remote desktop.(Citation: volexity_0day_sophos_FW) Compromised credentials may also grant an adversary increased privilege to specific systems or access to restricted areas of the network. Adversaries may choose not to use malware or tools in conjunction with the legitimate access those credentials provide to make it harder to detect their presence.
    ValidAccounts,

    /// [Exploitation for Privilege Escalation](https://attack.mitre.org/techniques/T1068)
    ///
    /// Adversaries may exploit software vulnerabilities in an attempt to elevate privileges. Exploitation of a software vulnerability occurs when an adversary takes advantage of a programming error in a program, service, or within the operating system software or kernel itself to execute adversary-controlled code. Security constructs such as permission levels will often hinder access to information and use of certain techniques, so adversaries will likely need to perform privilege escalation to include use of software exploitation to circumvent those restrictions.
    ExploitationForPrivilegeEscalation,

    /// [Event Triggered Execution](https://attack.mitre.org/techniques/T1546)
    ///
    /// Adversaries may establish persistence and/or elevate privileges using system mechanisms that trigger execution based on specific events. Various operating systems have means to monitor and subscribe to events such as logons or other user activity such as running specific applications/binaries. Cloud environments may also support various functions and services that monitor and can be invoked in response to specific cloud events.(Citation: Backdooring an AWS account)(Citation: Varonis Power Automate Data Exfiltration)(Citation: Microsoft DART Case Report 001)
    EventTriggeredExecution,

    /// [Access Token Manipulation](https://attack.mitre.org/techniques/T1134)
    ///
    /// Adversaries may modify access tokens to operate under a different user or system security context to perform actions and bypass access controls. Windows uses access tokens to determine the ownership of a running process. A user can manipulate access tokens to make a running process appear as though it is the child of a different process or belongs to someone other than the user that started the process. When this occurs, the process also takes on the security context associated with the new token.
    AccessTokenManipulation,

    /// [Domain Policy Modification](https://attack.mitre.org/techniques/T1484)
    ///
    /// Adversaries may modify the configuration settings of a domain to evade defenses and/or escalate privileges in domain environments. Domains provide a centralized means of managing how computer resources (ex: computers, user accounts) can act, and interact with each other, on a network. The policy of the domain also includes configuration settings that may apply between domains in a multi-domain/forest environment. Modifications to domain settings may include altering domain Group Policy Objects (GPOs) or changing trust settings for domains, including federation trusts.
    DomainPolicyModification,

}
/// [Lateral Movement](https://attack.mitre.org/tactics/TA0008)'s techniques
pub enum LateralMovement {
    /// [Taint Shared Content](https://attack.mitre.org/techniques/T1080)
    ///
    /// 
    TaintSharedContent,

    /// [Replication Through Removable Media](https://attack.mitre.org/techniques/T1091)
    ///
    /// Adversaries may move onto systems, possibly those on disconnected or air-gapped networks, by copying malware to removable media and taking advantage of Autorun features when the media is inserted into a system and executes. In the case of Lateral Movement, this may occur through modification of executable files stored on removable media or by copying malware and renaming it to look like a legitimate file to trick users into executing it on a separate system. In the case of Initial Access, this may occur through manual manipulation of the media, modification of systems used to initially format the media, or modification to the media's firmware itself.
    ReplicationThroughRemovableMedia,

    /// [Use Alternate Authentication Material](https://attack.mitre.org/techniques/T1550)
    ///
    /// Adversaries may use alternate authentication material, such as password hashes, Kerberos tickets, and application access tokens, in order to move laterally within an environment and bypass normal system access controls. 
    UseAlternateAuthenticationMaterial,

    /// [Remote Services](https://attack.mitre.org/techniques/T1021)
    ///
    /// Adversaries may use [Valid Accounts](https://attack.mitre.org/techniques/T1078) to log into a service that accepts remote connections, such as telnet, SSH, and VNC. The adversary may then perform actions as the logged-on user.
    RemoteServices,

    /// [Remote Service Session Hijacking](https://attack.mitre.org/techniques/T1563)
    ///
    /// Adversaries may take control of preexisting sessions with remote services to move laterally in an environment. Users may use valid credentials to log into a service specifically designed to accept remote connections, such as telnet, SSH, and RDP. When a user logs into a service, a session will be established that will allow them to maintain a continuous interaction with that service.
    RemoteServiceSessionHijacking,

    /// [Software Deployment Tools](https://attack.mitre.org/techniques/T1072)
    ///
    /// Adversaries may gain access to and use third-party software suites installed within an enterprise network, such as administration, monitoring, and deployment systems, to move laterally through the network. Third-party applications and software deployment systems may be in use in the network environment for administration purposes (e.g., SCCM, HBSS, Altiris, etc.).  
    SoftwareDeploymentTools,

    /// [Exploitation of Remote Services](https://attack.mitre.org/techniques/T1210)
    ///
    /// Adversaries may exploit remote services to gain unauthorized access to internal systems once inside of a network. Exploitation of a software vulnerability occurs when an adversary takes advantage of a programming error in a program, service, or within the operating system software or kernel itself to execute adversary-controlled code. A common goal for post-compromise exploitation of remote services is for lateral movement to enable access to a remote system.
    ExploitationOfRemoteServices,

    /// [Internal Spearphishing](https://attack.mitre.org/techniques/T1534)
    ///
    /// Adversaries may use internal spearphishing to gain access to additional information or exploit other users within the same organization after they already have access to accounts or systems within the environment. Internal spearphishing is multi-staged campaign where an email account is owned either by controlling the user's device with previously installed malware or by compromising the account credentials of the user. Adversaries attempt to take advantage of a trusted internal account to increase the likelihood of tricking the target into falling for the phish attempt.(Citation: Trend Micro When Phishing Starts from the Inside 2017)
    InternalSpearphishing,

    /// [Lateral Tool Transfer](https://attack.mitre.org/techniques/T1570)
    ///
    /// Adversaries may transfer tools or other files between systems in a compromised environment. Once brought into the victim environment (i.e., [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105)) files may then be copied from one system to another to stage adversary tools or other files over the course of an operation.
    LateralToolTransfer,

}
/// [Defense Evasion](https://attack.mitre.org/tactics/TA0005)'s techniques
pub enum DefenseEvasion {
    /// [Direct Volume Access](https://attack.mitre.org/techniques/T1006)
    ///
    /// Adversaries may directly access a volume to bypass file access controls and file system monitoring. Windows allows programs to have direct access to logical volumes. Programs with direct access may read and write files directly from the drive by analyzing file system data structures. This technique may bypass Windows file access controls as well as file system monitoring tools. (Citation: Hakobyan 2009)
    DirectVolumeAccess,

    /// [Rootkit](https://attack.mitre.org/techniques/T1014)
    ///
    /// Adversaries may use rootkits to hide the presence of programs, files, network connections, services, drivers, and other system components. Rootkits are programs that hide the existence of malware by intercepting/hooking and modifying operating system API calls that supply system information. (Citation: Symantec Windows Rootkits) 
    Rootkit,

    /// [Modify Cloud Compute Infrastructure](https://attack.mitre.org/techniques/T1578)
    ///
    /// An adversary may attempt to modify a cloud account's compute service infrastructure to evade defenses. A modification to the compute service infrastructure can include the creation, deletion, or modification of one or more components such as compute instances, virtual machines, and snapshots.
    ModifyCloudComputeInfrastructure,

    /// [Weaken Encryption](https://attack.mitre.org/techniques/T1600)
    ///
    /// Adversaries may compromise a network device’s encryption capability in order to bypass encryption that would otherwise protect data communications. (Citation: Cisco Synful Knock Evolution)
    WeakenEncryption,

    /// [Hide Artifacts](https://attack.mitre.org/techniques/T1564)
    ///
    /// Adversaries may attempt to hide artifacts associated with their behaviors to evade detection. Operating systems may have features to hide various artifacts, such as important system files and administrative task execution, to avoid disrupting user work environments and prevent users from changing files or features on the system. Adversaries may abuse these features to hide artifacts such as files, directories, user accounts, or other system activity to evade detection.(Citation: Sofacy Komplex Trojan)(Citation: Cybereason OSX Pirrit)(Citation: MalwareBytes ADS July 2015)
    HideArtifacts,

    /// [Indirect Command Execution](https://attack.mitre.org/techniques/T1202)
    ///
    /// Adversaries may abuse utilities that allow for command execution to bypass security restrictions that limit the use of command-line interpreters. Various Windows utilities may be used to execute commands, possibly without invoking [cmd](https://attack.mitre.org/software/S0106). For example, [Forfiles](https://attack.mitre.org/software/S0193), the Program Compatibility Assistant (pcalua.exe), components of the Windows Subsystem for Linux (WSL), as well as other utilities may invoke the execution of programs and commands from a [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059), Run window, or via scripts. (Citation: VectorSec ForFiles Aug 2017) (Citation: Evi1cg Forfiles Nov 2017)
    IndirectCommandExecution,

    /// [Deobfuscate/Decode Files or Information](https://attack.mitre.org/techniques/T1140)
    ///
    /// Adversaries may use [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027) to hide artifacts of an intrusion from analysis. They may require separate mechanisms to decode or deobfuscate that information depending on how they intend to use it. Methods for doing that include built-in functionality of malware or by using utilities present on the system.
    DeobfuscateDecodeFilesOrInformation,

    /// [Impair Defenses](https://attack.mitre.org/techniques/T1562)
    ///
    /// Adversaries may maliciously modify components of a victim environment in order to hinder or disable defensive mechanisms. This not only involves impairing preventative defenses, such as firewalls and anti-virus, but also detection capabilities that defenders can use to audit activity and identify malicious behavior. This may also span both native defenses as well as supplemental capabilities installed by users and administrators.
    ImpairDefenses,

    /// [Masquerading](https://attack.mitre.org/techniques/T1036)
    ///
    /// Adversaries may attempt to manipulate features of their artifacts to make them appear legitimate or benign to users and/or security tools. Masquerading occurs when the name or location of an object, legitimate or malicious, is manipulated or abused for the sake of evading defenses and observation. This may include manipulating file metadata, tricking users into misidentifying the file type, and giving legitimate task or service names.
    Masquerading,

    /// [Process Injection](https://attack.mitre.org/techniques/T1055)
    ///
    /// Adversaries may inject code into processes in order to evade process-based defenses as well as possibly elevate privileges. Process injection is a method of executing arbitrary code in the address space of a separate live process. Running code in the context of another process may allow access to the process's memory, system/network resources, and possibly elevated privileges. Execution via process injection may also evade detection from security products since the execution is masked under a legitimate process. 
    ProcessInjection,

    /// [Traffic Signaling](https://attack.mitre.org/techniques/T1205)
    ///
    /// Adversaries may use traffic signaling to hide open ports or other malicious functionality used for persistence or command and control. Traffic signaling involves the use of a magic value or sequence that must be sent to a system to trigger a special response, such as opening a closed port or executing a malicious task. This may take the form of sending a series of packets with certain characteristics before a port will be opened that the adversary can use for command and control. Usually this series of packets consists of attempted connections to a predefined sequence of closed ports (i.e. [Port Knocking](https://attack.mitre.org/techniques/T1205/001)), but can involve unusual flags, specific strings, or other unique characteristics. After the sequence is completed, opening a port may be accomplished by the host-based firewall, but could also be implemented by custom software.
    TrafficSignaling,

    /// [System Binary Proxy Execution](https://attack.mitre.org/techniques/T1218)
    ///
    /// Adversaries may bypass process and/or signature-based defenses by proxying execution of malicious content with signed, or otherwise trusted, binaries. Binaries used in this technique are often Microsoft-signed files, indicating that they have been either downloaded from Microsoft or are already native in the operating system.(Citation: LOLBAS Project) Binaries signed with trusted digital certificates can typically execute on Windows systems protected by digital signature validation. Several Microsoft signed binaries that are default on Windows installations can be used to proxy execution of other files or commands.
    SystemBinaryProxyExecution,

    /// [Reflective Code Loading](https://attack.mitre.org/techniques/T1620)
    ///
    /// Adversaries may reflectively load code into a process in order to conceal the execution of malicious payloads. Reflective loading involves allocating then executing payloads directly within the memory of the process, vice creating a thread or process backed by a file path on disk. Reflectively loaded payloads may be compiled binaries, anonymous files (only present in RAM), or just snubs of fileless executable code (ex: position-independent shellcode).(Citation: Introducing Donut)(Citation: S1 Custom Shellcode Tool)(Citation: Stuart ELF Memory)(Citation: 00sec Droppers)(Citation: Mandiant BYOL)
    ReflectiveCodeLoading,

    /// [Use Alternate Authentication Material](https://attack.mitre.org/techniques/T1550)
    ///
    /// Adversaries may use alternate authentication material, such as password hashes, Kerberos tickets, and application access tokens, in order to move laterally within an environment and bypass normal system access controls. 
    UseAlternateAuthenticationMaterial,

    /// [Rogue Domain Controller](https://attack.mitre.org/techniques/T1207)
    ///
    /// Adversaries may register a rogue Domain Controller to enable manipulation of Active Directory data. DCShadow may be used to create a rogue Domain Controller (DC). DCShadow is a method of manipulating Active Directory (AD) data, including objects and schemas, by registering (or reusing an inactive registration) and simulating the behavior of a DC. (Citation: DCShadow Blog) Once registered, a rogue DC may be able to inject and replicate changes into AD infrastructure for any domain object, including credentials and keys.
    RogueDomainController,

    /// [Deploy Container](https://attack.mitre.org/techniques/T1610)
    ///
    /// Adversaries may deploy a container into an environment to facilitate execution or evade defenses. In some cases, adversaries may deploy a new container to execute processes associated with a particular image or deployment, such as processes that execute or download malware. In others, an adversary may deploy a new container configured without network rules, user limitations, etc. to bypass existing defenses within the environment.
    DeployContainer,

    /// [Modify Registry](https://attack.mitre.org/techniques/T1112)
    ///
    /// Adversaries may interact with the Windows Registry to hide configuration information within Registry keys, remove information as part of cleaning up, or as part of other techniques to aid in persistence and execution.
    ModifyRegistry,

    /// [Unused/Unsupported Cloud Regions](https://attack.mitre.org/techniques/T1535)
    ///
    /// Adversaries may create cloud instances in unused geographic service regions in order to evade detection. Access is usually obtained through compromising accounts used to manage cloud infrastructure.
    UnusedUnsupportedCloudRegions,

    /// [File and Directory Permissions Modification](https://attack.mitre.org/techniques/T1222)
    ///
    /// Adversaries may modify file or directory permissions/attributes to evade access control lists (ACLs) and access protected files.(Citation: Hybrid Analysis Icacls1 June 2018)(Citation: Hybrid Analysis Icacls2 May 2018) File and directory permissions are commonly managed by ACLs configured by the file or directory owner, or users with the appropriate permissions. File and directory ACL implementations vary by platform, but generally explicitly designate which users or groups can perform which actions (read, write, execute, etc.).
    FileAndDirectoryPermissionsModification,

    /// [Abuse Elevation Control Mechanism](https://attack.mitre.org/techniques/T1548)
    ///
    /// Adversaries may circumvent mechanisms designed to control elevate privileges to gain higher-level permissions. Most modern systems contain native elevation control mechanisms that are intended to limit privileges that a user can perform on a machine. Authorization has to be granted to specific users in order to perform tasks that can be considered of higher risk. An adversary can perform several methods to take advantage of built-in control mechanisms in order to escalate privileges on a system.
    AbuseElevationControlMechanism,

    /// [Indicator Removal](https://attack.mitre.org/techniques/T1070)
    ///
    /// Adversaries may delete or modify artifacts generated within systems to remove evidence of their presence or hinder defenses. Various artifacts may be created by an adversary or something that can be attributed to an adversary’s actions. Typically these artifacts are used as defensive indicators related to monitored events, such as strings from downloaded files, logs that are generated from user actions, and other data analyzed by defenders. Location, format, and type of artifact (such as command or login history) are often specific to each platform.
    IndicatorRemoval,

    /// [Plist File Modification](https://attack.mitre.org/techniques/T1647)
    ///
    /// Adversaries may modify property list files (plist files) to enable other malicious activity, while also potentially evading and bypassing system defenses. macOS applications use plist files, such as the <code>info.plist</code> file, to store properties and configuration settings that inform the operating system how to handle the application at runtime. Plist files are structured metadata in key-value pairs formatted in XML based on Apple's Core Foundation DTD. Plist files can be saved in text or binary format.(Citation: fileinfo plist file description) 
    PlistFileModification,

    /// [Pre-OS Boot](https://attack.mitre.org/techniques/T1542)
    ///
    /// Adversaries may abuse Pre-OS Boot mechanisms as a way to establish persistence on a system. During the booting process of a computer, firmware and various startup services are loaded before the operating system. These programs control flow of execution before the operating system takes control.(Citation: Wikipedia Booting)
    PreOsBoot,

    /// [Build Image on Host](https://attack.mitre.org/techniques/T1612)
    ///
    /// Adversaries may build a container image directly on a host to bypass defenses that monitor for the retrieval of malicious images from a public registry. A remote <code>build</code> request may be sent to the Docker API that includes a Dockerfile that pulls a vanilla base image, such as alpine, from a public or local registry and then builds a custom image upon it.(Citation: Docker Build Image)
    BuildImageOnHost,

    /// [Virtualization/Sandbox Evasion](https://attack.mitre.org/techniques/T1497)
    ///
    /// Adversaries may employ various means to detect and avoid virtualization and analysis environments. This may include changing behaviors based on the results of checks for the presence of artifacts indicative of a virtual machine environment (VME) or sandbox. If the adversary detects a VME, they may alter their malware to disengage from the victim or conceal the core functions of the implant. They may also search for VME artifacts before dropping secondary or additional payloads. Adversaries may use the information learned from [Virtualization/Sandbox Evasion](https://attack.mitre.org/techniques/T1497) during automated discovery to shape follow-on behaviors.(Citation: Deloitte Environment Awareness)
    VirtualizationSandboxEvasion,

    /// [Execution Guardrails](https://attack.mitre.org/techniques/T1480)
    ///
    /// Adversaries may use execution guardrails to constrain execution or actions based on adversary supplied and environment specific conditions that are expected to be present on the target. Guardrails ensure that a payload only executes against an intended target and reduces collateral damage from an adversary’s campaign.(Citation: FireEye Kevin Mandia Guardrails) Values an adversary can provide about a target system or environment to use as guardrails may include specific network share names, attached physical devices, files, joined Active Directory (AD) domains, and local/external IP addresses.(Citation: FireEye Outlook Dec 2019)
    ExecutionGuardrails,

    /// [Modify System Image](https://attack.mitre.org/techniques/T1601)
    ///
    /// Adversaries may make changes to the operating system of embedded network devices to weaken defenses and provide new capabilities for themselves.  On such devices, the operating systems are typically monolithic and most of the device functionality and capabilities are contained within a single file.
    ModifySystemImage,

    /// [Hijack Execution Flow](https://attack.mitre.org/techniques/T1574)
    ///
    /// Adversaries may execute their own malicious payloads by hijacking the way operating systems run programs. Hijacking execution flow can be for the purposes of persistence, since this hijacked execution may reoccur over time. Adversaries may also use these mechanisms to elevate privileges or evade defenses, such as application control or other restrictions on execution.
    HijackExecutionFlow,

    /// [Valid Accounts](https://attack.mitre.org/techniques/T1078)
    ///
    /// Adversaries may obtain and abuse credentials of existing accounts as a means of gaining Initial Access, Persistence, Privilege Escalation, or Defense Evasion. Compromised credentials may be used to bypass access controls placed on various resources on systems within the network and may even be used for persistent access to remote systems and externally available services, such as VPNs, Outlook Web Access, network devices, and remote desktop.(Citation: volexity_0day_sophos_FW) Compromised credentials may also grant an adversary increased privilege to specific systems or access to restricted areas of the network. Adversaries may choose not to use malware or tools in conjunction with the legitimate access those credentials provide to make it harder to detect their presence.
    ValidAccounts,

    /// [Obfuscated Files or Information](https://attack.mitre.org/techniques/T1027)
    ///
    /// Adversaries may attempt to make an executable or file difficult to discover or analyze by encrypting, encoding, or otherwise obfuscating its contents on the system or in transit. This is common behavior that can be used across different platforms and the network to evade defenses. 
    ObfuscatedFilesOrInformation,

    /// [Network Boundary Bridging](https://attack.mitre.org/techniques/T1599)
    ///
    /// Adversaries may bridge network boundaries by compromising perimeter network devices or internal devices responsible for network segmentation. Breaching these devices may enable an adversary to bypass restrictions on traffic routing that otherwise separate trusted and untrusted networks.
    NetworkBoundaryBridging,

    /// [Subvert Trust Controls](https://attack.mitre.org/techniques/T1553)
    ///
    /// Adversaries may undermine security controls that will either warn users of untrusted activity or prevent execution of untrusted programs. Operating systems and security products may contain mechanisms to identify programs or websites as possessing some level of trust. Examples of such features would include a program being allowed to run because it is signed by a valid code signing certificate, a program prompting the user with a warning because it has an attribute set from being downloaded from the Internet, or getting an indication that you are about to connect to an untrusted site.
    SubvertTrustControls,

    /// [BITS Jobs](https://attack.mitre.org/techniques/T1197)
    ///
    /// Adversaries may abuse BITS jobs to persistently execute code and perform various background tasks. Windows Background Intelligent Transfer Service (BITS) is a low-bandwidth, asynchronous file transfer mechanism exposed through [Component Object Model](https://attack.mitre.org/techniques/T1559/001) (COM).(Citation: Microsoft COM)(Citation: Microsoft BITS) BITS is commonly used by updaters, messengers, and other applications preferred to operate in the background (using available idle bandwidth) without interrupting other networked applications. File transfer tasks are implemented as BITS jobs, which contain a queue of one or more file operations.
    BitsJobs,

    /// [Impersonation](https://attack.mitre.org/techniques/T1656)
    ///
    /// Adversaries may impersonate a trusted person or organization in order to persuade and trick a target into performing some action on their behalf. For example, adversaries may communicate with victims (via [Phishing for Information](https://attack.mitre.org/techniques/T1598), [Phishing](https://attack.mitre.org/techniques/T1566), or [Internal Spearphishing](https://attack.mitre.org/techniques/T1534)) while impersonating a known sender such as an executive, colleague, or third-party vendor. Established trust can then be leveraged to accomplish an adversary’s ultimate goals, possibly against multiple victims. 
    Impersonation,

    /// [Template Injection](https://attack.mitre.org/techniques/T1221)
    ///
    /// Adversaries may create or modify references in user document templates to conceal malicious code or force authentication attempts. For example, Microsoft’s Office Open XML (OOXML) specification defines an XML-based format for Office documents (.docx, xlsx, .pptx) to replace older binary formats (.doc, .xls, .ppt). OOXML files are packed together ZIP archives compromised of various XML files, referred to as parts, containing properties that collectively define how a document is rendered.(Citation: Microsoft Open XML July 2017)
    TemplateInjection,

    /// [Access Token Manipulation](https://attack.mitre.org/techniques/T1134)
    ///
    /// Adversaries may modify access tokens to operate under a different user or system security context to perform actions and bypass access controls. Windows uses access tokens to determine the ownership of a running process. A user can manipulate access tokens to make a running process appear as though it is the child of a different process or belongs to someone other than the user that started the process. When this occurs, the process also takes on the security context associated with the new token.
    AccessTokenManipulation,

    /// [Debugger Evasion](https://attack.mitre.org/techniques/T1622)
    ///
    /// Adversaries may employ various means to detect and avoid debuggers. Debuggers are typically used by defenders to trace and/or analyze the execution of potential malware payloads.(Citation: ProcessHacker Github)
    DebuggerEvasion,

    /// [Domain Policy Modification](https://attack.mitre.org/techniques/T1484)
    ///
    /// Adversaries may modify the configuration settings of a domain to evade defenses and/or escalate privileges in domain environments. Domains provide a centralized means of managing how computer resources (ex: computers, user accounts) can act, and interact with each other, on a network. The policy of the domain also includes configuration settings that may apply between domains in a multi-domain/forest environment. Modifications to domain settings may include altering domain Group Policy Objects (GPOs) or changing trust settings for domains, including federation trusts.
    DomainPolicyModification,

    /// [XSL Script Processing](https://attack.mitre.org/techniques/T1220)
    ///
    /// Adversaries may bypass application control and obscure execution of code by embedding scripts inside XSL files. Extensible Stylesheet Language (XSL) files are commonly used to describe the processing and rendering of data within XML files. To support complex operations, the XSL standard includes support for embedded scripting in various languages. (Citation: Microsoft XSLT Script Mar 2017)
    XslScriptProcessing,

    /// [Modify Authentication Process](https://attack.mitre.org/techniques/T1556)
    ///
    /// Adversaries may modify authentication mechanisms and processes to access user credentials or enable otherwise unwarranted access to accounts. The authentication process is handled by mechanisms, such as the Local Security Authentication Server (LSASS) process and the Security Accounts Manager (SAM) on Windows, pluggable authentication modules (PAM) on Unix-based systems, and authorization plugins on MacOS systems, responsible for gathering, storing, and validating credentials. By modifying an authentication process, an adversary may be able to authenticate to a service or system without using [Valid Accounts](https://attack.mitre.org/techniques/T1078).
    ModifyAuthenticationProcess,

    /// [System Script Proxy Execution](https://attack.mitre.org/techniques/T1216)
    ///
    /// Adversaries may use trusted scripts, often signed with certificates, to proxy the execution of malicious files. Several Microsoft signed scripts that have been downloaded from Microsoft or are default on Windows installations can be used to proxy execution of other files.(Citation: LOLBAS Project) This behavior may be abused by adversaries to execute malicious files that could bypass application control and signature validation on systems.(Citation: GitHub Ultimate AppLocker Bypass List)
    SystemScriptProxyExecution,

    /// [Exploitation for Defense Evasion](https://attack.mitre.org/techniques/T1211)
    ///
    /// Adversaries may exploit a system or application vulnerability to bypass security features. Exploitation of a vulnerability occurs when an adversary takes advantage of a programming error in a program, service, or within the operating system software or kernel itself to execute adversary-controlled code. Vulnerabilities may exist in defensive security software that can be used to disable or circumvent them.
    ExploitationForDefenseEvasion,

    /// [Trusted Developer Utilities Proxy Execution](https://attack.mitre.org/techniques/T1127)
    ///
    /// Adversaries may take advantage of trusted developer utilities to proxy execution of malicious payloads. There are many utilities used for software development related tasks that can be used to execute code in various forms to assist in development, debugging, and reverse engineering.(Citation: engima0x3 DNX Bypass)(Citation: engima0x3 RCSI Bypass)(Citation: Exploit Monday WinDbg)(Citation: LOLBAS Tracker) These utilities may often be signed with legitimate certificates that allow them to execute on a system and proxy execution of malicious code through a trusted process that effectively bypasses application control solutions.
    TrustedDeveloperUtilitiesProxyExecution,

}
/// [Exfiltration](https://attack.mitre.org/tactics/TA0010)'s techniques
pub enum Exfiltration {
    /// [Exfiltration Over Web Service](https://attack.mitre.org/techniques/T1567)
    ///
    /// Adversaries may use an existing, legitimate external Web service to exfiltrate data rather than their primary command and control channel. Popular Web services acting as an exfiltration mechanism may give a significant amount of cover due to the likelihood that hosts within a network are already communicating with them prior to compromise. Firewall rules may also already exist to permit traffic to these services.
    ExfiltrationOverWebService,

    /// [Scheduled Transfer](https://attack.mitre.org/techniques/T1029)
    ///
    /// Adversaries may schedule data exfiltration to be performed only at certain times of day or at certain intervals. This could be done to blend traffic patterns with normal activity or availability.
    ScheduledTransfer,

    /// [Exfiltration Over Other Network Medium](https://attack.mitre.org/techniques/T1011)
    ///
    /// Adversaries may attempt to exfiltrate data over a different network medium than the command and control channel. If the command and control network is a wired Internet connection, the exfiltration may occur, for example, over a WiFi connection, modem, cellular data connection, Bluetooth, or another radio frequency (RF) channel.
    ExfiltrationOverOtherNetworkMedium,

    /// [Automated Exfiltration](https://attack.mitre.org/techniques/T1020)
    ///
    /// Adversaries may exfiltrate data, such as sensitive documents, through the use of automated processing after being gathered during Collection. 
    AutomatedExfiltration,

    /// [Exfiltration Over C2 Channel](https://attack.mitre.org/techniques/T1041)
    ///
    /// Adversaries may steal data by exfiltrating it over an existing command and control channel. Stolen data is encoded into the normal communications channel using the same protocol as command and control communications.
    ExfiltrationOverC2Channel,

    /// [Exfiltration Over Alternative Protocol](https://attack.mitre.org/techniques/T1048)
    ///
    /// Adversaries may steal data by exfiltrating it over a different protocol than that of the existing command and control channel. The data may also be sent to an alternate network location from the main command and control server.  
    ExfiltrationOverAlternativeProtocol,

    /// [Data Transfer Size Limits](https://attack.mitre.org/techniques/T1030)
    ///
    /// An adversary may exfiltrate data in fixed size chunks instead of whole files or limit packet sizes below certain thresholds. This approach may be used to avoid triggering network data transfer threshold alerts.
    DataTransferSizeLimits,

    /// [Transfer Data to Cloud Account](https://attack.mitre.org/techniques/T1537)
    ///
    /// Adversaries may exfiltrate data by transferring the data, including backups of cloud environments, to another cloud account they control on the same service to avoid typical file transfers/downloads and network-based exfiltration detection.
    TransferDataToCloudAccount,

    /// [Exfiltration Over Physical Medium](https://attack.mitre.org/techniques/T1052)
    ///
    /// Adversaries may attempt to exfiltrate data via a physical medium, such as a removable drive. In certain circumstances, such as an air-gapped network compromise, exfiltration could occur via a physical medium or device introduced by a user. Such media could be an external hard drive, USB drive, cellular phone, MP3 player, or other removable storage and processing device. The physical medium or device could be used as the final exfiltration point or to hop between otherwise disconnected systems.
    ExfiltrationOverPhysicalMedium,

}
/// [Discovery](https://attack.mitre.org/tactics/TA0007)'s techniques
pub enum Discovery {
    /// [System Owner/User Discovery](https://attack.mitre.org/techniques/T1033)
    ///
    /// Adversaries may attempt to identify the primary user, currently logged in user, set of users that commonly uses a system, or whether a user is actively using the system. They may do this, for example, by retrieving account usernames or by using [OS Credential Dumping](https://attack.mitre.org/techniques/T1003). The information may be collected in a number of different ways using other Discovery techniques, because user and username details are prevalent throughout a system and include running process ownership, file/directory ownership, session information, and system logs. Adversaries may use the information from [System Owner/User Discovery](https://attack.mitre.org/techniques/T1033) during automated discovery to shape follow-on behaviors, including whether or not the adversary fully infects the target and/or attempts specific actions.
    SystemOwnerUserDiscovery,

    /// [Container and Resource Discovery](https://attack.mitre.org/techniques/T1613)
    ///
    /// Adversaries may attempt to discover containers and other resources that are available within a containers environment. Other resources may include images, deployments, pods, nodes, and other information such as the status of a cluster.
    ContainerAndResourceDiscovery,

    /// [Permission Groups Discovery](https://attack.mitre.org/techniques/T1069)
    ///
    /// Adversaries may attempt to discover group and permission settings. This information can help adversaries determine which user accounts and groups are available, the membership of users in particular groups, and which users and groups have elevated permissions.
    PermissionGroupsDiscovery,

    /// [Group Policy Discovery](https://attack.mitre.org/techniques/T1615)
    ///
    /// Adversaries may gather information on Group Policy settings to identify paths for privilege escalation, security measures applied within a domain, and to discover patterns in domain objects that can be manipulated or used to blend in the environment. Group Policy allows for centralized management of user and computer settings in Active Directory (AD). Group policy objects (GPOs) are containers for group policy settings made up of files stored within a predictable network path `\<DOMAIN>\SYSVOL\<DOMAIN>\Policies\`.(Citation: TechNet Group Policy Basics)(Citation: ADSecurity GPO Persistence 2016)
    GroupPolicyDiscovery,

    /// [Device Driver Discovery](https://attack.mitre.org/techniques/T1652)
    ///
    /// Adversaries may attempt to enumerate local device drivers on a victim host. Information about device drivers may highlight various insights that shape follow-on behaviors, such as the function/purpose of the host, present security tools (i.e. [Security Software Discovery](https://attack.mitre.org/techniques/T1518/001)) or other defenses (e.g., [Virtualization/Sandbox Evasion](https://attack.mitre.org/techniques/T1497)), as well as potential exploitable vulnerabilities (e.g., [Exploitation for Privilege Escalation](https://attack.mitre.org/techniques/T1068)).
    DeviceDriverDiscovery,

    /// [System Service Discovery](https://attack.mitre.org/techniques/T1007)
    ///
    /// Adversaries may try to gather information about registered local system services. Adversaries may obtain information about services using tools as well as OS utility commands such as <code>sc query</code>, <code>tasklist /svc</code>, <code>systemctl --type=service</code>, and <code>net start</code>.
    SystemServiceDiscovery,

    /// [Network Sniffing](https://attack.mitre.org/techniques/T1040)
    ///
    /// Adversaries may sniff network traffic to capture information about an environment, including authentication material passed over the network. Network sniffing refers to using the network interface on a system to monitor or capture information sent over a wired or wireless connection. An adversary may place a network interface into promiscuous mode to passively access data in transit over the network, or use span ports to capture a larger amount of data.
    NetworkSniffing,

    /// [Network Share Discovery](https://attack.mitre.org/techniques/T1135)
    ///
    /// Adversaries may look for folders and drives shared on remote systems as a means of identifying sources of information to gather as a precursor for Collection and to identify potential systems of interest for Lateral Movement. Networks often contain shared network drives and folders that enable users to access file directories on various systems across a network. 
    NetworkShareDiscovery,

    /// [Peripheral Device Discovery](https://attack.mitre.org/techniques/T1120)
    ///
    /// Adversaries may attempt to gather information about attached peripheral devices and components connected to a computer system.(Citation: Peripheral Discovery Linux)(Citation: Peripheral Discovery macOS) Peripheral devices could include auxiliary resources that support a variety of functionalities such as keyboards, printers, cameras, smart card readers, or removable storage. The information may be used to enhance their awareness of the system and network environment or may be used for further actions.
    PeripheralDeviceDiscovery,

    /// [System Information Discovery](https://attack.mitre.org/techniques/T1082)
    ///
    /// An adversary may attempt to get detailed information about the operating system and hardware, including version, patches, hotfixes, service packs, and architecture. Adversaries may use the information from [System Information Discovery](https://attack.mitre.org/techniques/T1082) during automated discovery to shape follow-on behaviors, including whether or not the adversary fully infects the target and/or attempts specific actions.
    SystemInformationDiscovery,

    /// [Application Window Discovery](https://attack.mitre.org/techniques/T1010)
    ///
    /// Adversaries may attempt to get a listing of open application windows. Window listings could convey information about how the system is used.(Citation: Prevailion DarkWatchman 2021) For example, information about application windows could be used identify potential data to collect as well as identifying security tooling ([Security Software Discovery](https://attack.mitre.org/techniques/T1518/001)) to evade.(Citation: ESET Grandoreiro April 2020)
    ApplicationWindowDiscovery,

    /// [Cloud Infrastructure Discovery](https://attack.mitre.org/techniques/T1580)
    ///
    /// An adversary may attempt to discover infrastructure and resources that are available within an infrastructure-as-a-service (IaaS) environment. This includes compute service resources such as instances, virtual machines, and snapshots as well as resources of other services including the storage and database services.
    CloudInfrastructureDiscovery,

    /// [Browser Information Discovery](https://attack.mitre.org/techniques/T1217)
    ///
    /// Adversaries may enumerate information about browsers to learn more about compromised environments. Data saved by browsers (such as bookmarks, accounts, and browsing history) may reveal a variety of personal information about users (e.g., banking sites, relationships/interests, social media, etc.) as well as details about internal network resources such as servers, tools/dashboards, or other related infrastructure.(Citation: Kaspersky Autofill)
    BrowserInformationDiscovery,

    /// [System Network Configuration Discovery](https://attack.mitre.org/techniques/T1016)
    ///
    /// Adversaries may look for details about the network configuration and settings, such as IP and/or MAC addresses, of systems they access or through information discovery of remote systems. Several operating system administration utilities exist that can be used to gather this information. Examples include [Arp](https://attack.mitre.org/software/S0099), [ipconfig](https://attack.mitre.org/software/S0100)/[ifconfig](https://attack.mitre.org/software/S0101), [nbtstat](https://attack.mitre.org/software/S0102), and [route](https://attack.mitre.org/software/S0103).
    SystemNetworkConfigurationDiscovery,

    /// [Account Discovery](https://attack.mitre.org/techniques/T1087)
    ///
    /// Adversaries may attempt to get a listing of valid accounts, usernames, or email addresses on a system or within a compromised environment. This information can help adversaries determine which accounts exist, which can aid in follow-on behavior such as brute-forcing, spear-phishing attacks, or account takeovers (e.g., [Valid Accounts](https://attack.mitre.org/techniques/T1078)).
    AccountDiscovery,

    /// [Domain Trust Discovery](https://attack.mitre.org/techniques/T1482)
    ///
    /// Adversaries may attempt to gather information on domain trust relationships that may be used to identify lateral movement opportunities in Windows multi-domain/forest environments. Domain trusts provide a mechanism for a domain to allow access to resources based on the authentication procedures of another domain.(Citation: Microsoft Trusts) Domain trusts allow the users of the trusted domain to access resources in the trusting domain. The information discovered may help the adversary conduct [SID-History Injection](https://attack.mitre.org/techniques/T1134/005), [Pass the Ticket](https://attack.mitre.org/techniques/T1550/003), and [Kerberoasting](https://attack.mitre.org/techniques/T1558/003).(Citation: AdSecurity Forging Trust Tickets)(Citation: Harmj0y Domain Trusts) Domain trusts can be enumerated using the `DSEnumerateDomainTrusts()` Win32 API call, .NET methods, and LDAP.(Citation: Harmj0y Domain Trusts) The Windows utility [Nltest](https://attack.mitre.org/software/S0359) is known to be used by adversaries to enumerate domain trusts.(Citation: Microsoft Operation Wilysupply)
    DomainTrustDiscovery,

    /// [File and Directory Discovery](https://attack.mitre.org/techniques/T1083)
    ///
    /// Adversaries may enumerate files and directories or may search in specific locations of a host or network share for certain information within a file system. Adversaries may use the information from [File and Directory Discovery](https://attack.mitre.org/techniques/T1083) during automated discovery to shape follow-on behaviors, including whether or not the adversary fully infects the target and/or attempts specific actions.
    FileAndDirectoryDiscovery,

    /// [System Network Connections Discovery](https://attack.mitre.org/techniques/T1049)
    ///
    /// Adversaries may attempt to get a listing of network connections to or from the compromised system they are currently accessing or from remote systems by querying for information over the network. 
    SystemNetworkConnectionsDiscovery,

    /// [Virtualization/Sandbox Evasion](https://attack.mitre.org/techniques/T1497)
    ///
    /// Adversaries may employ various means to detect and avoid virtualization and analysis environments. This may include changing behaviors based on the results of checks for the presence of artifacts indicative of a virtual machine environment (VME) or sandbox. If the adversary detects a VME, they may alter their malware to disengage from the victim or conceal the core functions of the implant. They may also search for VME artifacts before dropping secondary or additional payloads. Adversaries may use the information learned from [Virtualization/Sandbox Evasion](https://attack.mitre.org/techniques/T1497) during automated discovery to shape follow-on behaviors.(Citation: Deloitte Environment Awareness)
    VirtualizationSandboxEvasion,

    /// [Cloud Storage Object Discovery](https://attack.mitre.org/techniques/T1619)
    ///
    /// Adversaries may enumerate objects in cloud storage infrastructure. Adversaries may use this information during automated discovery to shape follow-on behaviors, including requesting all or specific objects from cloud storage.  Similar to [File and Directory Discovery](https://attack.mitre.org/techniques/T1083) on a local host, after identifying available storage services (i.e. [Cloud Infrastructure Discovery](https://attack.mitre.org/techniques/T1580)) adversaries may access the contents/objects stored in cloud infrastructure.
    CloudStorageObjectDiscovery,

    /// [Log Enumeration](https://attack.mitre.org/techniques/T1654)
    ///
    /// Adversaries may enumerate system and service logs to find useful data. These logs may highlight various types of valuable insights for an adversary, such as user authentication records ([Account Discovery](https://attack.mitre.org/techniques/T1087)), security or vulnerable software ([Software Discovery](https://attack.mitre.org/techniques/T1518)), or hosts within a compromised network ([Remote System Discovery](https://attack.mitre.org/techniques/T1018)).
    LogEnumeration,

    /// [Process Discovery](https://attack.mitre.org/techniques/T1057)
    ///
    /// Adversaries may attempt to get information about running processes on a system. Information obtained could be used to gain an understanding of common software/applications running on systems within the network. Adversaries may use the information from [Process Discovery](https://attack.mitre.org/techniques/T1057) during automated discovery to shape follow-on behaviors, including whether or not the adversary fully infects the target and/or attempts specific actions.
    ProcessDiscovery,

    /// [Password Policy Discovery](https://attack.mitre.org/techniques/T1201)
    ///
    /// Adversaries may attempt to access detailed information about the password policy used within an enterprise network or cloud environment. Password policies are a way to enforce complex passwords that are difficult to guess or crack through [Brute Force](https://attack.mitre.org/techniques/T1110). This information may help the adversary to create a list of common passwords and launch dictionary and/or brute force attacks which adheres to the policy (e.g. if the minimum password length should be 8, then not trying passwords such as 'pass123'; not checking for more than 3-4 passwords per account if the lockout is set to 6 as to not lock out accounts).
    PasswordPolicyDiscovery,

    /// [Query Registry](https://attack.mitre.org/techniques/T1012)
    ///
    /// Adversaries may interact with the Windows Registry to gather information about the system, configuration, and installed software.
    QueryRegistry,

    /// [System Location Discovery](https://attack.mitre.org/techniques/T1614)
    ///
    /// 
    SystemLocationDiscovery,

    /// [Cloud Service Discovery](https://attack.mitre.org/techniques/T1526)
    ///
    /// An adversary may attempt to enumerate the cloud services running on a system after gaining access. These methods can differ from platform-as-a-service (PaaS), to infrastructure-as-a-service (IaaS), or software-as-a-service (SaaS). Many services exist throughout the various cloud providers and can include Continuous Integration and Continuous Delivery (CI/CD), Lambda Functions, Azure AD, etc. They may also include security services, such as AWS GuardDuty and Microsoft Defender for Cloud, and logging services, such as AWS CloudTrail and Google Cloud Audit Logs.
    CloudServiceDiscovery,

    /// [Remote System Discovery](https://attack.mitre.org/techniques/T1018)
    ///
    /// Adversaries may attempt to get a listing of other systems by IP address, hostname, or other logical identifier on a network that may be used for Lateral Movement from the current system. Functionality could exist within remote access tools to enable this, but utilities available on the operating system could also be used such as  [Ping](https://attack.mitre.org/software/S0097) or <code>net view</code> using [Net](https://attack.mitre.org/software/S0039).
    RemoteSystemDiscovery,

    /// [Network Service Discovery](https://attack.mitre.org/techniques/T1046)
    ///
    /// Adversaries may attempt to get a listing of services running on remote hosts and local network infrastructure devices, including those that may be vulnerable to remote software exploitation. Common methods to acquire this information include port and/or vulnerability scans using tools that are brought onto a system.(Citation: CISA AR21-126A FIVEHANDS May 2021)   
    NetworkServiceDiscovery,

    /// [Software Discovery](https://attack.mitre.org/techniques/T1518)
    ///
    /// Adversaries may attempt to get a listing of software and software versions that are installed on a system or in a cloud environment. Adversaries may use the information from [Software Discovery](https://attack.mitre.org/techniques/T1518) during automated discovery to shape follow-on behaviors, including whether or not the adversary fully infects the target and/or attempts specific actions.
    SoftwareDiscovery,

    /// [Cloud Service Dashboard](https://attack.mitre.org/techniques/T1538)
    ///
    /// An adversary may use a cloud service dashboard GUI with stolen credentials to gain useful information from an operational cloud environment, such as specific services, resources, and features. For example, the GCP Command Center can be used to view all assets, findings of potential security risks, and to run additional queries, such as finding public IP addresses and open ports.(Citation: Google Command Center Dashboard)
    CloudServiceDashboard,

    /// [Debugger Evasion](https://attack.mitre.org/techniques/T1622)
    ///
    /// Adversaries may employ various means to detect and avoid debuggers. Debuggers are typically used by defenders to trace and/or analyze the execution of potential malware payloads.(Citation: ProcessHacker Github)
    DebuggerEvasion,

    /// [System Time Discovery](https://attack.mitre.org/techniques/T1124)
    ///
    /// An adversary may gather the system time and/or time zone from a local or remote system. The system time is set and stored by the Windows Time Service within a domain to maintain time synchronization between systems and services in an enterprise network. (Citation: MSDN System Time)(Citation: Technet Windows Time Service)
    SystemTimeDiscovery,

}
/// [Collection](https://attack.mitre.org/tactics/TA0009)'s techniques
pub enum Collection {
    /// [Screen Capture](https://attack.mitre.org/techniques/T1113)
    ///
    /// Adversaries may attempt to take screen captures of the desktop to gather information over the course of an operation. Screen capturing functionality may be included as a feature of a remote access tool used in post-compromise operations. Taking a screenshot is also typically possible through native utilities or API calls, such as <code>CopyFromScreen</code>, <code>xwd</code>, or <code>screencapture</code>.(Citation: CopyFromScreen .NET)(Citation: Antiquated Mac Malware)
    ScreenCapture,

    /// [Adversary-in-the-Middle](https://attack.mitre.org/techniques/T1557)
    ///
    /// Adversaries may attempt to position themselves between two or more networked devices using an adversary-in-the-middle (AiTM) technique to support follow-on behaviors such as [Network Sniffing](https://attack.mitre.org/techniques/T1040), [Transmitted Data Manipulation](https://attack.mitre.org/techniques/T1565/002), or replay attacks ([Exploitation for Credential Access](https://attack.mitre.org/techniques/T1212)). By abusing features of common networking protocols that can determine the flow of network traffic (e.g. ARP, DNS, LLMNR, etc.), adversaries may force a device to communicate through an adversary controlled system so they can collect information or perform additional actions.(Citation: Rapid7 MiTM Basics)
    AdversaryInTheMiddle,

    /// [Data from Configuration Repository](https://attack.mitre.org/techniques/T1602)
    ///
    /// Adversaries may collect data related to managed devices from configuration repositories. Configuration repositories are used by management systems in order to configure, manage, and control data on remote systems. Configuration repositories may also facilitate remote access and administration of devices.
    DataFromConfigurationRepository,

    /// [Audio Capture](https://attack.mitre.org/techniques/T1123)
    ///
    /// An adversary can leverage a computer's peripheral devices (e.g., microphones and webcams) or applications (e.g., voice and video call services) to capture audio recordings for the purpose of listening into sensitive conversations to gather information.
    AudioCapture,

    /// [Email Collection](https://attack.mitre.org/techniques/T1114)
    ///
    /// Adversaries may target user email to collect sensitive information. Emails may contain sensitive data, including trade secrets or personal information, that can prove valuable to adversaries. Adversaries can collect or forward email from mail servers or clients. 
    EmailCollection,

    /// [Data from Removable Media](https://attack.mitre.org/techniques/T1025)
    ///
    /// Adversaries may search connected removable media on computers they have compromised to find files of interest. Sensitive data can be collected from any removable media (optical disk drive, USB memory, etc.) connected to the compromised system prior to Exfiltration. Interactive command shells may be in use, and common functionality within [cmd](https://attack.mitre.org/software/S0106) may be used to gather information. 
    DataFromRemovableMedia,

    /// [Automated Collection](https://attack.mitre.org/techniques/T1119)
    ///
    /// Once established within a system or network, an adversary may use automated techniques for collecting internal data. Methods for performing this technique could include use of a [Command and Scripting Interpreter](https://attack.mitre.org/techniques/T1059) to search for and copy information fitting set criteria such as file type, location, or name at specific time intervals. In cloud-based environments, adversaries may also use cloud APIs, command line interfaces, or extract, transform, and load (ETL) services to automatically collect data. This functionality could also be built into remote access tools. 
    AutomatedCollection,

    /// [Clipboard Data](https://attack.mitre.org/techniques/T1115)
    ///
    /// Adversaries may collect data stored in the clipboard from users copying information within or between applications. 
    ClipboardData,

    /// [Data from Cloud Storage](https://attack.mitre.org/techniques/T1530)
    ///
    /// Adversaries may access data from cloud storage.
    DataFromCloudStorage,

    /// [Data from Local System](https://attack.mitre.org/techniques/T1005)
    ///
    /// Adversaries may search local system sources, such as file systems and configuration files or local databases, to find files of interest and sensitive data prior to Exfiltration.
    DataFromLocalSystem,

    /// [Archive Collected Data](https://attack.mitre.org/techniques/T1560)
    ///
    /// An adversary may compress and/or encrypt data that is collected prior to exfiltration. Compressing the data can help to obfuscate the collected data and minimize the amount of data sent over the network. Encryption can be used to hide information that is being exfiltrated from detection or make exfiltration less conspicuous upon inspection by a defender.
    ArchiveCollectedData,

    /// [Browser Session Hijacking](https://attack.mitre.org/techniques/T1185)
    ///
    /// Adversaries may take advantage of security vulnerabilities and inherent functionality in browser software to change content, modify user-behaviors, and intercept information as part of various browser session hijacking techniques.(Citation: Wikipedia Man in the Browser)
    BrowserSessionHijacking,

    /// [Video Capture](https://attack.mitre.org/techniques/T1125)
    ///
    /// An adversary can leverage a computer's peripheral devices (e.g., integrated cameras or webcams) or applications (e.g., video call services) to capture video recordings for the purpose of gathering information. Images may also be captured from devices or applications, potentially in specified intervals, in lieu of video files.
    VideoCapture,

    /// [Data Staged](https://attack.mitre.org/techniques/T1074)
    ///
    /// Adversaries may stage collected data in a central location or directory prior to Exfiltration. Data may be kept in separate files or combined into one file through techniques such as [Archive Collected Data](https://attack.mitre.org/techniques/T1560). Interactive command shells may be used, and common functionality within [cmd](https://attack.mitre.org/software/S0106) and bash may be used to copy data into a staging location.(Citation: PWC Cloud Hopper April 2017)
    DataStaged,

    /// [Data from Network Shared Drive](https://attack.mitre.org/techniques/T1039)
    ///
    /// Adversaries may search network shares on computers they have compromised to find files of interest. Sensitive data can be collected from remote systems via shared network drives (host shared directory, network file server, etc.) that are accessible from the current system prior to Exfiltration. Interactive command shells may be in use, and common functionality within [cmd](https://attack.mitre.org/software/S0106) may be used to gather information.
    DataFromNetworkSharedDrive,

    /// [Input Capture](https://attack.mitre.org/techniques/T1056)
    ///
    /// Adversaries may use methods of capturing user input to obtain credentials or collect information. During normal system usage, users often provide credentials to various different locations, such as login pages/portals or system dialog boxes. Input capture mechanisms may be transparent to the user (e.g. [Credential API Hooking](https://attack.mitre.org/techniques/T1056/004)) or rely on deceiving the user into providing input into what they believe to be a genuine service (e.g. [Web Portal Capture](https://attack.mitre.org/techniques/T1056/003)).
    InputCapture,

    /// [Data from Information Repositories](https://attack.mitre.org/techniques/T1213)
    ///
    /// Adversaries may leverage information repositories to mine valuable information. Information repositories are tools that allow for storage of information, typically to facilitate collaboration or information sharing between users, and can store a wide variety of data that may aid adversaries in further objectives, or direct access to the target information. Adversaries may also abuse external sharing features to share sensitive documents with recipients outside of the organization. 
    DataFromInformationRepositories,

}
/// [Resource Development](https://attack.mitre.org/tactics/TA0042)'s techniques
pub enum ResourceDevelopment {
    /// [Acquire Infrastructure](https://attack.mitre.org/techniques/T1583)
    ///
    /// Adversaries may buy, lease, or rent infrastructure that can be used during targeting. A wide variety of infrastructure exists for hosting and orchestrating adversary operations. Infrastructure solutions include physical or cloud servers, domains, and third-party web services.(Citation: TrendmicroHideoutsLease) Additionally, botnets are available for rent or purchase.
    AcquireInfrastructure,

    /// [Compromise Infrastructure](https://attack.mitre.org/techniques/T1584)
    ///
    /// Adversaries may compromise third-party infrastructure that can be used during targeting. Infrastructure solutions include physical or cloud servers, domains, and third-party web and DNS services. Instead of buying, leasing, or renting infrastructure an adversary may compromise infrastructure and use it during other phases of the adversary lifecycle.(Citation: Mandiant APT1)(Citation: ICANNDomainNameHijacking)(Citation: Talos DNSpionage Nov 2018)(Citation: FireEye EPS Awakens Part 2) Additionally, adversaries may compromise numerous machines to form a botnet they can leverage.
    CompromiseInfrastructure,

    /// [Compromise Accounts](https://attack.mitre.org/techniques/T1586)
    ///
    /// Adversaries may compromise accounts with services that can be used during targeting. For operations incorporating social engineering, the utilization of an online persona may be important. Rather than creating and cultivating accounts (i.e. [Establish Accounts](https://attack.mitre.org/techniques/T1585)), adversaries may compromise existing accounts. Utilizing an existing persona may engender a level of trust in a potential victim if they have a relationship, or knowledge of, the compromised persona. 
    CompromiseAccounts,

    /// [Stage Capabilities](https://attack.mitre.org/techniques/T1608)
    ///
    /// Adversaries may upload, install, or otherwise set up capabilities that can be used during targeting. To support their operations, an adversary may need to take capabilities they developed ([Develop Capabilities](https://attack.mitre.org/techniques/T1587)) or obtained ([Obtain Capabilities](https://attack.mitre.org/techniques/T1588)) and stage them on infrastructure under their control. These capabilities may be staged on infrastructure that was previously purchased/rented by the adversary ([Acquire Infrastructure](https://attack.mitre.org/techniques/T1583)) or was otherwise compromised by them ([Compromise Infrastructure](https://attack.mitre.org/techniques/T1584)). Capabilities may also be staged on web services, such as GitHub or Pastebin, or on Platform-as-a-Service (PaaS) offerings that enable users to easily provision applications.(Citation: Volexity Ocean Lotus November 2020)(Citation: Dragos Heroku Watering Hole)(Citation: Malwarebytes Heroku Skimmers)(Citation: Netskope GCP Redirection)(Citation: Netskope Cloud Phishing)
    StageCapabilities,

    /// [Establish Accounts](https://attack.mitre.org/techniques/T1585)
    ///
    /// Adversaries may create and cultivate accounts with services that can be used during targeting. Adversaries can create accounts that can be used to build a persona to further operations. Persona development consists of the development of public information, presence, history and appropriate affiliations. This development could be applied to social media, website, or other publicly available information that could be referenced and scrutinized for legitimacy over the course of an operation using that persona or identity.(Citation: NEWSCASTER2014)(Citation: BlackHatRobinSage)
    EstablishAccounts,

    /// [Obtain Capabilities](https://attack.mitre.org/techniques/T1588)
    ///
    /// Adversaries may buy and/or steal capabilities that can be used during targeting. Rather than developing their own capabilities in-house, adversaries may purchase, freely download, or steal them. Activities may include the acquisition of malware, software (including licenses), exploits, certificates, and information relating to vulnerabilities. Adversaries may obtain capabilities to support their operations throughout numerous phases of the adversary lifecycle.
    ObtainCapabilities,

    /// [Acquire Access](https://attack.mitre.org/techniques/T1650)
    ///
    /// Adversaries may purchase or otherwise acquire an existing access to a target system or network. A variety of online services and initial access broker networks are available to sell access to previously compromised systems.(Citation: Microsoft Ransomware as a Service)(Citation: CrowdStrike Access Brokers)(Citation: Krebs Access Brokers Fortune 500) In some cases, adversary groups may form partnerships to share compromised systems with each other.(Citation: CISA Karakurt 2022)
    AcquireAccess,

    /// [Develop Capabilities](https://attack.mitre.org/techniques/T1587)
    ///
    /// Adversaries may build capabilities that can be used during targeting. Rather than purchasing, freely downloading, or stealing capabilities, adversaries may develop their own capabilities in-house. This is the process of identifying development requirements and building solutions such as malware, exploits, and self-signed certificates. Adversaries may develop capabilities to support their operations throughout numerous phases of the adversary lifecycle.(Citation: Mandiant APT1)(Citation: Kaspersky Sofacy)(Citation: Bitdefender StrongPity June 2020)(Citation: Talos Promethium June 2020)
    DevelopCapabilities,

}
/// [Reconnaissance](https://attack.mitre.org/tactics/TA0043)'s techniques
pub enum Reconnaissance {
    /// [Gather Victim Host Information](https://attack.mitre.org/techniques/T1592)
    ///
    /// Adversaries may gather information about the victim's hosts that can be used during targeting. Information about hosts may include a variety of details, including administrative data (ex: name, assigned IP, functionality, etc.) as well as specifics regarding its configuration (ex: operating system, language, etc.).
    GatherVictimHostInformation,

    /// [Search Victim-Owned Websites](https://attack.mitre.org/techniques/T1594)
    ///
    /// Adversaries may search websites owned by the victim for information that can be used during targeting. Victim-owned websites may contain a variety of details, including names of departments/divisions, physical locations, and data about key employees such as names, roles, and contact info (ex: [Email Addresses](https://attack.mitre.org/techniques/T1589/002)). These sites may also have details highlighting business operations and relationships.(Citation: Comparitech Leak)
    SearchVictimOwnedWebsites,

    /// [Gather Victim Identity Information](https://attack.mitre.org/techniques/T1589)
    ///
    /// Adversaries may gather information about the victim's identity that can be used during targeting. Information about identities may include a variety of details, including personal data (ex: employee names, email addresses, etc.) as well as sensitive details such as credentials.
    GatherVictimIdentityInformation,

    /// [Search Open Technical Databases](https://attack.mitre.org/techniques/T1596)
    ///
    /// Adversaries may search freely available technical databases for information about victims that can be used during targeting. Information about victims may be available in online databases and repositories, such as registrations of domains/certificates as well as public collections of network data/artifacts gathered from traffic and/or scans.(Citation: WHOIS)(Citation: DNS Dumpster)(Citation: Circl Passive DNS)(Citation: Medium SSL Cert)(Citation: SSLShopper Lookup)(Citation: DigitalShadows CDN)(Citation: Shodan)
    SearchOpenTechnicalDatabases,

    /// [Active Scanning](https://attack.mitre.org/techniques/T1595)
    ///
    /// Adversaries may execute active reconnaissance scans to gather information that can be used during targeting. Active scans are those where the adversary probes victim infrastructure via network traffic, as opposed to other forms of reconnaissance that do not involve direct interaction.
    ActiveScanning,

    /// [Gather Victim Org Information](https://attack.mitre.org/techniques/T1591)
    ///
    /// Adversaries may gather information about the victim's organization that can be used during targeting. Information about an organization may include a variety of details, including the names of divisions/departments, specifics of business operations, as well as the roles and responsibilities of key employees.
    GatherVictimOrgInformation,

    /// [Gather Victim Network Information](https://attack.mitre.org/techniques/T1590)
    ///
    /// Adversaries may gather information about the victim's networks that can be used during targeting. Information about networks may include a variety of details, including administrative data (ex: IP ranges, domain names, etc.) as well as specifics regarding its topology and operations.
    GatherVictimNetworkInformation,

    /// [Search Open Websites/Domains](https://attack.mitre.org/techniques/T1593)
    ///
    /// Adversaries may search freely available websites and/or domains for information about victims that can be used during targeting. Information about victims may be available in various online sites, such as social media, new sites, or those hosting information about business operations such as hiring or requested/rewarded contracts.(Citation: Cyware Social Media)(Citation: SecurityTrails Google Hacking)(Citation: ExploitDB GoogleHacking)
    SearchOpenWebsitesDomains,

    /// [Search Closed Sources](https://attack.mitre.org/techniques/T1597)
    ///
    /// Adversaries may search and gather information about victims from closed sources that can be used during targeting. Information about victims may be available for purchase from reputable private sources and databases, such as paid subscriptions to feeds of technical/threat intelligence data.(Citation: D3Secutrity CTI Feeds) Adversaries may also purchase information from less-reputable sources such as dark web or cybercrime blackmarkets.(Citation: ZDNET Selling Data)
    SearchClosedSources,

    /// [Phishing for Information](https://attack.mitre.org/techniques/T1598)
    ///
    /// Adversaries may send phishing messages to elicit sensitive information that can be used during targeting. Phishing for information is an attempt to trick targets into divulging information, frequently credentials or other actionable information. Phishing for information is different from [Phishing](https://attack.mitre.org/techniques/T1566) in that the objective is gathering data from the victim rather than executing malicious code.
    PhishingForInformation,

}
/// [Command and Control](https://attack.mitre.org/tactics/TA0011)'s techniques
pub enum CommandAndControl {
    /// [Application Layer Protocol](https://attack.mitre.org/techniques/T1071)
    ///
    /// Adversaries may communicate using OSI application layer protocols to avoid detection/network filtering by blending in with existing traffic. Commands to the remote system, and often the results of those commands, will be embedded within the protocol traffic between the client and server. 
    ApplicationLayerProtocol,

    /// [Remote Access Software](https://attack.mitre.org/techniques/T1219)
    ///
    /// An adversary may use legitimate desktop support and remote access software to establish an interactive command and control channel to target systems within networks. These services, such as `VNC`, `Team Viewer`, `AnyDesk`, `ScreenConnect`, `LogMein`, `AmmyyAdmin`, and other remote monitoring and management (RMM) tools, are commonly used as legitimate technical support software and may be allowed by application control within a target environment.(Citation: Symantec Living off the Land)(Citation: CrowdStrike 2015 Global Threat Report)(Citation: CrySyS Blog TeamSpy)
    RemoteAccessSoftware,

    /// [Content Injection](https://attack.mitre.org/techniques/T1659)
    ///
    /// Adversaries may gain access and continuously communicate with victims by injecting malicious content into systems through online network traffic. Rather than luring victims to malicious payloads hosted on a compromised website (i.e., [Drive-by Target](https://attack.mitre.org/techniques/T1608/004) followed by [Drive-by Compromise](https://attack.mitre.org/techniques/T1189)), adversaries may initially access victims through compromised data-transfer channels where they can manipulate traffic and/or inject their own content. These compromised online network channels may also be used to deliver additional payloads (i.e., [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105)) and other data to already compromised systems.(Citation: ESET MoustachedBouncer)
    ContentInjection,

    /// [Traffic Signaling](https://attack.mitre.org/techniques/T1205)
    ///
    /// Adversaries may use traffic signaling to hide open ports or other malicious functionality used for persistence or command and control. Traffic signaling involves the use of a magic value or sequence that must be sent to a system to trigger a special response, such as opening a closed port or executing a malicious task. This may take the form of sending a series of packets with certain characteristics before a port will be opened that the adversary can use for command and control. Usually this series of packets consists of attempted connections to a predefined sequence of closed ports (i.e. [Port Knocking](https://attack.mitre.org/techniques/T1205/001)), but can involve unusual flags, specific strings, or other unique characteristics. After the sequence is completed, opening a port may be accomplished by the host-based firewall, but could also be implemented by custom software.
    TrafficSignaling,

    /// [Protocol Tunneling](https://attack.mitre.org/techniques/T1572)
    ///
    /// Adversaries may tunnel network communications to and from a victim system within a separate protocol to avoid detection/network filtering and/or enable access to otherwise unreachable systems. Tunneling involves explicitly encapsulating a protocol within another. This behavior may conceal malicious traffic by blending in with existing traffic and/or provide an outer layer of encryption (similar to a VPN). Tunneling could also enable routing of network packets that would otherwise not reach their intended destination, such as SMB, RDP, or other traffic that would be filtered by network appliances or not routed over the Internet. 
    ProtocolTunneling,

    /// [Communication Through Removable Media](https://attack.mitre.org/techniques/T1092)
    ///
    /// Adversaries can perform command and control between compromised hosts on potentially disconnected networks using removable media to transfer commands from system to system. Both systems would need to be compromised, with the likelihood that an Internet-connected system was compromised first and the second through lateral movement by [Replication Through Removable Media](https://attack.mitre.org/techniques/T1091). Commands and files would be relayed from the disconnected system to the Internet-connected system to which the adversary has direct access.
    CommunicationThroughRemovableMedia,

    /// [Proxy](https://attack.mitre.org/techniques/T1090)
    ///
    /// Adversaries may use a connection proxy to direct network traffic between systems or act as an intermediary for network communications to a command and control server to avoid direct connections to their infrastructure. Many tools exist that enable traffic redirection through proxies or port redirection, including [HTRAN](https://attack.mitre.org/software/S0040), ZXProxy, and ZXPortMap. (Citation: Trend Micro APT Attack Tools) Adversaries use these types of proxies to manage command and control communications, reduce the number of simultaneous outbound network connections, provide resiliency in the face of connection loss, or to ride over existing trusted communications paths between victims to avoid suspicion. Adversaries may chain together multiple proxies to further disguise the source of malicious traffic.
    Proxy,

    /// [Dynamic Resolution](https://attack.mitre.org/techniques/T1568)
    ///
    /// Adversaries may dynamically establish connections to command and control infrastructure to evade common detections and remediations. This may be achieved by using malware that shares a common algorithm with the infrastructure the adversary uses to receive the malware's communications. These calculations can be used to dynamically adjust parameters such as the domain name, IP address, or port number the malware uses for command and control.
    DynamicResolution,

    /// [Web Service](https://attack.mitre.org/techniques/T1102)
    ///
    /// Adversaries may use an existing, legitimate external Web service as a means for relaying data to/from a compromised system. Popular websites and social media acting as a mechanism for C2 may give a significant amount of cover due to the likelihood that hosts within a network are already communicating with them prior to a compromise. Using common services, such as those offered by Google or Twitter, makes it easier for adversaries to hide in expected noise. Web service providers commonly use SSL/TLS encryption, giving adversaries an added level of protection.
    WebService,

    /// [Multi-Stage Channels](https://attack.mitre.org/techniques/T1104)
    ///
    /// Adversaries may create multiple stages for command and control that are employed under different conditions or for certain functions. Use of multiple stages may obfuscate the command and control channel to make detection more difficult.
    MultiStageChannels,

    /// [Data Obfuscation](https://attack.mitre.org/techniques/T1001)
    ///
    /// Adversaries may obfuscate command and control traffic to make it more difficult to detect. Command and control (C2) communications are hidden (but not necessarily encrypted) in an attempt to make the content more difficult to discover or decipher and to make the communication less conspicuous and hide commands from being seen. This encompasses many methods, such as adding junk data to protocol traffic, using steganography, or impersonating legitimate protocols. 
    DataObfuscation,

    /// [Non-Standard Port](https://attack.mitre.org/techniques/T1571)
    ///
    /// Adversaries may communicate using a protocol and port pairing that are typically not associated. For example, HTTPS over port 8088(Citation: Symantec Elfin Mar 2019) or port 587(Citation: Fortinet Agent Tesla April 2018) as opposed to the traditional port 443. Adversaries may make changes to the standard port used by a protocol to bypass filtering or muddle analysis/parsing of network data.
    NonStandardPort,

    /// [Encrypted Channel](https://attack.mitre.org/techniques/T1573)
    ///
    /// Adversaries may employ a known encryption algorithm to conceal command and control traffic rather than relying on any inherent protections provided by a communication protocol. Despite the use of a secure algorithm, these implementations may be vulnerable to reverse engineering if secret keys are encoded and/or generated within malware samples/configuration files.
    EncryptedChannel,

    /// [Non-Application Layer Protocol](https://attack.mitre.org/techniques/T1095)
    ///
    /// Adversaries may use an OSI non-application layer protocol for communication between host and C2 server or among infected hosts within a network. The list of possible protocols is extensive.(Citation: Wikipedia OSI) Specific examples include use of network layer protocols, such as the Internet Control Message Protocol (ICMP), transport layer protocols, such as the User Datagram Protocol (UDP), session layer protocols, such as Socket Secure (SOCKS), as well as redirected/tunneled protocols, such as Serial over LAN (SOL).
    NonApplicationLayerProtocol,

    /// [Data Encoding](https://attack.mitre.org/techniques/T1132)
    ///
    /// Adversaries may encode data to make the content of command and control traffic more difficult to detect. Command and control (C2) information can be encoded using a standard data encoding system. Use of data encoding may adhere to existing protocol specifications and includes use of ASCII, Unicode, Base64, MIME, or other binary-to-text and character encoding systems.(Citation: Wikipedia Binary-to-text Encoding) (Citation: Wikipedia Character Encoding) Some data encoding systems may also result in data compression, such as gzip.
    DataEncoding,

    /// [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105)
    ///
    /// Adversaries may transfer tools or other files from an external system into a compromised environment. Tools or files may be copied from an external adversary-controlled system to the victim network through the command and control channel or through alternate protocols such as [ftp](https://attack.mitre.org/software/S0095). Once present, adversaries may also transfer/spread tools between victim devices within a compromised environment (i.e. [Lateral Tool Transfer](https://attack.mitre.org/techniques/T1570)). 
    IngressToolTransfer,

    /// [Fallback Channels](https://attack.mitre.org/techniques/T1008)
    ///
    /// Adversaries may use fallback or alternate communication channels if the primary channel is compromised or inaccessible in order to maintain reliable command and control and to avoid data transfer thresholds.
    FallbackChannels,

}
/// [Initial Access](https://attack.mitre.org/tactics/TA0001)'s techniques
pub enum InitialAccess {
    /// [External Remote Services](https://attack.mitre.org/techniques/T1133)
    ///
    /// Adversaries may leverage external-facing remote services to initially access and/or persist within a network. Remote services such as VPNs, Citrix, and other access mechanisms allow users to connect to internal enterprise network resources from external locations. There are often remote service gateways that manage connections and credential authentication for these services. Services such as [Windows Remote Management](https://attack.mitre.org/techniques/T1021/006) and [VNC](https://attack.mitre.org/techniques/T1021/005) can also be used externally.(Citation: MacOS VNC software for Remote Desktop)
    ExternalRemoteServices,

    /// [Replication Through Removable Media](https://attack.mitre.org/techniques/T1091)
    ///
    /// Adversaries may move onto systems, possibly those on disconnected or air-gapped networks, by copying malware to removable media and taking advantage of Autorun features when the media is inserted into a system and executes. In the case of Lateral Movement, this may occur through modification of executable files stored on removable media or by copying malware and renaming it to look like a legitimate file to trick users into executing it on a separate system. In the case of Initial Access, this may occur through manual manipulation of the media, modification of systems used to initially format the media, or modification to the media's firmware itself.
    ReplicationThroughRemovableMedia,

    /// [Supply Chain Compromise](https://attack.mitre.org/techniques/T1195)
    ///
    /// Adversaries may manipulate products or product delivery mechanisms prior to receipt by a final consumer for the purpose of data or system compromise.
    SupplyChainCompromise,

    /// [Exploit Public-Facing Application](https://attack.mitre.org/techniques/T1190)
    ///
    /// Adversaries may attempt to exploit a weakness in an Internet-facing host or system to initially access a network. The weakness in the system can be a software bug, a temporary glitch, or a misconfiguration.
    ExploitPublicFacingApplication,

    /// [Content Injection](https://attack.mitre.org/techniques/T1659)
    ///
    /// Adversaries may gain access and continuously communicate with victims by injecting malicious content into systems through online network traffic. Rather than luring victims to malicious payloads hosted on a compromised website (i.e., [Drive-by Target](https://attack.mitre.org/techniques/T1608/004) followed by [Drive-by Compromise](https://attack.mitre.org/techniques/T1189)), adversaries may initially access victims through compromised data-transfer channels where they can manipulate traffic and/or inject their own content. These compromised online network channels may also be used to deliver additional payloads (i.e., [Ingress Tool Transfer](https://attack.mitre.org/techniques/T1105)) and other data to already compromised systems.(Citation: ESET MoustachedBouncer)
    ContentInjection,

    /// [Trusted Relationship](https://attack.mitre.org/techniques/T1199)
    ///
    /// Adversaries may breach or otherwise leverage organizations who have access to intended victims. Access through trusted third party relationship abuses an existing connection that may not be protected or receives less scrutiny than standard mechanisms of gaining access to a network.
    TrustedRelationship,

    /// [Phishing](https://attack.mitre.org/techniques/T1566)
    ///
    /// Adversaries may send phishing messages to gain access to victim systems. All forms of phishing are electronically delivered social engineering. Phishing can be targeted, known as spearphishing. In spearphishing, a specific individual, company, or industry will be targeted by the adversary. More generally, adversaries can conduct non-targeted phishing, such as in mass malware spam campaigns.
    Phishing,

    /// [Valid Accounts](https://attack.mitre.org/techniques/T1078)
    ///
    /// Adversaries may obtain and abuse credentials of existing accounts as a means of gaining Initial Access, Persistence, Privilege Escalation, or Defense Evasion. Compromised credentials may be used to bypass access controls placed on various resources on systems within the network and may even be used for persistent access to remote systems and externally available services, such as VPNs, Outlook Web Access, network devices, and remote desktop.(Citation: volexity_0day_sophos_FW) Compromised credentials may also grant an adversary increased privilege to specific systems or access to restricted areas of the network. Adversaries may choose not to use malware or tools in conjunction with the legitimate access those credentials provide to make it harder to detect their presence.
    ValidAccounts,

    /// [Hardware Additions](https://attack.mitre.org/techniques/T1200)
    ///
    /// Adversaries may introduce computer accessories, networking hardware, or other computing devices into a system or network that can be used as a vector to gain access. Rather than just connecting and distributing payloads via removable storage (i.e. [Replication Through Removable Media](https://attack.mitre.org/techniques/T1091)), more robust hardware additions can be used to introduce new functionalities and/or features into a system that can then be abused.
    HardwareAdditions,

    /// [Drive-by Compromise](https://attack.mitre.org/techniques/T1189)
    ///
    /// Adversaries may gain access to a system through a user visiting a website over the normal course of browsing. With this technique, the user's web browser is typically targeted for exploitation, but adversaries may also use compromised websites for non-exploitation behavior such as acquiring [Application Access Token](https://attack.mitre.org/techniques/T1550/001).
    DriveByCompromise,

}
impl TryFrom<AttackTechnique> for Tactic {
    type Error = InvalidArgumentError;
    fn try_from(value: AttackTechnique) -> Result<Self, Self::Error> {
        let AttackTechnique { tactic, technique } = value;
        Ok(match tactic {
            0006 => Self::CredentialAccess(CredentialAccess::try_from(technique)?),
            0002 => Self::Execution(Execution::try_from(technique)?),
            0040 => Self::Impact(Impact::try_from(technique)?),
            0003 => Self::Persistence(Persistence::try_from(technique)?),
            0004 => Self::PrivilegeEscalation(PrivilegeEscalation::try_from(technique)?),
            0008 => Self::LateralMovement(LateralMovement::try_from(technique)?),
            0005 => Self::DefenseEvasion(DefenseEvasion::try_from(technique)?),
            0010 => Self::Exfiltration(Exfiltration::try_from(technique)?),
            0007 => Self::Discovery(Discovery::try_from(technique)?),
            0009 => Self::Collection(Collection::try_from(technique)?),
            0042 => Self::ResourceDevelopment(ResourceDevelopment::try_from(technique)?),
            0043 => Self::Reconnaissance(Reconnaissance::try_from(technique)?),
            0011 => Self::CommandAndControl(CommandAndControl::try_from(technique)?),
            0001 => Self::InitialAccess(InitialAccess::try_from(technique)?),
            _ => return Err(InvalidArgumentError::InvalidMitreTactic(tactic)),
        })
    }
}
impl TryFrom<u32> for CredentialAccess {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1557 => Self::AdversaryInTheMiddle,
            1003 => Self::OsCredentialDumping,
            1539 => Self::StealWebSessionCookie,
            1040 => Self::NetworkSniffing,
            1558 => Self::StealOrForgeKerberosTickets,
            1555 => Self::CredentialsFromPasswordStores,
            1552 => Self::UnsecuredCredentials,
            1649 => Self::StealOrForgeAuthenticationCertificates,
            1528 => Self::StealApplicationAccessToken,
            1606 => Self::ForgeWebCredentials,
            1621 => Self::MultiFactorAuthenticationRequestGeneration,
            1212 => Self::ExploitationForCredentialAccess,
            1110 => Self::BruteForce,
            1187 => Self::ForcedAuthentication,
            1056 => Self::InputCapture,
            1111 => Self::MultiFactorAuthenticationInterception,
            1556 => Self::ModifyAuthenticationProcess,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0006, value)),
        })
    }
}
impl TryFrom<u32> for Execution {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1047 => Self::WindowsManagementInstrumentation,
            1129 => Self::SharedModules,
            1053 => Self::ScheduledTaskJob,
            1106 => Self::NativeApi,
            1610 => Self::DeployContainer,
            1059 => Self::CommandAndScriptingInterpreter,
            1609 => Self::ContainerAdministrationCommand,
            1204 => Self::UserExecution,
            1072 => Self::SoftwareDeploymentTools,
            1559 => Self::InterProcessCommunication,
            1203 => Self::ExploitationForClientExecution,
            1569 => Self::SystemServices,
            1651 => Self::CloudAdministrationCommand,
            1648 => Self::ServerlessExecution,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0002, value)),
        })
    }
}
impl TryFrom<u32> for Impact {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1561 => Self::DiskWipe,
            1489 => Self::ServiceStop,
            1491 => Self::Defacement,
            1657 => Self::FinancialTheft,
            1565 => Self::DataManipulation,
            1531 => Self::AccountAccessRemoval,
            1486 => Self::DataEncryptedForImpact,
            1499 => Self::EndpointDenialOfService,
            1496 => Self::ResourceHijacking,
            1485 => Self::DataDestruction,
            1498 => Self::NetworkDenialOfService,
            1495 => Self::FirmwareCorruption,
            1490 => Self::InhibitSystemRecovery,
            1529 => Self::SystemShutdownReboot,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0040, value)),
        })
    }
}
impl TryFrom<u32> for Persistence {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1037 => Self::BootOrLogonInitializationScripts,
            1543 => Self::CreateOrModifySystemProcess,
            1133 => Self::ExternalRemoteServices,
            1547 => Self::BootOrLogonAutostartExecution,
            1137 => Self::OfficeApplicationStartup,
            1053 => Self::ScheduledTaskJob,
            1176 => Self::BrowserExtensions,
            1205 => Self::TrafficSignaling,
            1525 => Self::ImplantInternalImage,
            1542 => Self::PreOsBoot,
            1554 => Self::CompromiseClientSoftwareBinary,
            1098 => Self::AccountManipulation,
            1574 => Self::HijackExecutionFlow,
            1078 => Self::ValidAccounts,
            1546 => Self::EventTriggeredExecution,
            1197 => Self::BitsJobs,
            1505 => Self::ServerSoftwareComponent,
            1136 => Self::CreateAccount,
            1653 => Self::PowerSettings,
            1556 => Self::ModifyAuthenticationProcess,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0003, value)),
        })
    }
}
impl TryFrom<u32> for PrivilegeEscalation {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1037 => Self::BootOrLogonInitializationScripts,
            1543 => Self::CreateOrModifySystemProcess,
            1547 => Self::BootOrLogonAutostartExecution,
            1053 => Self::ScheduledTaskJob,
            1055 => Self::ProcessInjection,
            1611 => Self::EscapeToHost,
            1548 => Self::AbuseElevationControlMechanism,
            1098 => Self::AccountManipulation,
            1574 => Self::HijackExecutionFlow,
            1078 => Self::ValidAccounts,
            1068 => Self::ExploitationForPrivilegeEscalation,
            1546 => Self::EventTriggeredExecution,
            1134 => Self::AccessTokenManipulation,
            1484 => Self::DomainPolicyModification,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0004, value)),
        })
    }
}
impl TryFrom<u32> for LateralMovement {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1080 => Self::TaintSharedContent,
            1091 => Self::ReplicationThroughRemovableMedia,
            1550 => Self::UseAlternateAuthenticationMaterial,
            1021 => Self::RemoteServices,
            1563 => Self::RemoteServiceSessionHijacking,
            1072 => Self::SoftwareDeploymentTools,
            1210 => Self::ExploitationOfRemoteServices,
            1534 => Self::InternalSpearphishing,
            1570 => Self::LateralToolTransfer,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0008, value)),
        })
    }
}
impl TryFrom<u32> for DefenseEvasion {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1006 => Self::DirectVolumeAccess,
            1014 => Self::Rootkit,
            1578 => Self::ModifyCloudComputeInfrastructure,
            1600 => Self::WeakenEncryption,
            1564 => Self::HideArtifacts,
            1202 => Self::IndirectCommandExecution,
            1140 => Self::DeobfuscateDecodeFilesOrInformation,
            1562 => Self::ImpairDefenses,
            1036 => Self::Masquerading,
            1055 => Self::ProcessInjection,
            1205 => Self::TrafficSignaling,
            1218 => Self::SystemBinaryProxyExecution,
            1620 => Self::ReflectiveCodeLoading,
            1550 => Self::UseAlternateAuthenticationMaterial,
            1207 => Self::RogueDomainController,
            1610 => Self::DeployContainer,
            1112 => Self::ModifyRegistry,
            1535 => Self::UnusedUnsupportedCloudRegions,
            1222 => Self::FileAndDirectoryPermissionsModification,
            1548 => Self::AbuseElevationControlMechanism,
            1070 => Self::IndicatorRemoval,
            1647 => Self::PlistFileModification,
            1542 => Self::PreOsBoot,
            1612 => Self::BuildImageOnHost,
            1497 => Self::VirtualizationSandboxEvasion,
            1480 => Self::ExecutionGuardrails,
            1601 => Self::ModifySystemImage,
            1574 => Self::HijackExecutionFlow,
            1078 => Self::ValidAccounts,
            1027 => Self::ObfuscatedFilesOrInformation,
            1599 => Self::NetworkBoundaryBridging,
            1553 => Self::SubvertTrustControls,
            1197 => Self::BitsJobs,
            1656 => Self::Impersonation,
            1221 => Self::TemplateInjection,
            1134 => Self::AccessTokenManipulation,
            1622 => Self::DebuggerEvasion,
            1484 => Self::DomainPolicyModification,
            1220 => Self::XslScriptProcessing,
            1556 => Self::ModifyAuthenticationProcess,
            1216 => Self::SystemScriptProxyExecution,
            1211 => Self::ExploitationForDefenseEvasion,
            1127 => Self::TrustedDeveloperUtilitiesProxyExecution,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0005, value)),
        })
    }
}
impl TryFrom<u32> for Exfiltration {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1567 => Self::ExfiltrationOverWebService,
            1029 => Self::ScheduledTransfer,
            1011 => Self::ExfiltrationOverOtherNetworkMedium,
            1020 => Self::AutomatedExfiltration,
            1041 => Self::ExfiltrationOverC2Channel,
            1048 => Self::ExfiltrationOverAlternativeProtocol,
            1030 => Self::DataTransferSizeLimits,
            1537 => Self::TransferDataToCloudAccount,
            1052 => Self::ExfiltrationOverPhysicalMedium,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0010, value)),
        })
    }
}
impl TryFrom<u32> for Discovery {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1033 => Self::SystemOwnerUserDiscovery,
            1613 => Self::ContainerAndResourceDiscovery,
            1069 => Self::PermissionGroupsDiscovery,
            1615 => Self::GroupPolicyDiscovery,
            1652 => Self::DeviceDriverDiscovery,
            1007 => Self::SystemServiceDiscovery,
            1040 => Self::NetworkSniffing,
            1135 => Self::NetworkShareDiscovery,
            1120 => Self::PeripheralDeviceDiscovery,
            1082 => Self::SystemInformationDiscovery,
            1010 => Self::ApplicationWindowDiscovery,
            1580 => Self::CloudInfrastructureDiscovery,
            1217 => Self::BrowserInformationDiscovery,
            1016 => Self::SystemNetworkConfigurationDiscovery,
            1087 => Self::AccountDiscovery,
            1482 => Self::DomainTrustDiscovery,
            1083 => Self::FileAndDirectoryDiscovery,
            1049 => Self::SystemNetworkConnectionsDiscovery,
            1497 => Self::VirtualizationSandboxEvasion,
            1619 => Self::CloudStorageObjectDiscovery,
            1654 => Self::LogEnumeration,
            1057 => Self::ProcessDiscovery,
            1201 => Self::PasswordPolicyDiscovery,
            1012 => Self::QueryRegistry,
            1614 => Self::SystemLocationDiscovery,
            1526 => Self::CloudServiceDiscovery,
            1018 => Self::RemoteSystemDiscovery,
            1046 => Self::NetworkServiceDiscovery,
            1518 => Self::SoftwareDiscovery,
            1538 => Self::CloudServiceDashboard,
            1622 => Self::DebuggerEvasion,
            1124 => Self::SystemTimeDiscovery,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0007, value)),
        })
    }
}
impl TryFrom<u32> for Collection {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1113 => Self::ScreenCapture,
            1557 => Self::AdversaryInTheMiddle,
            1602 => Self::DataFromConfigurationRepository,
            1123 => Self::AudioCapture,
            1114 => Self::EmailCollection,
            1025 => Self::DataFromRemovableMedia,
            1119 => Self::AutomatedCollection,
            1115 => Self::ClipboardData,
            1530 => Self::DataFromCloudStorage,
            1005 => Self::DataFromLocalSystem,
            1560 => Self::ArchiveCollectedData,
            1185 => Self::BrowserSessionHijacking,
            1125 => Self::VideoCapture,
            1074 => Self::DataStaged,
            1039 => Self::DataFromNetworkSharedDrive,
            1056 => Self::InputCapture,
            1213 => Self::DataFromInformationRepositories,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0009, value)),
        })
    }
}
impl TryFrom<u32> for ResourceDevelopment {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1583 => Self::AcquireInfrastructure,
            1584 => Self::CompromiseInfrastructure,
            1586 => Self::CompromiseAccounts,
            1608 => Self::StageCapabilities,
            1585 => Self::EstablishAccounts,
            1588 => Self::ObtainCapabilities,
            1650 => Self::AcquireAccess,
            1587 => Self::DevelopCapabilities,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0042, value)),
        })
    }
}
impl TryFrom<u32> for Reconnaissance {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1592 => Self::GatherVictimHostInformation,
            1594 => Self::SearchVictimOwnedWebsites,
            1589 => Self::GatherVictimIdentityInformation,
            1596 => Self::SearchOpenTechnicalDatabases,
            1595 => Self::ActiveScanning,
            1591 => Self::GatherVictimOrgInformation,
            1590 => Self::GatherVictimNetworkInformation,
            1593 => Self::SearchOpenWebsitesDomains,
            1597 => Self::SearchClosedSources,
            1598 => Self::PhishingForInformation,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0043, value)),
        })
    }
}
impl TryFrom<u32> for CommandAndControl {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1071 => Self::ApplicationLayerProtocol,
            1219 => Self::RemoteAccessSoftware,
            1659 => Self::ContentInjection,
            1205 => Self::TrafficSignaling,
            1572 => Self::ProtocolTunneling,
            1092 => Self::CommunicationThroughRemovableMedia,
            1090 => Self::Proxy,
            1568 => Self::DynamicResolution,
            1102 => Self::WebService,
            1104 => Self::MultiStageChannels,
            1001 => Self::DataObfuscation,
            1571 => Self::NonStandardPort,
            1573 => Self::EncryptedChannel,
            1095 => Self::NonApplicationLayerProtocol,
            1132 => Self::DataEncoding,
            1105 => Self::IngressToolTransfer,
            1008 => Self::FallbackChannels,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0011, value)),
        })
    }
}
impl TryFrom<u32> for InitialAccess {
    type Error = InvalidArgumentError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1133 => Self::ExternalRemoteServices,
            1091 => Self::ReplicationThroughRemovableMedia,
            1195 => Self::SupplyChainCompromise,
            1190 => Self::ExploitPublicFacingApplication,
            1659 => Self::ContentInjection,
            1199 => Self::TrustedRelationship,
            1566 => Self::Phishing,
            1078 => Self::ValidAccounts,
            1200 => Self::HardwareAdditions,
            1189 => Self::DriveByCompromise,
            _ => return Err(InvalidArgumentError::InvalidMitreTechnique(0001, value)),
        })
    }
}
impl From<Tactic> for AttackTechnique {
    fn from(value: Tactic) -> Self {
        let (tactic, technique) = match value {
            Tactic::CredentialAccess(technique) => (0006, technique.into()),
            Tactic::Execution(technique) => (0002, technique.into()),
            Tactic::Impact(technique) => (0040, technique.into()),
            Tactic::Persistence(technique) => (0003, technique.into()),
            Tactic::PrivilegeEscalation(technique) => (0004, technique.into()),
            Tactic::LateralMovement(technique) => (0008, technique.into()),
            Tactic::DefenseEvasion(technique) => (0005, technique.into()),
            Tactic::Exfiltration(technique) => (0010, technique.into()),
            Tactic::Discovery(technique) => (0007, technique.into()),
            Tactic::Collection(technique) => (0009, technique.into()),
            Tactic::ResourceDevelopment(technique) => (0042, technique.into()),
            Tactic::Reconnaissance(technique) => (0043, technique.into()),
            Tactic::CommandAndControl(technique) => (0011, technique.into()),
            Tactic::InitialAccess(technique) => (0001, technique.into()),
        };
        AttackTechnique { tactic, technique }
    }
}
impl From<CredentialAccess> for u32 {
    fn from(value: CredentialAccess) -> Self {
        match value {
            CredentialAccess::AdversaryInTheMiddle => 1557,
            CredentialAccess::OsCredentialDumping => 1003,
            CredentialAccess::StealWebSessionCookie => 1539,
            CredentialAccess::NetworkSniffing => 1040,
            CredentialAccess::StealOrForgeKerberosTickets => 1558,
            CredentialAccess::CredentialsFromPasswordStores => 1555,
            CredentialAccess::UnsecuredCredentials => 1552,
            CredentialAccess::StealOrForgeAuthenticationCertificates => 1649,
            CredentialAccess::StealApplicationAccessToken => 1528,
            CredentialAccess::ForgeWebCredentials => 1606,
            CredentialAccess::MultiFactorAuthenticationRequestGeneration => 1621,
            CredentialAccess::ExploitationForCredentialAccess => 1212,
            CredentialAccess::BruteForce => 1110,
            CredentialAccess::ForcedAuthentication => 1187,
            CredentialAccess::InputCapture => 1056,
            CredentialAccess::MultiFactorAuthenticationInterception => 1111,
            CredentialAccess::ModifyAuthenticationProcess => 1556,
        }
    }
}
impl From<Execution> for u32 {
    fn from(value: Execution) -> Self {
        match value {
            Execution::WindowsManagementInstrumentation => 1047,
            Execution::SharedModules => 1129,
            Execution::ScheduledTaskJob => 1053,
            Execution::NativeApi => 1106,
            Execution::DeployContainer => 1610,
            Execution::CommandAndScriptingInterpreter => 1059,
            Execution::ContainerAdministrationCommand => 1609,
            Execution::UserExecution => 1204,
            Execution::SoftwareDeploymentTools => 1072,
            Execution::InterProcessCommunication => 1559,
            Execution::ExploitationForClientExecution => 1203,
            Execution::SystemServices => 1569,
            Execution::CloudAdministrationCommand => 1651,
            Execution::ServerlessExecution => 1648,
        }
    }
}
impl From<Impact> for u32 {
    fn from(value: Impact) -> Self {
        match value {
            Impact::DiskWipe => 1561,
            Impact::ServiceStop => 1489,
            Impact::Defacement => 1491,
            Impact::FinancialTheft => 1657,
            Impact::DataManipulation => 1565,
            Impact::AccountAccessRemoval => 1531,
            Impact::DataEncryptedForImpact => 1486,
            Impact::EndpointDenialOfService => 1499,
            Impact::ResourceHijacking => 1496,
            Impact::DataDestruction => 1485,
            Impact::NetworkDenialOfService => 1498,
            Impact::FirmwareCorruption => 1495,
            Impact::InhibitSystemRecovery => 1490,
            Impact::SystemShutdownReboot => 1529,
        }
    }
}
impl From<Persistence> for u32 {
    fn from(value: Persistence) -> Self {
        match value {
            Persistence::BootOrLogonInitializationScripts => 1037,
            Persistence::CreateOrModifySystemProcess => 1543,
            Persistence::ExternalRemoteServices => 1133,
            Persistence::BootOrLogonAutostartExecution => 1547,
            Persistence::OfficeApplicationStartup => 1137,
            Persistence::ScheduledTaskJob => 1053,
            Persistence::BrowserExtensions => 1176,
            Persistence::TrafficSignaling => 1205,
            Persistence::ImplantInternalImage => 1525,
            Persistence::PreOsBoot => 1542,
            Persistence::CompromiseClientSoftwareBinary => 1554,
            Persistence::AccountManipulation => 1098,
            Persistence::HijackExecutionFlow => 1574,
            Persistence::ValidAccounts => 1078,
            Persistence::EventTriggeredExecution => 1546,
            Persistence::BitsJobs => 1197,
            Persistence::ServerSoftwareComponent => 1505,
            Persistence::CreateAccount => 1136,
            Persistence::PowerSettings => 1653,
            Persistence::ModifyAuthenticationProcess => 1556,
        }
    }
}
impl From<PrivilegeEscalation> for u32 {
    fn from(value: PrivilegeEscalation) -> Self {
        match value {
            PrivilegeEscalation::BootOrLogonInitializationScripts => 1037,
            PrivilegeEscalation::CreateOrModifySystemProcess => 1543,
            PrivilegeEscalation::BootOrLogonAutostartExecution => 1547,
            PrivilegeEscalation::ScheduledTaskJob => 1053,
            PrivilegeEscalation::ProcessInjection => 1055,
            PrivilegeEscalation::EscapeToHost => 1611,
            PrivilegeEscalation::AbuseElevationControlMechanism => 1548,
            PrivilegeEscalation::AccountManipulation => 1098,
            PrivilegeEscalation::HijackExecutionFlow => 1574,
            PrivilegeEscalation::ValidAccounts => 1078,
            PrivilegeEscalation::ExploitationForPrivilegeEscalation => 1068,
            PrivilegeEscalation::EventTriggeredExecution => 1546,
            PrivilegeEscalation::AccessTokenManipulation => 1134,
            PrivilegeEscalation::DomainPolicyModification => 1484,
        }
    }
}
impl From<LateralMovement> for u32 {
    fn from(value: LateralMovement) -> Self {
        match value {
            LateralMovement::TaintSharedContent => 1080,
            LateralMovement::ReplicationThroughRemovableMedia => 1091,
            LateralMovement::UseAlternateAuthenticationMaterial => 1550,
            LateralMovement::RemoteServices => 1021,
            LateralMovement::RemoteServiceSessionHijacking => 1563,
            LateralMovement::SoftwareDeploymentTools => 1072,
            LateralMovement::ExploitationOfRemoteServices => 1210,
            LateralMovement::InternalSpearphishing => 1534,
            LateralMovement::LateralToolTransfer => 1570,
        }
    }
}
impl From<DefenseEvasion> for u32 {
    fn from(value: DefenseEvasion) -> Self {
        match value {
            DefenseEvasion::DirectVolumeAccess => 1006,
            DefenseEvasion::Rootkit => 1014,
            DefenseEvasion::ModifyCloudComputeInfrastructure => 1578,
            DefenseEvasion::WeakenEncryption => 1600,
            DefenseEvasion::HideArtifacts => 1564,
            DefenseEvasion::IndirectCommandExecution => 1202,
            DefenseEvasion::DeobfuscateDecodeFilesOrInformation => 1140,
            DefenseEvasion::ImpairDefenses => 1562,
            DefenseEvasion::Masquerading => 1036,
            DefenseEvasion::ProcessInjection => 1055,
            DefenseEvasion::TrafficSignaling => 1205,
            DefenseEvasion::SystemBinaryProxyExecution => 1218,
            DefenseEvasion::ReflectiveCodeLoading => 1620,
            DefenseEvasion::UseAlternateAuthenticationMaterial => 1550,
            DefenseEvasion::RogueDomainController => 1207,
            DefenseEvasion::DeployContainer => 1610,
            DefenseEvasion::ModifyRegistry => 1112,
            DefenseEvasion::UnusedUnsupportedCloudRegions => 1535,
            DefenseEvasion::FileAndDirectoryPermissionsModification => 1222,
            DefenseEvasion::AbuseElevationControlMechanism => 1548,
            DefenseEvasion::IndicatorRemoval => 1070,
            DefenseEvasion::PlistFileModification => 1647,
            DefenseEvasion::PreOsBoot => 1542,
            DefenseEvasion::BuildImageOnHost => 1612,
            DefenseEvasion::VirtualizationSandboxEvasion => 1497,
            DefenseEvasion::ExecutionGuardrails => 1480,
            DefenseEvasion::ModifySystemImage => 1601,
            DefenseEvasion::HijackExecutionFlow => 1574,
            DefenseEvasion::ValidAccounts => 1078,
            DefenseEvasion::ObfuscatedFilesOrInformation => 1027,
            DefenseEvasion::NetworkBoundaryBridging => 1599,
            DefenseEvasion::SubvertTrustControls => 1553,
            DefenseEvasion::BitsJobs => 1197,
            DefenseEvasion::Impersonation => 1656,
            DefenseEvasion::TemplateInjection => 1221,
            DefenseEvasion::AccessTokenManipulation => 1134,
            DefenseEvasion::DebuggerEvasion => 1622,
            DefenseEvasion::DomainPolicyModification => 1484,
            DefenseEvasion::XslScriptProcessing => 1220,
            DefenseEvasion::ModifyAuthenticationProcess => 1556,
            DefenseEvasion::SystemScriptProxyExecution => 1216,
            DefenseEvasion::ExploitationForDefenseEvasion => 1211,
            DefenseEvasion::TrustedDeveloperUtilitiesProxyExecution => 1127,
        }
    }
}
impl From<Exfiltration> for u32 {
    fn from(value: Exfiltration) -> Self {
        match value {
            Exfiltration::ExfiltrationOverWebService => 1567,
            Exfiltration::ScheduledTransfer => 1029,
            Exfiltration::ExfiltrationOverOtherNetworkMedium => 1011,
            Exfiltration::AutomatedExfiltration => 1020,
            Exfiltration::ExfiltrationOverC2Channel => 1041,
            Exfiltration::ExfiltrationOverAlternativeProtocol => 1048,
            Exfiltration::DataTransferSizeLimits => 1030,
            Exfiltration::TransferDataToCloudAccount => 1537,
            Exfiltration::ExfiltrationOverPhysicalMedium => 1052,
        }
    }
}
impl From<Discovery> for u32 {
    fn from(value: Discovery) -> Self {
        match value {
            Discovery::SystemOwnerUserDiscovery => 1033,
            Discovery::ContainerAndResourceDiscovery => 1613,
            Discovery::PermissionGroupsDiscovery => 1069,
            Discovery::GroupPolicyDiscovery => 1615,
            Discovery::DeviceDriverDiscovery => 1652,
            Discovery::SystemServiceDiscovery => 1007,
            Discovery::NetworkSniffing => 1040,
            Discovery::NetworkShareDiscovery => 1135,
            Discovery::PeripheralDeviceDiscovery => 1120,
            Discovery::SystemInformationDiscovery => 1082,
            Discovery::ApplicationWindowDiscovery => 1010,
            Discovery::CloudInfrastructureDiscovery => 1580,
            Discovery::BrowserInformationDiscovery => 1217,
            Discovery::SystemNetworkConfigurationDiscovery => 1016,
            Discovery::AccountDiscovery => 1087,
            Discovery::DomainTrustDiscovery => 1482,
            Discovery::FileAndDirectoryDiscovery => 1083,
            Discovery::SystemNetworkConnectionsDiscovery => 1049,
            Discovery::VirtualizationSandboxEvasion => 1497,
            Discovery::CloudStorageObjectDiscovery => 1619,
            Discovery::LogEnumeration => 1654,
            Discovery::ProcessDiscovery => 1057,
            Discovery::PasswordPolicyDiscovery => 1201,
            Discovery::QueryRegistry => 1012,
            Discovery::SystemLocationDiscovery => 1614,
            Discovery::CloudServiceDiscovery => 1526,
            Discovery::RemoteSystemDiscovery => 1018,
            Discovery::NetworkServiceDiscovery => 1046,
            Discovery::SoftwareDiscovery => 1518,
            Discovery::CloudServiceDashboard => 1538,
            Discovery::DebuggerEvasion => 1622,
            Discovery::SystemTimeDiscovery => 1124,
        }
    }
}
impl From<Collection> for u32 {
    fn from(value: Collection) -> Self {
        match value {
            Collection::ScreenCapture => 1113,
            Collection::AdversaryInTheMiddle => 1557,
            Collection::DataFromConfigurationRepository => 1602,
            Collection::AudioCapture => 1123,
            Collection::EmailCollection => 1114,
            Collection::DataFromRemovableMedia => 1025,
            Collection::AutomatedCollection => 1119,
            Collection::ClipboardData => 1115,
            Collection::DataFromCloudStorage => 1530,
            Collection::DataFromLocalSystem => 1005,
            Collection::ArchiveCollectedData => 1560,
            Collection::BrowserSessionHijacking => 1185,
            Collection::VideoCapture => 1125,
            Collection::DataStaged => 1074,
            Collection::DataFromNetworkSharedDrive => 1039,
            Collection::InputCapture => 1056,
            Collection::DataFromInformationRepositories => 1213,
        }
    }
}
impl From<ResourceDevelopment> for u32 {
    fn from(value: ResourceDevelopment) -> Self {
        match value {
            ResourceDevelopment::AcquireInfrastructure => 1583,
            ResourceDevelopment::CompromiseInfrastructure => 1584,
            ResourceDevelopment::CompromiseAccounts => 1586,
            ResourceDevelopment::StageCapabilities => 1608,
            ResourceDevelopment::EstablishAccounts => 1585,
            ResourceDevelopment::ObtainCapabilities => 1588,
            ResourceDevelopment::AcquireAccess => 1650,
            ResourceDevelopment::DevelopCapabilities => 1587,
        }
    }
}
impl From<Reconnaissance> for u32 {
    fn from(value: Reconnaissance) -> Self {
        match value {
            Reconnaissance::GatherVictimHostInformation => 1592,
            Reconnaissance::SearchVictimOwnedWebsites => 1594,
            Reconnaissance::GatherVictimIdentityInformation => 1589,
            Reconnaissance::SearchOpenTechnicalDatabases => 1596,
            Reconnaissance::ActiveScanning => 1595,
            Reconnaissance::GatherVictimOrgInformation => 1591,
            Reconnaissance::GatherVictimNetworkInformation => 1590,
            Reconnaissance::SearchOpenWebsitesDomains => 1593,
            Reconnaissance::SearchClosedSources => 1597,
            Reconnaissance::PhishingForInformation => 1598,
        }
    }
}
impl From<CommandAndControl> for u32 {
    fn from(value: CommandAndControl) -> Self {
        match value {
            CommandAndControl::ApplicationLayerProtocol => 1071,
            CommandAndControl::RemoteAccessSoftware => 1219,
            CommandAndControl::ContentInjection => 1659,
            CommandAndControl::TrafficSignaling => 1205,
            CommandAndControl::ProtocolTunneling => 1572,
            CommandAndControl::CommunicationThroughRemovableMedia => 1092,
            CommandAndControl::Proxy => 1090,
            CommandAndControl::DynamicResolution => 1568,
            CommandAndControl::WebService => 1102,
            CommandAndControl::MultiStageChannels => 1104,
            CommandAndControl::DataObfuscation => 1001,
            CommandAndControl::NonStandardPort => 1571,
            CommandAndControl::EncryptedChannel => 1573,
            CommandAndControl::NonApplicationLayerProtocol => 1095,
            CommandAndControl::DataEncoding => 1132,
            CommandAndControl::IngressToolTransfer => 1105,
            CommandAndControl::FallbackChannels => 1008,
        }
    }
}
impl From<InitialAccess> for u32 {
    fn from(value: InitialAccess) -> Self {
        match value {
            InitialAccess::ExternalRemoteServices => 1133,
            InitialAccess::ReplicationThroughRemovableMedia => 1091,
            InitialAccess::SupplyChainCompromise => 1195,
            InitialAccess::ExploitPublicFacingApplication => 1190,
            InitialAccess::ContentInjection => 1659,
            InitialAccess::TrustedRelationship => 1199,
            InitialAccess::Phishing => 1566,
            InitialAccess::ValidAccounts => 1078,
            InitialAccess::HardwareAdditions => 1200,
            InitialAccess::DriveByCompromise => 1189,
        }
    }
}
