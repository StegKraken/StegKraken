#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <array>
#include <algorithm>
#include <execution>
#include <functional>





int main() {
    std::ifstream file("/usr/share/wordlists/rockyou.txt");
    if (file.is_open()) {
        std::string line;
        std::array<std::string, 1000> buffer; 
        size_t size = 0;
        // std::vector< std::string > buffer;

        // reads file line by line
        while (std::getline(file, line)) {
            buffer[size++] = line.c_str();
            // buffer size is 1k so we call steghide on this
            if (size == 1000){
                run_batch(buffer);
                size = 0;
            }
        }
        run_batch(buffer);
        file.close();
}
}

void run_batch(std::array<std::string, 1000> batch){
        std::find_if(std::execution::par_unseq, batch.begin(), batch.end(), [](auto& i) { ... });
}

