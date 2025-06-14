# Windows Containers on Windows VM from a Linux Host Guide
## Use Case: Working with Windows Containers from a Windows VM in Libvirtd/KVM
 
### Prerequisites
- A functional Linux host with libvirtd and nested virtualization enabled.
- Approximately 200GB+ of free disk space.
 
## Download and Convert Windows Evaluation Image to Raw Format
 
```bash
wget https://download.microsoft.com/download/e/e/c/eec4775f-f2e8-4476-98b2-ca51502a6429/WinDev2407Eval.VMWare.zip
unzip WinDev2407Eval.VMWare.zip
qemu-img convert -f vmdk WinDev2407Eval-disk1.vmdk -O raw output.raw
```
 
## Configure Virtual Machine in Virt-Manager
 
### Steps:
1. Open **virt-manager**.
2. Navigate to **New VM** > **Manual Install**.
3. Set the OS type as **Windows 11**.
4. Allocate appropriate RAM and CPU resources.
5. Select or Create custom Storage: Manage > select **output.raw** file that was converted in previous step.
6. Create the VM and start it
 
## Remove VMware User Process from Windows VM Registry

1. Once in the VM we can remove VMware Tools that was pre-packaged in the image and install the KVM video driver
```powershell
Remove-ItemProperty -Path "HKLM:\Software\Microsoft\Windows\CurrentVersion\Run" -Name "VMware User Process"
```
 
2. Download the guest tools ISO:
```bash
cd ~/Downloads
$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri https://getutm.app/downloads/utm-guest-tools-latest.iso -OutFile .\utm-guest-tools-latest.iso
```
3. Mount and install the driver:
```powershell
$isoImg = "C:\Users\User\Downloads\utm-guest-tools-latest.iso"
$driveLetter = "U:"
$diskImg = Mount-DiskImage -ImagePath $isoImg -NoDriveLetter
mountvol $driveLetter $($diskImg | Get-Volume).UniqueId
Start-Process "U:\utm-guest-tools-0.*.exe"
```
4. Complete the installation and restart the VM.
 
## Install Chocolatey Package Manager
 
```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```
 
## Install Git 
 
```powershell
choco install git.install --params "/GitAndUnixToolsOnPath" -y
```
 
## Enable WSL2 and Hyperv Features
 
```powershell
choco install wsl2 -y
wsl --update
Enable-WindowsOptionalFeature -Online -FeatureName containers –All
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All
```
1. A reboot may be needed here
 
## Install Docker Desktop for Windows Containers
 
1. Download the installer:
```bash
cd ~/Downloads
$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri https://desktop.docker.com/win/stable/Docker%20Desktop%20Installer.exe -OutFile .\DockerDesktopInstaller.exe
```
2. Install and configure Docker Desktop:
```bash
Start-Process .\DockerDesktopInstaller.exe -Verb RunAs
```
3. Select the options to run Windows Containers and to use WSL2
 
## Configure Docker Desktop
 
1. Start Menu > run Docker Desktop from the **Run as Administrator** context option, accept EULA and skip optionals
2. Navigate to settings (gear icon) > Docker Engine.
3. Set experimental mode to **true**, apply changes, and restart.
4. Disable resource saver under resources settings.
5. Right click the Docker Desktop TaskTray icon and change the service to run Windows Containers
 
## Run Windows Container with Chocolatey
 
```bash
docker run -it -p 11000:11000 -p 11001:11001 -p 11002:11002 -p 11003:11003 -p 11004:11004 amitie10g/chocolatey:ltsc2022 powershell
```
 
## Install Rust and Build Tools inside the Windows Container
 
### Steps:
1. Install Rustup:
```powershell
cd ~/Downloads
# Download and install Rust
function reload {
   foreach($level in "Machine","User") {
      [Environment]::GetEnvironmentVariables($level).GetEnumerator() | % {
         # For Path variables, append the new values, if they're not already in there
         if($_.Name -match 'Path$') {
            $_.Value = ($((Get-Content "Env:$($_.Name)") + ";$($_.Value)" ) -split ';' | Select -unique) -join ';'
         }
         $_
      } | Set-Content -Path { "Env:$($_.Name)" }
   }
}
Write-Host "Installing Rust..." -ForegroundColor Cyan
$exePath = "$env:TEMP\rustup-init.exe"
Write-Host "Downloading..."
(New-Object Net.WebClient).DownloadFile('https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe', $exePath)
Write-Host "Installing..."
cmd /c start /wait $exePath -y
Remove-Item $exePath
$addPath = "$env:USERPROFILE\.cargo\bin"
[Environment]::SetEnvironmentVariable ($addPath, $env:Path, [System.EnvironmentVariableTarget]::Machine)
reload
```
 
## Verify Installation
 
```powershell
cargo --version
rustup --version
rustc --version
```

## Install additional packages
```powershell
choco install git.install --params "/GitAndUnixToolsOnPath" -y
choco install visualstudio2022buildtools -y
choco install visualstudio2022-workload-vctools -y
reload
```
 
## Build Rocket Example Application
 
### Steps:
1. Create a project directory and clone the repository:
```bash
mkdir ~/Rust_Rocket_Example
cd ~/Rust_Rocket_Example
git clone https://github.com/wpcodevo/simple-api-rocket.git
cd .\simple-api-rocket\
```
2. Configure environment variables:
```powershell
$env:ROCKET_PORT = "11001"
$env:ROCKET_ADDRESS = "0.0.0.0"
$ipv4 = (Get-NetIPAddress -AddressFamily IPv4).IPAddress[0]
$env:LOCALHOST_ADDRESS = $ipv4
```
3. Modify the configuration file:
```bash
sed -i 's/8000/11001/g' .\Todo.postman_collection.json
echo "[server]
port = 11001" | Out-File -FilePath Rocket.toml -Encoding UTF8
```
4. Build and run the application:
```bash
cargo r -r
```
 
## Commit Changes to Container Image
 
### Steps:
1. Stop the running container:
```bash
docker stop {container_name}
```
2. Create a new image from the stopped container:
```bash
docker commit {container_name} my_project_image:latest
```
 
## Start the dev environment container you commited
 
```bash
docker run -it -p 11000:11000 -p 11001:11001 -p 11002:11002 -p 11003:11003 -p 11004:11004 my_project_image:latest powershell
```
 
## Access and Start the Application
 
```bash
cd "C:\Users\ContainerAdministrator\Rust_Rocket_Example\simple-api-rocket"
$env:ROCKET_PORT = "11001"
$env:ROCKET_ADDRESS = "0.0.0.0"
$env:LOCALHOST_ADDRESS = $ipConfig
$ipv4 = (Get-NetIPAddress -AddressFamily IPv4).IPAddress[0]
$env:LOCALHOST_ADDRESS = $ipv4
cargo r -r
```
 
---
