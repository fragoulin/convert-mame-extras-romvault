# convert-mame-extras-romvault

## Description
Convert MAME Extras datafiles to a datafile compatible with RomVault.

## Usage
`convert-mame-extras-romvault <inputfile>`

`convert-mame-extras-romvault <inputfile> <outputfile>`

`inputfile` is a Zip file containing MAME Extras datafiles (e.g. 'MAME 0.264 EXTRAs.zip').

`outputfile` will be generated (e.g. 'Extras.dat'), ready to be used with RomVault.

if `outputfile` is not specified, the generated output file will match the `inputfile` name (e.g. if `inputfile` name is 'MAME 0.264 EXTRAs.zip', `outputfile` name will be 'MAME 0.264 EXTRAs.dat')

## Resources
https://pleasuredome.miraheze.org/wiki/MAME_EXTRAs
