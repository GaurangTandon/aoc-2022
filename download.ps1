param($day);

Write-Output $day

curl https://adventofcode.com/2022/day/$day/input --cookie "session=$(Get-Content session.cookie)" -o inputs\$day.txt