# https://adventofcode.com/2022/day/1

$block = {
    if ($_ -eq '') {
        Write-Output $sum
        $sum = 0
    } else {
        $sum += $_
    }
}

# Part 1
$top_elf = get-content .\calories.txt | `
    % {$sum=0} $block {Write-Output $sum} | `
    Sort-Object -Descending | `
    Select-Object -First 1
Write-Host $top_elf

# Part 2
$top_3_elves = get-content .\calories.txt | % {$sum=0} $block {Write-Output $sum} | `
    Sort-Object -Descending | `
    Select-Object -First 3 | `
    %{$sum2=0} {$sum2 += $_} {$sum2}
Write-Host $top_3_elves