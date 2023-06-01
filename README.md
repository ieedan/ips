# ips
A simple CLI built in rust for setting your static IP Address quickly without using a mouse.

<a id="using-ips"></a>
## Using ips

### Change your IP Address

Open a new command prompt or powershell and type in `ips` followed by the ip you want to set.

```bash
ips 172.16.160.244
```

You can set your subnet here as well by adding it as a second argument. If you choose not to add a subnet the default subnet is 255.255.255.0.

```bash
ips 172.16.160.244 255.255.255.0
```

# Setup
**1.** Fork this repo and copy the contents of /target/release
<br/>
**2.** Go to C:\Program Files and make a folder called `ips`
<br/>
**3.** Paste the contents of /target/release under C:\Program Files\ips and right click on `ips`
<br/>

<img src="https://github.com/ieedan/ips/assets/117548273/6e228ff3-5bab-4af6-b332-a2dd459b20d4"/>

**4.** Open `Properties` from the context menu then find `Compatibility` and under `Settings` check `Run this program as an administrator`
<br/>

<img src="https://github.com/ieedan/ips/assets/117548273/2e56b985-debc-4850-b3f2-0e5d074bc612"/>

**5.** Setup the enviorment variable by going to win + R then type 
```bash
sysdm.cpl
```
<br/>

<img src="https://github.com/ieedan/ips/assets/117548273/1f6ee7c1-32d0-41d1-9d51-7f715f915d93"/>

**6.** Press `OK` and then go to `Advanced`
<br/>

<img src="https://github.com/ieedan/ips/assets/117548273/5b69ce62-1389-425c-80b9-4a0ff74494d0"/>

**7.** Click `Enviorment Variables...` then under `System Variables` find `Path` and click `Edit`
<br/>

<img src="https://github.com/ieedan/ips/assets/117548273/4c4f8277-6e39-44f4-b12d-216e59c104f3"/>

**8.** Click `New` to add your enviorment variable
<br/>

<img src="https://github.com/ieedan/ips/assets/117548273/4f3e394d-7a89-4a44-ae31-5d8f3bd98409"/>

**9.** If you were following along correctly you can now enter the path 
```bash
C:\Program Files\ips\
```
<br/>

<img src="https://github.com/ieedan/ips/assets/117548273/35ab7606-ab53-4942-bdf3-24eac20d4a6a"/>

[Start Using ips](#using-ips)
