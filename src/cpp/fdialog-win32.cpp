#include <cstdio>
#include <cwchar>
#include <cstring>
#include <locale>
#include <codecvt>

#include <windows.h>
#include <commdlg.h>

#pragma comment(lib, "comdlg32.lib")

#ifndef _FDIALOG_WIN32
#define _FDIALOG_WIN32

extern "C"
char* open_dialog()
{
	const size_t MAX_STRING_LENGTH = 300;

    OPENFILENAME ofn;
    ZeroMemory(&ofn, sizeof(ofn));

    char *szFile = (char*)malloc(MAX_STRING_LENGTH * sizeof(char));
    szFile[0] = '\0';

    ofn.lStructSize = sizeof(ofn);
	ofn.hwndOwner = NULL;
	ofn.lpstrFile = (LPSTR) szFile;
	ofn.nMaxFile = 300;
	ofn.lpstrFilter = "All\0*.*\0Text\0*.txt\0";
	ofn.nFilterIndex = 1;
	ofn.lpstrFileTitle = NULL;
	ofn.nMaxFileTitle = 0;
	ofn.lpstrInitialDir = NULL;
	ofn.Flags = OFN_PATHMUSTEXIST | OFN_FILEMUSTEXIST;

	if(!GetOpenFileNameA((LPOPENFILENAMEA) &ofn))
	{
		printf("Can't open Windows open dialog\n");
	}

    return szFile;
}

#endif
