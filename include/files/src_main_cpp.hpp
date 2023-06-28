#ifndef FILES_SRC_MAIN_CPP_HPP
#define FILES_SRC_MAIN_CPP_HPP

#include "fstream.h"

int src_main_cpp() {
    fout.open("./src/main.cpp");
    fout << "#include <iostream>\n\n"
    << "int main(int argc, char* argv[]) {\n"
    << "    std::cout << \"Hello Young!\";\n"
    << "    return 0;\n"
    << "}\n";
    fout.close();
    return 0;
}

#endif