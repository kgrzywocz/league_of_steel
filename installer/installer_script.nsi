OutFile "..\target\league_of_steel_setup.exe"
 
RequestExecutionLevel admin
InstallDir $SMSTARTUP

Section
    SetOutPath "$TEMP\league_of_steel_setup"
    File VC_redist.x64.exe
    ExecWait '"$TEMP\league_of_steel_setup\VC_redist.x64.exe" /passive /norestart'

    Execwait "taskkill /F /IM league_of_steel.exe"
    SetOutPath $INSTDIR
    File ..\target\x86_64-pc-windows-msvc\release\league_of_steel.exe
    Exec $INSTDIR\league_of_steel.exe

    RMDir /r "$TEMP\league_of_steel_setup"
SectionEnd
