#ifndef FILES_GITIGNORE_HPP
#define FILES_GITIGNORE_HPP

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