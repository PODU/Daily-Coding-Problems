// Day 1759: "What happens when you type a URL and press Enter."
// Approach: return the ordered list of stages and print them. O(s) time/space over s stages.
package main

import "fmt"

func urlNavigationStages() []string {
	return []string{
		"URL parsing (scheme, host, port, path, query)",
		"DNS resolution (hostname -> IP address)",
		"TCP handshake (SYN, SYN-ACK, ACK)",
		"TLS handshake (certificate exchange, key negotiation for HTTPS)",
		"HTTP request sent (method, headers, body)",
		"Server processing (routing, application logic)",
		"HTTP response received (status, headers, body)",
		"Browser parsing HTML/CSS/JS",
		"DOM/CSSOM construction and render tree creation",
		"Layout (computing geometry of render tree)",
		"Paint and composite (pixels drawn to screen)",
	}
}

func main() {
	for i, stage := range urlNavigationStages() {
		fmt.Printf("%d. %s\n", i+1, stage)
	}
}
