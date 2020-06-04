!define APPNAME "league_of_steel"
!define COMPANYNAME "Krzysztof Grzywocz"
!define HELPURL "https://github.com/kgrzywocz/league_of_steel"
!define UPDATEURL "https://github.com/kgrzywocz/league_of_steel/releases"
!define ABOUTURL "https://github.com/kgrzywocz/league_of_steel"
!include versions.nsi
 
RequestExecutionLevel admin
InstallDir "$PROGRAMFILES64\${APPNAME}"
LicenseData "..\LICENSE"
Name "${APPNAME}"
Icon "logo.ico"
OutFile "..\target\league_of_steel_setup.exe"
 
!include LogicLib.nsh
page license
page directory
Page instfiles
 
!macro VerifyUserIsAdmin
UserInfo::GetAccountType
pop $0
${If} $0 != "admin"
        messageBox mb_iconstop "Administrator rights required!"
        setErrorLevel 740 ;ERROR_ELEVATION_REQUIRED
        quit
${EndIf}
!macroend
 
function .onInit
	Execwait "taskkill /F /IM league_of_steel.exe"
    Delete "$SMSTARTUP\league_of_steel.exe"

	SetRegView 64
	setShellVarContext all
	!insertmacro VerifyUserIsAdmin
functionEnd
 
section "install"
    #MSVC Redistributable
    SetOutPath "$TEMP\league_of_steel_setup"
    File VC_redist.x64.exe
    ExecWait '"$TEMP\league_of_steel_setup\VC_redist.x64.exe" /passive /norestart'
    RMDir /r "$TEMP\league_of_steel_setup"

    #Install and run
    Execwait "taskkill /F /IM league_of_steel.exe"
    SetOutPath "$INSTDIR"
    File "..\target\x86_64-pc-windows-msvc\release\league_of_steel.exe"
    Exec "$INSTDIR\${APPNAME}.exe"

    #Autostart
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Run" "${APPNAME}" "$INSTDIR\${APPNAME}.exe"

	# Uninstaller - See function un.onInit and section "uninstall" for configuration
	writeUninstaller "$INSTDIR\uninstall.exe"

	# Registry information for add/remove programs
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "DisplayName" "${APPNAME}"
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "UninstallString" "$INSTDIR\uninstall.exe"
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "QuietUninstallString" "$INSTDIR\uninstall.exe /S"
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "InstallLocation" "$INSTDIR"
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "DisplayIcon" "$INSTDIR\logo.ico"
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "Publisher" "${COMPANYNAME}"
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "HelpLink" "${HELPURL}"
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "URLUpdateInfo" "${UPDATEURL}"
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "URLInfoAbout" "${ABOUTURL}"
	WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "DisplayVersion" "${VERSIONMAJOR}.${VERSIONMINOR}.${VERSIONBUILD}"
	WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "VersionMajor" ${VERSIONMAJOR}
	WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "VersionMinor" ${VERSIONMINOR}
	WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "NoModify" 1
	WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}" "NoRepair" 1
sectionEnd
 
# Uninstaller 
function un.onInit
	SetShellVarContext all

	MessageBox MB_OKCANCEL "Permanantly remove ${APPNAME}?" IDOK next
		Abort
	next:
	!insertmacro VerifyUserIsAdmin
functionEnd
 
section "uninstall"
	#Kill if running
    Execwait "taskkill /F /IM league_of_steel.exe"

    #Remove autostart
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Run\${APPNAME}"

    #TODO deregister games

	# Remove files
	delete "$INSTDIR\${APPNAME}.exe"
	delete "$INSTDIR\logo.ico"
 
	delete "$INSTDIR\uninstall.exe"
	rmDir /r "$INSTDIR"
 
	# Remove uninstaller information from the registry
	DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${APPNAME}"
sectionEnd