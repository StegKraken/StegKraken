package main

import (
	"fmt"
	"io"
	"os"
	"os/exec"
	"runtime"

	"github.com/akamensky/argparse"
)

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

	// Create new parser object
	parser := argparse.NewParser("StegKraken", "Fast Steg Cracking")
	// Create string flag
	s := parser.String("i", "image", &argparse.Options{Required: true, Help: "The image to crack"})
	// Parse input
	err := parser.Parse(os.Args)
	if err != nil {
		// In case of error print error and print usage
		// This can also be done by passing -h or --help flags
		fmt.Print(parser.Usage(err))
	}
	// Finally print the collected string
	fmt.Println(*s)
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
