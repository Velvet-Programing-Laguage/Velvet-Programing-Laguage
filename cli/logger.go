package main

import (
    "log"
    "os"
)

func setupLogger(debug bool) *log.Logger {
    logger := log.New(os.Stdout, "Velvet: ", log.LstdFlags)
    if debug {
        logger.SetFlags(log.LstdFlags | log.Lshortfile)
    }
    return logger
}
