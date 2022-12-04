Base command to download inputs:

```Powershell
curl https://adventofcode.com/2022/day/1/input --cookie "session=$(Get-Content ..\session.cookie)" -o inputs\a.txt
```