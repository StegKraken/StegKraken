FROM gcc:4.9
LABEL description="RustKraken - The World's Fastest Stegcracker." 
RUN git clone https://github.com/StegKraken/StegKraken
WORKDIR /StegKraken/src
RUN ls
RUN g++ -o stegkraken src/main.cpp
CMD ["./stegkraken"]