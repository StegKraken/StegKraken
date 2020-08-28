package main

import (
	"flag"
	"fmt"
	"io"
	"os"
	"os/exec"
	"runtime"
)

func Int(name string, value int, usage string) *int {
	var count int
	flag.IntVar(&count, "count", 5, "the count of items")
}

func execute_steghide(password string) {
	// let's try the pwd command herer
	out, err := exec.Command("pwd").Output()
	if err != nil {
		fmt.Printf("%s", err)
	}
	fmt.Println("Command Successfully Executed")
	output := string(out[:])
	fmt.Println(output)
}

func main() {
	if runtime.GOOS == "windows" {
		fmt.Println("Windows does not support Steghide.")
		return
	}

	// tries execution once with no password
	execute_steghide("")

}

func read_wordlist_and_execute() {
	// Reads file in bytes of 100
	const BufferSize = 100
	file, err := os.Open("filetoread.txt")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer file.Close()

	buffer := make([]byte, BufferSize)

	for {
		bytesread, err := file.Read(buffer)

		if err != nil {
			if err != io.EOF {
				fmt.Println(err)
			}

			break
		}

		fmt.Println("bytes read: ", bytesread)
		fmt.Println("bytestream to string: ", string(buffer[:bytesread]))
	}
}
