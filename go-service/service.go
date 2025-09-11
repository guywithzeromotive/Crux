package main

/*
#cgo LDFLAGS: -L./rust_lib -lrust_core
#include "rust_lib/crux_core.h"
*/
import "C"

import (
	"bufio"
	"fmt"
	"net"
	"strings"
)

func handle(conn net.Conn) {
	defer conn.Close()
	reader := bufio.NewReader(conn)

	for {
		cmd, err := reader.ReadString('\n')
		if err != nil {
			return
		}
		cmd = strings.TrimSpace(cmd)
		fmt.Println("Got command:", cmd)

		if cmd == "list_processes" {
			s := C.collect_unique_processes_list()
			defer C.free_cstring(s)
			result := C.GoString(s)
			fmt.Println("Sending result:", len(result), "bytes")
			conn.Write([]byte(result))
			return
		}
	}
}

func main() {
	fmt.Println("Go service listening on port 9000...")
	ln, err := net.Listen("tcp", ":9000")
	if err != nil {
		panic(err)
	}
	for {
		conn, err := ln.Accept()
		if err != nil {
			continue
		}
		go handle(conn)
	}
}
