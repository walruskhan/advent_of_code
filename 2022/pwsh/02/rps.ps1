# https://adventofcode.com/2022/day/2

function Convert-SymbolToPoints {
    PARAM (
        [Parameter(Mandatory, ValueFromPipeline)] $Hand
    )

    return @{
        'A' = 1;
        'X' = 1;

        'B' = 2;
        'Y' = 2;

        'C' = 3;
        'Z' = 3;
        
    }[$Hand]
}

function Get-Score {
    PARAM(
        [Parameter(Mandatory, ValueFromPipeline)] [String] $Line
    )

    $them,$us = $Line.Split(' ')

    @{
        "A X" = 3; # R R
        "B Y" = 3; # P P 
        "C Z" = 3; # S S

        "A Z" = 0; # R S
        "B X" = 0; # P R
        "C Y" = 0; # S P

        "A Y" = 6; # R P
        "B Z" = 6; # P S
        "C X" = 6; # S R
    }[$Line] + (Convert-SymbolToPoints $us)
}


function Get-Hand {
    PARAM (
        [Parameter(Mandatory, ValueFromPipeline)] [String] $Line
    )

    $them,$required = $Line.Split(' ')

    $me = @{
        # lose
        "A X" = "Z"; # R
        "B X" = "X"; # P
        "C X" = "Y"; # S

        # draw
        "A Y" = "X"; # R
        "B Y" = "Y"; # P
        "C Y" = "Z"; # S

        # win
        "A Z" = "Y"; # R
        "B Z" = "Z"; # P
        "C Z" = "X"; # S

    }[$Line]

    "$them $me"
}

# Part 1
Get-Content strategy.txt | %{Get-Score $_} | %{$sum=0}{$sum+=$_}{$sum}

# Part 2
Get-Content .\strategy.txt | %{
    Get-Score (Get-Hand $_)
} | %{$sum=0}{$sum+=$_}{$sum}