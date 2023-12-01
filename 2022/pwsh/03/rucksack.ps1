function Get-Score {
    PARAM(
       [Parameter(ValueFromPipeline)] [char] $char
    )

    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".IndexOf($char)+1
}

$score = (Get-Content ./contents.txt).ForEach({
    if ($_ -eq '') {
        continue
    }

    $i = $_.Length/2
    $a = $_.Substring(0,$i) -split '' | ?{$_ -ne ''}
    $b = $_.Substring($i) -split '' | ?{$_ -ne ''}

    $union = Compare-Object $a $b -IncludeEqual -ExcludeDifferent -PassThru -CaseSensitive | `
        Select-Object -Unique
    # Write-Host $union

    $union | %{
        Get-Score $_
    } 
    # Write-Host $val
})| %{$sum=0}{$sum+=$_}{$sum}
Write-Host $score



function Chunk-By {
    PARAM(
        [Parameter(ValueFromPipeline)] [object[]] $Input,
        [int] $ChunkSize = 3
    )

    $buffer = @()
    for($i=0; $i -lt $Input.Count; $i+=$ChunkSize) {
        for($j = 0; $j -lt $ChunkSize; $j++) {
            if ($i + $j -ge $Input.Count) {
                return $buffer
            }

            $buffer += $Input[$i + $j]
        }
        Write-Output -NoEnumerate $buffer
        $buffer = @()
    }
}

(Get-Content ./contents.txt) | `
    Chunk-By -ChunkSize 3 | `
    % {
        $a = $_[0] -split '' | ?{$_ -ne ''}
        $b = $_[1] -split '' | ?{$_ -ne ''}
        $c = $_[2] -split '' | ?{$_ -ne ''}

        $xs = Compare-Object $a $b -IncludeEqual -ExcludeDifferent -PassThru -CaseSensitive | Select-Object -Unique
        $xs = Compare-Object $xs $c -IncludeEqual -ExcludeDifferent -PassThru -CaseSensitive | Select-Object -Unique
        $xs | Select-Object -First 1 | Get-Score
    } | %{$sum=0}{$sum+=$_}{$sum}
    