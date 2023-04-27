#ifndef FILES_GITIGNORE_H
#define FILES_GITIGNORE_H

#include "fstream.h"

int gitignore(){
    fout.open(".gitignore");
    fout << "/build\n"
    << "*.DS_Store\n"
    << "/.vscode\n"
    << "/.idea";
    fout.close();
    return 0;
}

#endif