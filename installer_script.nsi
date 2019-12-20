OutFile "target\league_of_steel_setup.exe"
 
RequestExecutionLevel user
InstallDir $SMSTARTUP

Section
    Execwait "taskkill /F /IM league_of_steel.exe"
    SetOutPath $INSTDIR
    File target\x86_64-pc-windows-msvc\release\league_of_steel.exe
    Exec $INSTDIR\league_of_steel.exe
SectionEnd