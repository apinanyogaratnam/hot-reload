package main

import (
	"log"
	"os"
	"os/exec"
	"os/signal"
	"path/filepath"
	"strings"
	"sync"
	"syscall"
	"time"
)

var (
	// Map of file paths to their modification time
	files = make(map[string]time.Time)
	// Mutex to prevent concurrent map write errors
	mux sync.Mutex
)

func main() {
	// Start the server
	cmd := startServer()

	// Set up a channel to listen for the interrupt signal (Ctrl+C)
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	go func() {
		for sig := range c {
			log.Println("Received signal:", sig)
			if err := cmd.Process.Kill(); err != nil {
				log.Println("Failed to kill process: ", err)
			} else {
				log.Println("Server stopped.")
			}
			os.Exit(0)
		}
	}()

	for {
		err := filepath.Walk(".", func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return err
			}
			// Only consider .go files
			if strings.HasSuffix(path, ".go") {
				mux.Lock()
				modTime, ok := files[path]
				if !ok || modTime != info.ModTime() {
					log.Println("File", path, "has been modified. Reloading...")
					files[path] = info.ModTime()
					// Restart the server
					cmd = startServer()
				}
				mux.Unlock()
			}
			return nil
		})
		if err != nil {
			log.Println("Error walking the path .: ", err)
			continue
		}

		// Sleep for a second before the next file check
		time.Sleep(1 * time.Second)
	}
}

func startServer() *exec.Cmd {
	// If a previous server process is still running, kill it
	if cmd := recover(); cmd != nil {
		cmd.(*exec.Cmd).Process.Kill()
	}
	// Start a new server process
	cmd := exec.Command("make", "start")
	cmd.SysProcAttr = &syscall.SysProcAttr{Setpgid: true}
	err := cmd.Start()
	if err != nil {
		log.Fatal("Failed to start the server: ", err)
	}
	log.Println("Server started with PID", cmd.Process.Pid)
	return cmd
}

